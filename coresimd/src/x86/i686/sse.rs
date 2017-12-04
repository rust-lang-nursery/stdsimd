//! `i686` Streaming SIMD Extensions (SSE)

use v128::f32x4;
use v64::{i16x4, i32x2, i8x8, u8x8};
use x86::__m64;
use core::mem;
use x86::i586;
use x86::i686::mmx;

#[cfg(test)]
use stdsimd_test::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.sse.cvtpi2ps"]
    fn cvtpi2ps(a: f32x4, b: __m64) -> f32x4;
    #[link_name = "llvm.x86.mmx.pextr.w"]
    fn pextrw(a: __m64, imm8: i32) -> i32;
    #[link_name = "llvm.x86.mmx.pmaxs.w"]
    fn pmaxsw(a: __m64, b: __m64) -> __m64;
    #[link_name = "llvm.x86.mmx.pmaxu.b"]
    fn pmaxub(a: __m64, b: __m64) -> __m64;
    #[link_name = "llvm.x86.mmx.pmins.w"]
    fn pminsw(a: __m64, b: __m64) -> __m64;
    #[link_name = "llvm.x86.mmx.pminu.b"]
    fn pminub(a: __m64, b: __m64) -> __m64;
    #[link_name = "llvm.x86.sse.cvtps2pi"]
    fn cvtps2pi(a: f32x4) -> __m64;
    #[link_name = "llvm.x86.sse.cvttps2pi"]
    fn cvttps2pi(a: f32x4) -> __m64;
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pmaxsw))]
pub unsafe fn _mm_max_pi16(a: i16x4, b: i16x4) -> i16x4 {
    mem::transmute(pmaxsw(mem::transmute(a), mem::transmute(b)))
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pmaxsw))]
pub unsafe fn _m_pmaxsw(a: i16x4, b: i16x4) -> i16x4 {
    _mm_max_pi16(a, b)
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pmaxub))]
pub unsafe fn _mm_max_pu8(a: u8x8, b: u8x8) -> u8x8 {
    mem::transmute(pmaxub(mem::transmute(a), mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// greatest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pmaxub))]
pub unsafe fn _m_pmaxub(a: u8x8, b: u8x8) -> u8x8 {
    _mm_max_pu8(a, b)
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pminsw))]
pub unsafe fn _mm_min_pi16(a: i16x4, b: i16x4) -> i16x4 {
    mem::transmute(pminsw(mem::transmute(a), mem::transmute(b)))
}

/// Compares the packed 16-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pminsw))]
pub unsafe fn _m_pminsw(a: i16x4, b: i16x4) -> i16x4 {
    _mm_min_pi16(a, b)
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pminub))]
pub unsafe fn _mm_min_pu8(a: u8x8, b: u8x8) -> u8x8 {
    mem::transmute(pminub(mem::transmute(a), mem::transmute(b)))
}

/// Compares the packed 8-bit signed integers of `a` and `b` writing the
/// smallest value into the result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pminub))]
pub unsafe fn _m_pminub(a: u8x8, b: u8x8) -> u8x8 {
    _mm_min_pu8(a, b)
}

/// Converts two elements of a 64-bit vector of [2 x i32] into two
/// floating point values and writes them to the lower 64-bits of the
/// destination. The remaining higher order elements of the destination are
/// copied from the corresponding elements in the first operand.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvtpi2ps))]
pub unsafe fn _mm_cvt_pi2ps(a: f32x4, b: i32x2) -> f32x4 {
    cvtpi2ps(a, mem::transmute(b))
}

/// Extracts 16-bit element from a 64-bit vector of [4 x i16] and
/// returns it, as specified by the immediate integer operand.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(pextrw, imm8 = 0))]
pub unsafe fn _mm_extract_pi16(a: i16x4, imm8: i32) -> i16 {
    // a.extract((imm8 & 0b11) as u32)
    pextrw(mem::transmute(a), imm8) as i16
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers with truncation.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvttps2pi))]
pub unsafe fn _mm_cvttps_pi32(a: f32x4) -> i32x2 {
    mem::transmute(cvttps2pi(a))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers with truncation.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvttps2pi))]
