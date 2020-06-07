use crate::{
    core_arch::{simd::*, simd_llvm::*, x86::*},
    mem::{self, transmute},
};

#[cfg(test)]
use stdarch_test::assert_instr;

/// Computes the absolute values of packed 32-bit integers in `a`.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#avx512techs=AVX512F&expand=33,34,4990,33&text=_mm512_abs_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpabsd))]
pub unsafe fn _mm512_abs_epi32(a: __m512i) -> __m512i {
    let a = a.as_i32x16();
    // all-0 is a properly initialized i32x16
    let zero: i32x16 = mem::zeroed();
    let sub = simd_sub(zero, a);
    let cmp: i32x16 = simd_gt(a, zero);
    transmute(simd_select(cmp, a, sub))
}

/// Computes the absolute value of packed 32-bit integers in `a`, and store the
/// unsigned results in `dst` using writemask `k` (elements are copied from
/// `src` when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#avx512techs=AVX512F&expand=33,34,4990,33&text=_mm512_abs_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpabsd))]
pub unsafe fn _mm512_mask_abs_epi32(src: __m512i, k: __mmask16, a: __m512i) -> __m512i {
    let abs = _mm512_abs_epi32(a).as_i32x16();
    transmute(simd_select_bitmask(k, abs, src.as_i32x16()))
}

/// Computes the absolute value of packed 32-bit integers in `a`, and store the
/// unsigned results in `dst` using zeromask `k` (elements are zeroed out when
/// the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#avx512techs=AVX512F&expand=33,34,4990,33,34,35,35&text=_mm512_maskz_abs_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpabsd))]
pub unsafe fn _mm512_maskz_abs_epi32(k: __mmask16, a: __m512i) -> __m512i {
    let abs = _mm512_abs_epi32(a).as_i32x16();
    let zero = _mm512_setzero_si512().as_i32x16();
    transmute(simd_select_bitmask(k, abs, zero))
}

/// Returns vector of type `__m512d` with all elements set to zero.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#avx512techs=AVX512F&expand=33,34,4990&text=_mm512_setzero_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm512_setzero_pd() -> __m512d {
    // All-0 is a properly initialized __m512d
    mem::zeroed()
}

/// Returns vector of type `__m512i` with all elements set to zero.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#avx512techs=AVX512F&expand=33,34,4990&text=_mm512_setzero_si512)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm512_setzero_si512() -> __m512i {
    // All-0 is a properly initialized __m512i
    mem::zeroed()
}

/// Sets packed 32-bit integers in `dst` with the supplied values in reverse
/// order.
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_setr_epi32(
    e15: i32,
    e14: i32,
    e13: i32,
    e12: i32,
    e11: i32,
    e10: i32,
    e9: i32,
    e8: i32,
    e7: i32,
    e6: i32,
    e5: i32,
    e4: i32,
    e3: i32,
    e2: i32,
    e1: i32,
    e0: i32,
) -> __m512i {
    let r = i32x16(
        e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    );
    transmute(r)
}

