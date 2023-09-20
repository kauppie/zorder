use num_traits::PrimInt;

/// Used to determine the number of bits in the given type.
pub trait BitCount {
    const BITS: u32;

    // HACK: only works for powers of 2.
    const BITS_ILOG2: u32 = Self::BITS.trailing_zeros();
}

macro_rules! impl_bit_count {
    ($($t:ty),*) => {
        $(
            impl BitCount for $t {
                const BITS: u32 = Self::BITS;
            }
        )*
    };
}

impl_bit_count! {
    u8, u16, u32, u64, u128
}

/// Used to determine the minimum width output type which
/// fits the given input type `N` (dimensions) number of times.
pub trait DimensionOutput<const N: usize>: private::Sealed {
    type Output: num_traits::PrimInt + BitCount;
}

macro_rules! impl_dimension_output {
    ($($impl_type:ty, $dim:expr => $out_type:ty);*) => {
        $(
            impl DimensionOutput<$dim> for $impl_type {
                type Output = $out_type;
            }
        )*
    };
}

impl_dimension_output! {
    u8, 2 => u16;
    u8, 3 => u32;
    u8, 4 => u32;
    u16, 2 => u32;
    u16, 3 => u64;
    u16, 4 => u64;
    u32, 2 => u64;
    u32, 3 => u128;
    u32, 4 => u128;
    u64, 2 => u128
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

    // NOTE: This is a workaround to not need type conversions in runtime code.
    const U32_DIM: u32 = N as u32;

    fn interleave(self) -> Self::Output;
}

impl<T, const N: usize> Interleave<N> for T
where
    T: DimensionOutput<N> + BitCount + PrimInt,
{
    type Output = <T as DimensionOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x: Self::Output = unsafe { num_cast(self) };

        for i in (0..<Self as BitCount>::BITS_ILOG2).rev() {
            let mask = interleave_mask(Self::U32_DIM, 1 << i);
            let shift_count = interleave_shift(i, Self::U32_DIM);

            x = (x | x.unsigned_shl(shift_count)) & mask;
        }

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
#[inline(always)]
const fn interleave_shift(i: u32, n: u32) -> u32 {
    (1 << i) * (n - 1)
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

    #[test]
    fn ilog2() {
        assert_eq!(u8::BITS_ILOG2, 3);
        assert_eq!(u16::BITS_ILOG2, 4);
        assert_eq!(u32::BITS_ILOG2, 5);
        assert_eq!(u64::BITS_ILOG2, 6);
        assert_eq!(u128::BITS_ILOG2, 7);
    }
}