pub unsafe fn _mm_cvtt_ps2pi(a: f32x4) -> i32x2 {
    _mm_cvttps_pi32(a)
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi32(a: f32x4) -> i32x2 {
    mem::transmute(cvtps2pi(a))
}

/// Convert the two lower packed single-precision (32-bit) floating-point
/// elements in `a` to packed 32-bit integers.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvt_ps2pi(a: f32x4) -> i32x2 {
    _mm_cvtps_pi32(a)
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed 16-bit integers.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi16(a: f32x4) -> i16x4 {
    let b = _mm_cvtps_pi32(a);
    let a = i586::_mm_movehl_ps(a, a);
    let c = _mm_cvtps_pi32(a);
    mmx::_mm_packs_pi32(b, c)
}

/// Convert packed single-precision (32-bit) floating-point elements in `a` to
/// packed 8-bit integers, and returns theem in the lower 4 elements of the
/// result.
#[inline(always)]
#[target_feature = "+sse"]
#[cfg_attr(test, assert_instr(cvtps2pi))]
pub unsafe fn _mm_cvtps_pi8(a: f32x4) -> i8x8 {
    let b = _mm_cvtps_pi16(a);
    let c = mmx::_mm_setzero_si64();
    mmx::_mm_packs_pi16(b, mem::transmute(c))
}

#[cfg(test)]
mod tests {
    use v128::f32x4;
    use v64::{i16x4, i32x2, i8x8, u8x8};
    use x86::i686::sse;
    use stdsimd_test::simd_test;

    #[simd_test = "sse"]
    unsafe fn _mm_max_pi16() {
        let a = i16x4::new(-1, 6, -3, 8);
        let b = i16x4::new(5, -2, 7, -4);
        let r = i16x4::new(5, 6, 7, 8);

        assert_eq!(r, sse::_mm_max_pi16(a, b));
        assert_eq!(r, sse::_m_pmaxsw(a, b));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_max_pu8() {
        let a = u8x8::new(2, 6, 3, 8, 2, 6, 3, 8);
        let b = u8x8::new(5, 2, 7, 4, 5, 2, 7, 4);
        let r = u8x8::new(5, 6, 7, 8, 5, 6, 7, 8);

        assert_eq!(r, sse::_mm_max_pu8(a, b));
        assert_eq!(r, sse::_m_pmaxub(a, b));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_min_pi16() {
        let a = i16x4::new(-1, 6, -3, 8);
        let b = i16x4::new(5, -2, 7, -4);
        let r = i16x4::new(-1, -2, -3, -4);

        assert_eq!(r, sse::_mm_min_pi16(a, b));
        assert_eq!(r, sse::_m_pminsw(a, b));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_min_pu8() {
        let a = u8x8::new(2, 6, 3, 8, 2, 6, 3, 8);
        let b = u8x8::new(5, 2, 7, 4, 5, 2, 7, 4);
        let r = u8x8::new(2, 2, 3, 4, 2, 2, 3, 4);

        assert_eq!(r, sse::_mm_min_pu8(a, b));
        assert_eq!(r, sse::_m_pminub(a, b));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cvtps_pi32() {
        let a = f32x4::new(1.0, 2.0, 3.0, 4.0);
        let r = i32x2::new(1, 2);

        assert_eq!(r, sse::_mm_cvtps_pi32(a));
        assert_eq!(r, sse::_mm_cvt_ps2pi(a));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cvttps_pi32() {
        let a = f32x4::new(7.0, 2.0, 3.0, 4.0);
        let r = i32x2::new(7, 2);

        assert_eq!(r, sse::_mm_cvttps_pi32(a));
        assert_eq!(r, sse::_mm_cvtt_ps2pi(a));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cvtps_pi16() {
        let a = f32x4::new(7.0, 2.0, 3.0, 4.0);
        let r = i16x4::new(7, 2, 3, 4);
        assert_eq!(r, sse::_mm_cvtps_pi16(a));
    }

    #[simd_test = "sse"]
    unsafe fn _mm_cvtps_pi8() {
        let a = f32x4::new(7.0, 2.0, 3.0, 4.0);
        let r = i8x8::new(7, 2, 3, 4, 0, 0, 0, 0);
        assert_eq!(r, sse::_mm_cvtps_pi8(a));
    }
}
