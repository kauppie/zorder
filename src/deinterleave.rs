use num_traits::{cast::AsPrimitive, PrimInt};

use crate::mask::{interleave_mask, interleave_shift, BitCount};

/// Deinterleave a single number from a set of interleaved numbers.
///
// Original idea from:
/// https://stackoverflow.com/questions/4909263/how-to-efficiently-de-interleave-bits-inverse-morton
///
/// Current implementation is rather based on the [Interleave](crate::interleave::Interleave) trait
/// and its implementation.
pub trait Deinterleave<const N: usize>: private::Sealed {
    type Output;

    /// Deinterleaves a number from a set of interleaved numbers starting from
    /// the given least significant bit `lsb` position.
    ///
    /// For `lsb` == 0, the actual LSB of the number is used is the first
    /// interleaved bit. Dimension `N` is used to determine the number of
    /// bits there are between output's bits.
    fn deinterleave(self, lsb: usize) -> Self::Output;
}

impl<T, const N: usize> Deinterleave<N> for T
where
    T: DeinterleaveOutput<N>,
    T: AsPrimitive<<Self as DeinterleaveOutput<N>>::Output>,
    T: BitCount + PrimInt,
{
    type Output = <Self as DeinterleaveOutput<N>>::Output;

    #[inline(always)]
    fn deinterleave(self, lsb: usize) -> <Self as Deinterleave<N>>::Output {
        let mut x = (self >> lsb) & interleave_mask(N as u32, 1);

        for i in 0..<Self::Output as BitCount>::BITS_ILOG2 {
            let mask = interleave_mask(N as u32, 1 << (i + 1));
            let shift_count = interleave_shift(i, N as u32);

            x = (x | x.unsigned_shr(shift_count)) & mask;
        }

        x.as_()
    }
}

/// Used to determine the minimum width output type which fits
/// all dimensions `N` stored in the input type.
///
/// Inverse conversion of [`InterleaveOutput`](crate::interleave::InterleaveOutput).
pub trait DeinterleaveOutput<const N: usize>: private::Sealed {
    type Output: BitCount + PrimInt;
}

macro_rules! impl_deinterleave_output {
    ($($impl_type:ty => $dim:expr, $out_type:ty);*) => {
        $(
            impl DeinterleaveOutput<$dim> for $impl_type {
                type Output = $out_type;
            }
        )*
    };
}

impl_deinterleave_output! {
    u16 => 2, u8;
    u32 => 2, u16;
    u32 => 3, u8;
    u32 => 4, u8;
    u64 => 2, u32;
    u64 => 3, u16;
    u64 => 4, u16;
    u64 => 5, u8;
    u64 => 6, u8;
    u64 => 7, u8;
    u64 => 8, u8;
    u128 => 2, u64;
    u128 => 3, u32;
    u128 => 4, u32;
    u128 => 5, u16;
    u128 => 6, u16;
    u128 => 7, u16;
    u128 => 8, u16;
    u128 => 9, u8;
    u128 => 10, u8;
    u128 => 11, u8;
    u128 => 12, u8;
    u128 => 13, u8;
    u128 => 14, u8;
    u128 => 15, u8;
    u128 => 16, u8
}

mod private {
    pub trait Sealed {}

    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deinterleave_dim2_u8() {
        let x = <u16 as Deinterleave<2>>::deinterleave(2u16, 0);
        let y = <u16 as Deinterleave<2>>::deinterleave(2u16, 1);

        assert_eq!(x, 0);
        assert_eq!(y, 1);
    }

    #[test]
    fn deinterleave_dim3_u8() {
        let x = <u32 as Deinterleave<3>>::deinterleave(2u32, 0);
        let y = <u32 as Deinterleave<3>>::deinterleave(2u32, 1);
        let z = <u32 as Deinterleave<3>>::deinterleave(2u32, 2);

        assert_eq!(x, 0);
        assert_eq!(y, 1);
        assert_eq!(z, 0);
    }

    #[test]
    fn deinterleave_dim4_u8() {
        let x = <u32 as Deinterleave<4>>::deinterleave(2u32, 0);
        let y = <u32 as Deinterleave<4>>::deinterleave(2u32, 1);
        let z = <u32 as Deinterleave<4>>::deinterleave(2u32, 2);
        let w = <u32 as Deinterleave<4>>::deinterleave(2u32, 3);

        assert_eq!(x, 0);
        assert_eq!(y, 1);
        assert_eq!(z, 0);
        assert_eq!(w, 0);
    }

    #[test]
    fn deinterleave_dim2_u64() {
        let x =
            <u128 as Deinterleave<2>>::deinterleave(0x5555_5555_5555_5555_5555_5555_5555_5555, 0);
        let y =
            <u128 as Deinterleave<2>>::deinterleave(0x5555_5555_5555_5555_5555_5555_5555_5555, 1);

        assert_eq!(x, u64::MAX);
        assert_eq!(y, 0);
    }

    #[test]
    fn odd_large_dimension_interleave() {
        let idx =
            <u128 as Deinterleave<13>>::deinterleave(0x0000_0000_0800_4002_0010_0080_0400_2001, 0);
        assert_eq!(idx, u8::MAX);
    }
}
