const MASK_5_DIM_2: u128 = 0x00000000FFFFFFFF00000000FFFFFFFF;
const MASK_4_DIM_2: u128 = 0x0000FFFF0000FFFF0000FFFF0000FFFF;
const MASK_3_DIM_2: u128 = 0x00FF00FF00FF00FF00FF00FF00FF00FF;
const MASK_2_DIM_2: u128 = 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F;
const MASK_1_DIM_2: u128 = 0x33333333333333333333333333333333;
const MASK_0_DIM_2: u128 = 0x55555555555555555555555555555555;

const MASK_4_DIM_3: u128 = 0x0000FFFF00000000FFFF00000000FFFF;
const MASK_3_DIM_3: u128 = 0xFF0000FF0000FF0000FF0000FF0000FF;
const MASK_2_DIM_3: u128 = 0x0F00F00F00F00F00F00F00F00F00F00F;
const MASK_1_DIM_3: u128 = 0xC30C30C30C30C30C30C30C30C30C30C3;
const MASK_0_DIM_3: u128 = 0x49249249249249249249249249249249;

const MASK_4_DIM_4: u128 = 0x000000000000FFFF000000000000FFFF;
const MASK_3_DIM_4: u128 = 0x000000FF000000FF000000FF000000FF;
const MASK_2_DIM_4: u128 = 0x000F000F000F000F000F000F000F000F;
const MASK_1_DIM_4: u128 = 0x03030303030303030303030303030303;
const MASK_0_DIM_4: u128 = 0x11111111111111111111111111111111;

pub trait Mask<const N: usize> {
    type Output: num_traits::PrimInt + BitCount;

    const MASK_5: Self::Output;
    const MASK_4: Self::Output;
    const MASK_3: Self::Output;
    const MASK_2: Self::Output;
    const MASK_1: Self::Output;
    const MASK_0: Self::Output;
}

impl Mask<2> for u8 {
    type Output = u16;

    const MASK_2: Self::Output = MASK_2_DIM_2 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_2 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_2 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
    const MASK_3: Self::Output = 0;
}

impl Mask<3> for u8 {
    type Output = u32;

    const MASK_2: Self::Output = MASK_2_DIM_3 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_3 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_3 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
    const MASK_3: Self::Output = 0;
}

impl Mask<4> for u8 {
    type Output = u32;

    const MASK_2: Self::Output = MASK_2_DIM_4 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_4 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_4 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
    const MASK_3: Self::Output = 0;
}

impl Mask<2> for u16 {
    type Output = u32;

    const MASK_3: Self::Output = MASK_3_DIM_2 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_2 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_2 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_2 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
}

impl Mask<3> for u16 {
    type Output = u64;

    const MASK_3: Self::Output = MASK_3_DIM_3 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_3 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_3 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_3 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
}

impl Mask<4> for u16 {
    type Output = u64;

    const MASK_3: Self::Output = MASK_3_DIM_4 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_4 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_4 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_4 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
    const MASK_4: Self::Output = 0;
}

impl Mask<2> for u32 {
    type Output = u64;

    const MASK_4: Self::Output = MASK_4_DIM_2 as Self::Output;
    const MASK_3: Self::Output = MASK_3_DIM_2 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_2 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_2 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_2 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
}

impl Mask<3> for u32 {
    type Output = u128;

    const MASK_4: Self::Output = MASK_4_DIM_3 as Self::Output;
    const MASK_3: Self::Output = MASK_3_DIM_3 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_3 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_3 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_3 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
}

impl Mask<4> for u32 {
    type Output = u128;

    const MASK_4: Self::Output = MASK_4_DIM_4 as Self::Output;
    const MASK_3: Self::Output = MASK_3_DIM_4 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_4 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_4 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_4 as Self::Output;

    // Unused masks.
    const MASK_5: Self::Output = 0;
}

impl Mask<2> for u64 {
    type Output = u128;

    const MASK_5: Self::Output = MASK_5_DIM_2 as Self::Output;
    const MASK_4: Self::Output = MASK_4_DIM_2 as Self::Output;
    const MASK_3: Self::Output = MASK_3_DIM_2 as Self::Output;
    const MASK_2: Self::Output = MASK_2_DIM_2 as Self::Output;
    const MASK_1: Self::Output = MASK_1_DIM_2 as Self::Output;
    const MASK_0: Self::Output = MASK_0_DIM_2 as Self::Output;
}

pub trait Interleave<const N: usize>: private::Sealed {
    type Output: num_traits::PrimInt;

    fn interleave(self) -> Self::Output;
}

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

impl<const N: usize> Interleave<N> for u8
where
    u8: Mask<N>,
{
    type Output = <u8 as Mask<N>>::Output;

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
    u16: Mask<N>,
{
    type Output = <u16 as Mask<N>>::Output;

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
    u32: Mask<N>,
{
    type Output = <u32 as Mask<N>>::Output;

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
    u64: Mask<N>,
{
    type Output = <u64 as Mask<N>>::Output;

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

// Masks with zero bits are not allowed.
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

mod tests {
    // This is false positive.
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn mask_test() {
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
    fn dim2() {
        assert_eq!(interleave_mask::<u128>(2, 32), MASK_5_DIM_2);
        assert_eq!(interleave_mask::<u128>(2, 16), MASK_4_DIM_2);
        assert_eq!(interleave_mask::<u128>(2, 8), MASK_3_DIM_2);
        assert_eq!(interleave_mask::<u128>(2, 4), MASK_2_DIM_2);
        assert_eq!(interleave_mask::<u128>(2, 2), MASK_1_DIM_2);
        assert_eq!(interleave_mask::<u128>(2, 1), MASK_0_DIM_2);
    }

    #[test]
    fn dim3() {
        assert_eq!(interleave_mask::<u128>(3, 16), MASK_4_DIM_3);
        assert_eq!(interleave_mask::<u128>(3, 8), MASK_3_DIM_3);
        assert_eq!(interleave_mask::<u128>(3, 4), MASK_2_DIM_3);
        assert_eq!(interleave_mask::<u128>(3, 2), MASK_1_DIM_3);
        assert_eq!(interleave_mask::<u128>(3, 1), MASK_0_DIM_3);
    }

    #[test]
    fn dim4() {
        assert_eq!(interleave_mask::<u128>(4, 16), MASK_4_DIM_4);
        assert_eq!(interleave_mask::<u128>(4, 8), MASK_3_DIM_4);
        assert_eq!(interleave_mask::<u128>(4, 4), MASK_2_DIM_4);
        assert_eq!(interleave_mask::<u128>(4, 2), MASK_1_DIM_4);
        assert_eq!(interleave_mask::<u128>(4, 1), MASK_0_DIM_4);
    }

    #[test]
    fn truncated_mask() {
        assert_eq!(interleave_mask::<u32>(2, 32), MASK_5_DIM_2 as u32);
        assert_eq!(interleave_mask::<u32>(2, 16), MASK_4_DIM_2 as u32);
        assert_eq!(interleave_mask::<u32>(2, 8), MASK_3_DIM_2 as u32);
        assert_eq!(interleave_mask::<u32>(2, 4), MASK_2_DIM_2 as u32);
        assert_eq!(interleave_mask::<u32>(2, 2), MASK_1_DIM_2 as u32);
        assert_eq!(interleave_mask::<u32>(2, 1), MASK_0_DIM_2 as u32);
    }
}
