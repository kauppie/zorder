pub trait BitCount {
    const BITS: u32;
}

impl BitCount for u8 {
    const BITS: u32 = Self::BITS;
}

impl BitCount for u16 {
    const BITS: u32 = Self::BITS;
}

impl BitCount for u32 {
    const BITS: u32 = Self::BITS;
}

impl BitCount for u64 {
    const BITS: u32 = Self::BITS;
}

impl BitCount for u128 {
    const BITS: u32 = Self::BITS;
}

pub trait DimensionOutput<const N: usize>: private::Sealed {
    type Output: num_traits::PrimInt + BitCount;
}

impl DimensionOutput<2> for u8 {
    type Output = u16;
}

impl DimensionOutput<3> for u8 {
    type Output = u32;
}

impl DimensionOutput<4> for u8 {
    type Output = u32;
}

impl DimensionOutput<2> for u16 {
    type Output = u32;
}

impl DimensionOutput<3> for u16 {
    type Output = u64;
}

impl DimensionOutput<4> for u16 {
    type Output = u64;
}

impl DimensionOutput<2> for u32 {
    type Output = u64;
}

impl DimensionOutput<3> for u32 {
    type Output = u128;
}

impl DimensionOutput<4> for u32 {
    type Output = u128;
}

impl DimensionOutput<2> for u64 {
    type Output = u128;
}

/// Interleaves the bits of the given number, while taking output dimension
/// into account.
///
/// Naive implementation of this algorithm would be O(n) where n is the
/// number of bits in the number. Implementations seen here are O(log n).
/// They are extrapolated and generalized from the algorithm described here:
/// http://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN.
pub trait Interleave<const N: usize>: private::Sealed {
    type Output: num_traits::PrimInt;

    fn interleave(self) -> Self::Output;
}

impl<const N: usize> Interleave<N> for u8
where
    u8: DimensionOutput<N>,
{
    type Output = <u8 as DimensionOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<2, N>())) & interleave_mask(N as u32, 1 << 2);
        x = (x | (x << interleave_shift::<1, N>())) & interleave_mask(N as u32, 1 << 1);
        x = (x | (x << interleave_shift::<0, N>())) & interleave_mask(N as u32, 1 << 0);

        x
    }
}

impl<const N: usize> Interleave<N> for u16
where
    u16: DimensionOutput<N>,
{
    type Output = <u16 as DimensionOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<3, N>())) & interleave_mask(N as u32, 1 << 3);
        x = (x | (x << interleave_shift::<2, N>())) & interleave_mask(N as u32, 1 << 2);
        x = (x | (x << interleave_shift::<1, N>())) & interleave_mask(N as u32, 1 << 1);
        x = (x | (x << interleave_shift::<0, N>())) & interleave_mask(N as u32, 1 << 0);

        x
    }
}

impl<const N: usize> Interleave<N> for u32
where
    u32: DimensionOutput<N>,
{
    type Output = <u32 as DimensionOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<4, N>())) & interleave_mask(N as u32, 1 << 4);
        x = (x | (x << interleave_shift::<3, N>())) & interleave_mask(N as u32, 1 << 3);
        x = (x | (x << interleave_shift::<2, N>())) & interleave_mask(N as u32, 1 << 2);
        x = (x | (x << interleave_shift::<1, N>())) & interleave_mask(N as u32, 1 << 1);
        x = (x | (x << interleave_shift::<0, N>())) & interleave_mask(N as u32, 1 << 0);

        x
    }
}

impl<const N: usize> Interleave<N> for u64
where
    u64: DimensionOutput<N>,
{
    type Output = <u64 as DimensionOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<5, N>())) & interleave_mask(N as u32, 1 << 5);
        x = (x | (x << interleave_shift::<4, N>())) & interleave_mask(N as u32, 1 << 4);
        x = (x | (x << interleave_shift::<3, N>())) & interleave_mask(N as u32, 1 << 3);
        x = (x | (x << interleave_shift::<2, N>())) & interleave_mask(N as u32, 1 << 2);
        x = (x | (x << interleave_shift::<1, N>())) & interleave_mask(N as u32, 1 << 1);
        x = (x | (x << interleave_shift::<0, N>())) & interleave_mask(N as u32, 1 << 0);

        x
    }
}

/// Casts numeric types without checking for success.
#[inline(always)]
unsafe fn num_cast<I, O>(input: I) -> O
where
    I: num_traits::ToPrimitive,
    O: num_traits::NumCast,
{
    <O as num_traits::NumCast>::from(input).unwrap_unchecked()
}

/// Calculates the shift amount for the given interleave step and dimension.
const fn interleave_shift<const I: usize, const N: usize>() -> usize {
    (1 << I) * (N - 1)
}

