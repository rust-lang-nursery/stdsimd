//! Caches run-time feature detection so that it only needs to be computed
//! once.

#![allow(dead_code)] // not used on all platforms

use crate::sync::atomic::Ordering;

use crate::sync::atomic::AtomicUsize;

/// Sets the `bit` of `x`.
#[inline]
const fn set_bit(x: u64, bit: u32) -> u64 {
    x | 1 << bit
}

/// Tests the `bit` of `x`.
#[inline]
const fn test_bit(x: u64, bit: u32) -> bool {
    x & (1 << bit) != 0
}

/// Unset the `bit of `x`.
#[inline]
const fn unset_bit(x: u64, bit: u32) -> u64 {
    x & !(1 << bit)
}

/// Maximum number of features that can be cached.
const CACHE_CAPACITY: u32 = 62;

/// This type is used to initialize the cache
#[derive(Copy, Clone)]
pub(crate) struct Initializer(u64);

#[allow(clippy::use_self)]
impl Default for Initializer {
    fn default() -> Self {
        Initializer(0)
    }
}

// NOTE: the `debug_assert!` would catch that we do not add more Features than
// the one fitting our cache.
impl Initializer {
    /// Tests the `bit` of the cache.
    #[inline]
    pub(crate) fn test(self, bit: u32) -> bool {
        debug_assert!(
            bit < CACHE_CAPACITY,
            "too many features, time to increase the cache size!"
        );
        test_bit(self.0, bit)
    }

    /// Sets the `bit` of the cache.
    #[inline]
    pub(crate) fn set(&mut self, bit: u32) {
        debug_assert!(
            bit < CACHE_CAPACITY,
            "too many features, time to increase the cache size!"
        );
        let v = self.0;
        self.0 = set_bit(v, bit);
    }

    /// Unsets the `bit` of the cache.
    #[inline]
    pub(crate) fn unset(&mut self, bit: u32) {
        debug_assert!(
            bit < CACHE_CAPACITY,
            "too many features, time to increase the cache size!"
        );
        let v = self.0;
        self.0 = unset_bit(v, bit);
    }
}

/// This global variable is a cache of the features supported by the CPU.
// Note: on x64, we only use the first slot
static CACHE: [Cache; 2] = [Cache::uninitialized(), Cache::uninitialized()];

/// Feature cache with capacity for `usize::MAX - 1` features.
///
/// Note: the last feature bit is used to represent an
/// uninitialized cache.
///
/// Note: we use `Relaxed` atomic operations, because we are only interested in
/// the effects of operations on a single memory location. That is, we only need
/// "modification order", and not the full-blown "happens before".
struct Cache(AtomicUsize);

impl Cache {
    const CAPACITY: u32 = (core::mem::size_of::<usize>() * 8 - 1) as u32;
    const MASK: usize = (1 << Cache::CAPACITY) - 1;

    /// Creates an uninitialized cache.
    #[allow(clippy::declare_interior_mutable_const)]
    const fn uninitialized() -> Self {
        Cache(AtomicUsize::new(usize::MAX))
    }

    /// Is the `bit` in the cache set? Returns `None` if the cache has not been initialized.
    #[inline]
    pub(crate) fn test(&self, bit: u32) -> Option<bool> {
        let cached = self.0.load(Ordering::Relaxed);
        if cached == usize::MAX {
            None
        } else {
            Some(test_bit(cached as u64, bit))
        }
    }

    /// Initializes the cache.
    #[inline]
    fn initialize(&self, value: usize) -> usize {
        // Use `SeqCst` store to ensure a thread which sees one entry in `CACHE`
        // initialized will definitely see the other. While it likely never can
        // happen, this avoids something like:
        //
        // ```
        // is_foo_feature_detected!("bar") && is_foo_feature_detected!("baz")
        // ```
        //
        // needing to initialize the cache twice.
        self.0.store(value, Ordering::SeqCst);
        value
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std_detect_env_override")] {
        #[inline]
        fn initialize(mut value: Initializer) {
            if let Ok(disable) = crate::env::var("RUST_STD_DETECT_UNSTABLE") {
                for v in disable.split(" ") {
                    let _ = super::Feature::from_str(v).map(|v| value.unset(v as u32));
                }
            }
            do_initialize(value);
        }
    } else {
        #[inline]
        fn initialize(value: Initializer) {
            do_initialize(value);
        }
    }
}

#[inline]
fn do_initialize(value: Initializer) {
    CACHE[0].initialize((value.0) as usize & Cache::MASK);
    CACHE[1].initialize((value.0 >> Cache::CAPACITY) as usize & Cache::MASK);
}

// We only have to detect features once, and it's fairly costly, so hint to LLVM
// that it should assume that cache hits are more common than misses (which is
// the point of caching). It's possibly unfortunate that this function needs to
// reach across modules like this to call `os::detect_features`, but it produces
// the best code out of several attempted variants.
//
// The `Initializer` that the cache was initialized with is returned, so that
// the caller can call `test()` on it without having to load the value from the
// cache again.
#[cold]
fn detect_and_initialize() -> Initializer {
    let result = super::os::detect_features();
    initialize(result);
    result
}

/// Tests the `bit` of the storage. If the storage has not been initialized,
/// initializes it with the result of `os::detect_features()`.
///
/// On its first invocation, it detects the CPU features and caches them in the
/// `CACHE` global variable as an `AtomicU64`.
///
/// It uses the `Feature` variant to index into this variable as a bitset. If
/// the bit is set, the feature is enabled, and otherwise it is disabled.
///
/// If the feature `std_detect_env_override` is enabled looks for the env
/// variable `RUST_STD_DETECT_UNSTABLE` and uses its its content to disable
/// Features that would had been otherwise detected.
#[inline]
pub(crate) fn test(bit: u32) -> bool {
    let (relative_bit, idx) = if bit < Cache::CAPACITY {
        (bit, 0)
    } else {
        (bit - Cache::CAPACITY, 1)
    };
    CACHE[idx]
        .test(relative_bit)
        .unwrap_or_else(|| detect_and_initialize().test(bit))
}
