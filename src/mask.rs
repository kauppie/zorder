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
    type Output: num_traits::PrimInt;

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

pub trait DimMask<const N: usize> {
    type Output: num_traits::PrimInt;

    fn interleave(self) -> Self::Output;
}

impl<const N: usize> DimMask<N> for u8
where
    u8: Mask<N>,
{
    type Output = <u8 as Mask<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<2, N>())) & <u8 as Mask<N>>::MASK_2;
        x = (x | (x << interleave_shift::<1, N>())) & <u8 as Mask<N>>::MASK_1;
        x = (x | (x << interleave_shift::<0, N>())) & <u8 as Mask<N>>::MASK_0;

        x
    }
}

impl<const N: usize> DimMask<N> for u16
where
    u16: Mask<N>,
{
    type Output = <u16 as Mask<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<3, N>())) & <u16 as Mask<N>>::MASK_3;
        x = (x | (x << interleave_shift::<2, N>())) & <u16 as Mask<N>>::MASK_2;
        x = (x | (x << interleave_shift::<1, N>())) & <u16 as Mask<N>>::MASK_1;
        x = (x | (x << interleave_shift::<0, N>())) & <u16 as Mask<N>>::MASK_0;

        x
    }
}

impl<const N: usize> DimMask<N> for u32
where
    u32: Mask<N>,
{
    type Output = <u32 as Mask<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<4, N>())) & <u32 as Mask<N>>::MASK_4;
        x = (x | (x << interleave_shift::<3, N>())) & <u32 as Mask<N>>::MASK_3;
        x = (x | (x << interleave_shift::<2, N>())) & <u32 as Mask<N>>::MASK_2;
        x = (x | (x << interleave_shift::<1, N>())) & <u32 as Mask<N>>::MASK_1;
        x = (x | (x << interleave_shift::<0, N>())) & <u32 as Mask<N>>::MASK_0;

        x
    }
}

impl<const N: usize> DimMask<N> for u64
where
    u64: Mask<N>,
{
    type Output = <u64 as Mask<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x = unsafe { num_cast(self) };

        x = (x | (x << interleave_shift::<5, N>())) & <u64 as Mask<N>>::MASK_5;
        x = (x | (x << interleave_shift::<4, N>())) & <u64 as Mask<N>>::MASK_4;
        x = (x | (x << interleave_shift::<3, N>())) & <u64 as Mask<N>>::MASK_3;
        x = (x | (x << interleave_shift::<2, N>())) & <u64 as Mask<N>>::MASK_2;
        x = (x | (x << interleave_shift::<1, N>())) & <u64 as Mask<N>>::MASK_1;
        x = (x | (x << interleave_shift::<0, N>())) & <u64 as Mask<N>>::MASK_0;

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
