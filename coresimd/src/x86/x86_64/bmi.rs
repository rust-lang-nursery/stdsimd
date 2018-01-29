#[cfg(test)]
use stdsimd_test::assert_instr;

/// Extracts bits in range [`start`, `start` + `length`) from `a` into
/// the least significant bits of the result.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(bextr))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bextr_u64(a: u64, start: u32, len: u32) -> u64 {
    _bextr2_u64(a, ((start & 0xff) | ((len & 0xff) << 8)) as u64)
}

/// Extracts bits of `a` specified by `control` into
/// the least significant bits of the result.
///
/// Bits [7,0] of `control` specify the index to the first bit in the range to
/// be extracted, and bits [15,8] specify the length of the range.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(bextr))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bextr2_u64(a: u64, control: u64) -> u64 {
    x86_bmi_bextr_64(a, control)
}

/// Bitwise logical `AND` of inverted `a` with `b`.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(andn))]
pub unsafe fn _andn_u64(a: u64, b: u64) -> u64 {
    !a & b
}

/// Extract lowest set isolated bit.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(blsi))]
#[cfg(not(target_arch = "x86"))] // generates lots of instructions
pub unsafe fn _blsi_u64(x: u64) -> u64 {
    x & x.wrapping_neg()
}

/// Get mask up to lowest set bit.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(blsmsk))]
#[cfg(not(target_arch = "x86"))] // generates lots of instructions
pub unsafe fn _blsmsk_u64(x: u64) -> u64 {
    x ^ (x.wrapping_sub(1_u64))
}

/// Resets the lowest set bit of `x`.
///
/// If `x` is sets CF.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(blsr))]
#[cfg(not(target_arch = "x86"))] // generates lots of instructions
pub unsafe fn _blsr_u64(x: u64) -> u64 {
    x & (x.wrapping_sub(1))
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _tzcnt_u64(x: u64) -> u64 {
    x.trailing_zeros() as u64
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _mm_tzcnt_64(x: u64) -> i64 {
    x.trailing_zeros() as i64
}

extern "C" {
    #[link_name = "llvm.x86.bmi.bextr.64"]
    fn x86_bmi_bextr_64(x: u64, y: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use x86::*;

    #[simd_test = "bmi"]
    unsafe fn test_bextr_u64() {
        let r = _bextr_u64(0b0101_0000u64, 4, 4);
        assert_eq!(r, 0b0000_0101u64);
    }

    #[simd_test = "bmi"]
    unsafe fn test_andn_u64() {
        assert_eq!(_andn_u64(0, 0), 0);
        assert_eq!(_andn_u64(0, 1), 1);
        assert_eq!(_andn_u64(1, 0), 0);
        assert_eq!(_andn_u64(1, 1), 0);

        let r = _andn_u64(0b0000_0000u64, 0b0000_0000u64);
        assert_eq!(r, 0b0000_0000u64);

        let r = _andn_u64(0b0000_0000u64, 0b1111_1111u64);
        assert_eq!(r, 0b1111_1111u64);

        let r = _andn_u64(0b1111_1111u64, 0b0000_0000u64);
        assert_eq!(r, 0b0000_0000u64);

        let r = _andn_u64(0b1111_1111u64, 0b1111_1111u64);
        assert_eq!(r, 0b0000_0000u64);

        let r = _andn_u64(0b0100_0000u64, 0b0101_1101u64);
        assert_eq!(r, 0b0001_1101u64);
    }

    #[simd_test = "bmi"]
    unsafe fn test_blsi_u64() {
        assert_eq!(_blsi_u64(0b1101_0000u64), 0b0001_0000u64);
    }

    #[simd_test = "bmi"]
    unsafe fn test_blsmsk_u64() {
        let r = _blsmsk_u64(0b0011_0000u64);
        assert_eq!(r, 0b0001_1111u64);
    }

    #[simd_test = "bmi"]
    unsafe fn test_blsr_u64() {
        // TODO: test the behavior when the input is 0
        let r = _blsr_u64(0b0011_0000u64);
        assert_eq!(r, 0b0010_0000u64);
    }

    #[simd_test = "bmi"]
    unsafe fn test_tzcnt_u64() {
        assert_eq!(_tzcnt_u64(0b0000_0001u64), 0u64);
        assert_eq!(_tzcnt_u64(0b0000_0000u64), 64u64);
        assert_eq!(_tzcnt_u64(0b1001_0000u64), 4u64);
    }
}