/// Gather double-precision (64-bit) floating-point elements from memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i32gather_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i32gather_pd(offsets: __m256i, slice: *const u8, scale: i32) -> __m512d {
    let zero = _mm512_setzero_pd().as_f64x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherdpd(zero, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather double-precision (64-bit) floating-point elements from memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i32gather_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherdpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i32gather_pd(
    src: __m512d,
    mask: __mmask8,
    offsets: __m256i,
    slice: *const u8,
    scale: i32,
) -> __m512d {
    let src = src.as_f64x8();
    let slice = slice as *const i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherdpd(src, slice, offsets, mask as i8, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather double-precision (64-bit) floating-point elements from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64gather_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i64gather_pd(offsets: __m512i, slice: *const u8, scale: i32) -> __m512d {
    let zero = _mm512_setzero_pd().as_f64x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherqpd(zero, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather double-precision (64-bit) floating-point elements from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64gather_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherqpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64gather_pd(
    src: __m512d,
    mask: __mmask8,
    offsets: __m512i,
    slice: *const u8,
    scale: i32,
) -> __m512d {
    let src = src.as_f64x8();
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherqpd(src, slice, offsets, mask as i8, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather single-precision (32-bit) floating-point elements from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64gather_ps)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i64gather_ps(offsets: __m512i, slice: *const u8, scale: i32) -> __m256 {
    let zero = _mm256_setzero_ps().as_f32x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherqps(zero, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather single-precision (32-bit) floating-point elements from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64gather_ps)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vgatherqps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64gather_ps(
    src: __m256,
    mask: __mmask8,
    offsets: __m512i,
    slice: *const u8,
    scale: i32,
) -> __m256 {
    let src = src.as_f32x8();
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vgatherqps(src, slice, offsets, mask as i8, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 64-bit integers from memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i32gather_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i32gather_epi64(offsets: __m256i, slice: *const u8, scale: i32) -> __m512i {
    let zero = _mm512_setzero_si512().as_i64x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherdq(zero, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 64-bit integers from memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i32gather_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherdq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i32gather_epi64(
    src: __m512i,
    mask: __mmask8,
    offsets: __m256i,
    slice: *const u8,
    scale: i32,
) -> __m512i {
    let src = src.as_i64x8();
    let mask = mask as i8;
    let slice = slice as *const i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherdq(src, slice, offsets, mask, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 64-bit integers from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64gather_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i64gather_epi64(offsets: __m512i, slice: *const u8, scale: i32) -> __m512i {
    let zero = _mm512_setzero_si512().as_i64x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherqq(zero, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 64-bit integers from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64gather_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherqq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64gather_epi64(
    src: __m512i,
    mask: __mmask8,
    offsets: __m512i,
    slice: *const u8,
    scale: i32,
) -> __m512i {
    let src = src.as_i64x8();
    let mask = mask as i8;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherqq(src, slice, offsets, mask, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 32-bit integers from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64gather_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(2)]
pub unsafe fn _mm512_i64gather_epi32(offsets: __m512i, slice: *const u8, scale: i32) -> __m256i {
    let zeros = _mm256_setzero_si256().as_i32x8();
    let neg_one = -1;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherqd(zeros, slice, offsets, neg_one, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Gather 32-bit integers from memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64gather_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpgatherqd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64gather_epi32(
    src: __m256i,
    mask: __mmask8,
    offsets: __m512i,
    slice: *const u8,
    scale: i32,
) -> __m256i {
    let src = src.as_i32x8();
    let mask = mask as i8;
    let slice = slice as *const i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpgatherqd(src, slice, offsets, mask, $imm8)
        };
    }
    let r = constify_imm8_gather!(scale, call);
    transmute(r)
}

/// Scatter double-precision (64-bit) floating-point elements from memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i32scatter_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterdpd, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i32scatter_pd(slice: *mut u8, offsets: __m256i, src: __m512d, scale: i32) {
    let src = src.as_f64x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterdpd(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter double-precision (64-bit) floating-point elements from src into memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i32scatter_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterdpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i32scatter_pd(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m256i,
    src: __m512d,
    scale: i32,
) {
    let src = src.as_f64x8();
    let slice = slice as *mut i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterdpd(slice, mask as i8, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter double-precision (64-bit) floating-point elements from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64scatter_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterqpd, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i64scatter_pd(slice: *mut u8, offsets: __m512i, src: __m512d, scale: i32) {
    let src = src.as_f64x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterqpd(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter double-precision (64-bit) floating-point elements from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64scatter_pd)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterqpd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64scatter_pd(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m512i,
    src: __m512d,
    scale: i32,
) {
    let src = src.as_f64x8();
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterqpd(slice, mask as i8, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter single-precision (32-bit) floating-point elements from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64scatter_ps)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterqps, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i64scatter_ps(slice: *mut u8, offsets: __m512i, src: __m256, scale: i32) {
    let src = src.as_f32x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterqps(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter single-precision (32-bit) floating-point elements from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64scatter_ps)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vscatterqps, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64scatter_ps(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m512i,
    src: __m256,
    scale: i32,
) {
    let src = src.as_f32x8();
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vscatterqps(slice, mask as i8, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 64-bit integers from src into memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i32scatter_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterdq, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i32scatter_epi64(slice: *mut u8, offsets: __m256i, src: __m512i, scale: i32) {
    let src = src.as_i64x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterdq(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 64-bit integers from src into memory using 32-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i32scatter_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterdq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i32scatter_epi64(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m256i,
    src: __m512i,
    scale: i32,
) {
    let src = src.as_i64x8();
    let mask = mask as i8;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i32x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterdq(slice, mask, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 64-bit integers from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64scatter_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterqq, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i64scatter_epi64(slice: *mut u8, offsets: __m512i, src: __m512i, scale: i32) {
    let src = src.as_i64x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterqq(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 64-bit integers from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64scatter_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterqq, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64scatter_epi64(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m512i,
    src: __m512i,
    scale: i32,
) {
    let src = src.as_i64x8();
    let mask = mask as i8;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterqq(slice, mask, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 32-bit integers from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_i64scatter_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterqd, scale = 1))]
#[rustc_args_required_const(3)]
pub unsafe fn _mm512_i64scatter_epi32(slice: *mut u8, offsets: __m512i, src: __m256i, scale: i32) {
    let src = src.as_i32x8();
    let neg_one = -1;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterqd(slice, neg_one, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

/// Scatter 32-bit integers from src into memory using 64-bit indices.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_i64scatter_epi32)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpscatterqd, scale = 1))]
#[rustc_args_required_const(4)]
pub unsafe fn _mm512_mask_i64scatter_epi32(
    slice: *mut u8,
    mask: __mmask8,
    offsets: __m512i,
    src: __m256i,
    scale: i32,
) {
    let src = src.as_i32x8();
    let mask = mask as i8;
    let slice = slice as *mut i8;
    let offsets = offsets.as_i64x8();
    macro_rules! call {
        ($imm8:expr) => {
            vpscatterqd(slice, mask, offsets, src, $imm8)
        };
    }
    constify_imm8_gather!(scale, call);
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.avx512.gather.dpd.512"]
    fn vgatherdpd(src: f64x8, slice: *const i8, offsets: i32x8, mask: i8, scale: i32) -> f64x8;
    #[link_name = "llvm.x86.avx512.gather.qpd.512"]
    fn vgatherqpd(src: f64x8, slice: *const i8, offsets: i64x8, mask: i8, scale: i32) -> f64x8;
    #[link_name = "llvm.x86.avx512.gather.qps.512"]
    fn vgatherqps(src: f32x8, slice: *const i8, offsets: i64x8, mask: i8, scale: i32) -> f32x8;
    #[link_name = "llvm.x86.avx512.gather.dpq.512"]
    fn vpgatherdq(src: i64x8, slice: *const i8, offsets: i32x8, mask: i8, scale: i32) -> i64x8;
    #[link_name = "llvm.x86.avx512.gather.qpq.512"]
    fn vpgatherqq(src: i64x8, slice: *const i8, offsets: i64x8, mask: i8, scale: i32) -> i64x8;
    #[link_name = "llvm.x86.avx512.gather.qpi.512"]
    fn vpgatherqd(src: i32x8, slice: *const i8, offsets: i64x8, mask: i8, scale: i32) -> i32x8;

    #[link_name = "llvm.x86.avx512.scatter.dpd.512"]
    fn vscatterdpd(slice: *mut i8, mask: i8, offsets: i32x8, src: f64x8, scale: i32);
    #[link_name = "llvm.x86.avx512.scatter.qpd.512"]
    fn vscatterqpd(slice: *mut i8, mask: i8, offsets: i64x8, src: f64x8, scale: i32);
    #[link_name = "llvm.x86.avx512.scatter.qps.512"]
    fn vscatterqps(slice: *mut i8, mask: i8, offsets: i64x8, src: f32x8, scale: i32);
    #[link_name = "llvm.x86.avx512.scatter.dpq.512"]
    fn vpscatterdq(slice: *mut i8, mask: i8, offsets: i32x8, src: i64x8, scale: i32);
    #[link_name = "llvm.x86.avx512.scatter.qpq.512"]
    fn vpscatterqq(slice: *mut i8, mask: i8, offsets: i64x8, src: i64x8, scale: i32);
    #[link_name = "llvm.x86.avx512.scatter.qpi.512"]
    fn vpscatterqd(slice: *mut i8, mask: i8, offsets: i64x8, src: i32x8, scale: i32);

}

/// Broadcast 64-bit float `a` to all elements of `dst`.
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_set1_pd(a: f64) -> __m512d {
    transmute(f64x8::splat(a))
}

/// Broadcast 64-bit integer `a` to all elements of `dst`.
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_set1_epi64(a: i64) -> __m512i {
    transmute(i64x8::splat(a))
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmplt_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmplt_epu64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_lt(a.as_u64x8(), b.as_u64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmplt_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmplt_epu64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epu64_mask(a, b) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpgt_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpgt_epu64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_gt(a.as_u64x8(), b.as_u64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpgt_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpgt_epu64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epu64_mask(a, b) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmple_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmple_epu64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epu64_mask(b, a)
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmple_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmple_epu64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epu64_mask(b, a) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpge_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpge_epu64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epu64_mask(b, a)
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpge_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpge_epu64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epu64_mask(b, a) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpeq_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpeq_epu64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_eq(a.as_u64x8(), b.as_u64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpeq_epu64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpeq_epu64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpeq_epu64_mask(a, b) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmplt_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmplt_epi64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_lt(a.as_i64x8(), b.as_i64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmplt_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmplt_epi64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epi64_mask(a, b) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpgt_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpgt_epi64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_gt(a.as_i64x8(), b.as_i64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpgt_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpgt_epi64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epi64_mask(a, b) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmple_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmple_epi64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epi64_mask(b, a)
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmple_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmple_epi64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpgt_epi64_mask(b, a) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpge_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpge_epi64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epi64_mask(b, a)
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpge_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpge_epi64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmplt_epi64_mask(b, a) & m
}

/// Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062&text=_mm512_cmpeq_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_cmpeq_epi64_mask(a: __m512i, b: __m512i) -> __mmask8 {
    simd_bitmask::<__m512i, _>(simd_eq(a.as_i64x8(), b.as_i64x8()))
}

///Compare packed unsigned 64-bit integers in a and b for less-than, and store the results in a mask vector k
/// using zeromask m (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,1063&text=_mm512_mask_cmpeq_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
#[cfg_attr(test, assert_instr(vpcmp))]
pub unsafe fn _mm512_mask_cmpeq_epi64_mask(m: __mmask8, a: __m512i, b: __m512i) -> __mmask8 {
    _mm512_cmpeq_epi64_mask(a, b) & m
}

#[cfg(test)]
mod tests {
    use std;
    use stdarch_test::simd_test;

    use crate::core_arch::x86::*;

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_abs_epi32() {
        #[rustfmt::skip]
        let a = _mm512_setr_epi32(
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
        );
        let r = _mm512_abs_epi32(a);
        let e = _mm512_setr_epi32(
            0,
            1,
            1,
            i32::MAX,
            i32::MAX.wrapping_add(1),
            100,
            100,
            32,
            0,
            1,
            1,
            i32::MAX,
            i32::MAX.wrapping_add(1),
            100,
            100,
            32,
        );
        assert_eq_m512i(r, e);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_abs_epi32() {
        #[rustfmt::skip]
        let a = _mm512_setr_epi32(
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
        );
        let r = _mm512_mask_abs_epi32(a, 0, a);
        assert_eq_m512i(r, a);
        let r = _mm512_mask_abs_epi32(a, 0b11111111, a);
        let e = _mm512_setr_epi32(
            0,
            1,
            1,
            i32::MAX,
            i32::MAX.wrapping_add(1),
            100,
            100,
            32,
            0,
            1,
            -1,
            i32::MAX,
            i32::MIN,
            100,
            -100,
            -32,
        );
        assert_eq_m512i(r, e);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_maskz_abs_epi32() {
        #[rustfmt::skip]
        let a = _mm512_setr_epi32(
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
            0, 1, -1, i32::MAX,
            i32::MIN, 100, -100, -32,
        );
        let r = _mm512_maskz_abs_epi32(0, a);
        assert_eq_m512i(r, _mm512_setzero_si512());
        let r = _mm512_maskz_abs_epi32(0b11111111, a);
        let e = _mm512_setr_epi32(
            0,
            1,
            1,
            i32::MAX,
            i32::MAX.wrapping_add(1),
            100,
            100,
            32,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        );
        assert_eq_m512i(r, e);
    }
}