/// # Panics
///
/// If any of the following conditions are met:
///
/// - `bits` == 0
/// - `bits` > `T::BITS`.
#[inline(always)]
fn mask<T: num_traits::PrimInt + BitCount>(bits: u32) -> T {
    <T as num_traits::Bounded>::max_value().unsigned_shr(<T as BitCount>::BITS - bits)
}

#[inline(always)]
fn interleave_mask<T: num_traits::PrimInt + BitCount>(dim: u32, bits: u32) -> T {
    let mut acc = <T as num_traits::Zero>::zero();
    let mask = mask::<T>(bits);

    let ceil_div_dim = (<T as BitCount>::BITS + dim - 1) / dim;
    let ceil_div_bits = (ceil_div_dim + bits - 1) / bits;

    for i in 0..ceil_div_bits {
        acc = acc | mask.unsigned_shl(i * dim * bits);
    }

    acc
}

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

mod tests {
    // This is false positive.
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn allowed_masks() {
        assert_eq!(mask::<u8>(4), 0xF);
        assert_eq!(mask::<u16>(4), 0xF);
        assert_eq!(mask::<u32>(4), 0xF);
        assert_eq!(mask::<u64>(4), 0xF);
        assert_eq!(mask::<u128>(4), 0xF);

        assert_eq!(mask::<u8>(8), 0xFF);
        assert_eq!(mask::<u16>(16), 0xFFFF);
        assert_eq!(mask::<u32>(32), 0xFFFF_FFFF);
        assert_eq!(mask::<u64>(64), 0xFFFF_FFFF_FFFF_FFFF);
        assert_eq!(mask::<u128>(128), 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    }

    #[test]
    #[should_panic]
    fn zero_mask() {
        mask::<u8>(0);
    }

    #[test]
    #[should_panic]
    fn too_large_mask() {
        mask::<u8>(u8::MAX as u32 + 1);
    }

    #[test]
    fn dim2() {
        assert_eq!(
            interleave_mask::<u128>(2, 32),
            0x0000_0000_FFFF_FFFF_00000000_FFFF_FFFF
        );
        assert_eq!(
            interleave_mask::<u128>(2, 16),
            0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF
        );
        assert_eq!(
            interleave_mask::<u128>(2, 8),
            0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF
        );
        assert_eq!(
            interleave_mask::<u128>(2, 4),
            0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F
        );
        assert_eq!(
            interleave_mask::<u128>(2, 2),
            0x3333_3333_3333_3333_3333_3333_3333_3333
        );
        assert_eq!(
            interleave_mask::<u128>(2, 1),
            0x5555_5555_5555_5555_5555_5555_5555_5555
        );
    }

    #[test]
    fn dim3() {
        assert_eq!(
            interleave_mask::<u128>(3, 16),
            0x0000_FFFF_0000_0000_FFFF_0000_0000_FFFF
        );
        assert_eq!(
            interleave_mask::<u128>(3, 8),
            0xFF00_00FF_0000_FF00_00FF_0000_FF00_00FF
        );
        assert_eq!(
            interleave_mask::<u128>(3, 4),
            0x0F00_F00F_00F0_0F00_F00F_00F0_0F00_F00F
        );
        assert_eq!(
            interleave_mask::<u128>(3, 2),
            0xC30C_30C3_0C30_C30C_30C3_0C30_C30C_30C3
        );
        assert_eq!(
            interleave_mask::<u128>(3, 1),
            0x4924_9249_2492_4924_9249_2492_4924_9249
        );
    }

    #[test]
    fn dim4() {
        assert_eq!(
            interleave_mask::<u128>(4, 16),
            0x0000_0000_0000_FFFF_0000_0000_0000_FFFF
        );
        assert_eq!(
            interleave_mask::<u128>(4, 8),
            0x0000_00FF_0000_00FF_0000_00FF_0000_00FF
        );
        assert_eq!(
            interleave_mask::<u128>(4, 4),
            0x000F_000F_000F_000F_000F_000F_000F_000F
        );
        assert_eq!(
            interleave_mask::<u128>(4, 2),
            0x0303_0303_0303_0303_0303_0303_0303_0303
        );
        assert_eq!(
            interleave_mask::<u128>(4, 1),
            0x1111_1111_1111_1111_1111_1111_1111_1111
        );
    }

    #[test]
    fn truncated_interleave_mask() {
        assert_eq!(interleave_mask::<u32>(2, 32), 0xFFFF_FFFF);
        assert_eq!(interleave_mask::<u32>(2, 16), 0x0000_FFFF);
        assert_eq!(interleave_mask::<u32>(2, 8), 0x00FF_00FF);
        assert_eq!(interleave_mask::<u32>(2, 4), 0x0F0F_0F0F);
        assert_eq!(interleave_mask::<u32>(2, 2), 0x3333_3333);
        assert_eq!(interleave_mask::<u32>(2, 1), 0x5555_5555);
    }
}
