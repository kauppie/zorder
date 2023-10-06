use num_traits::{cast::AsPrimitive, PrimInt};

use crate::mask::{interleave_mask, interleave_shift, BitCount};

/// Interleaves the bits of the given number, while taking output dimension
/// into account.
///
/// Naive implementation of this algorithm would be O(n) where `n` is the
/// number of bits in the number. Implementations seen here are O(log n).
/// They are extrapolated and generalized from the algorithm described here:
/// http://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN.
pub trait Interleave<const N: usize>: private::Sealed {
    type Output: PrimInt;

    /// Interleaves the bits of the given number.
    ///
    /// Dimension `N` determines the number of unused bits between the
    /// used bits, so that all numbers can be interleaved without
    /// overlapping.
    fn interleave(self) -> Self::Output;
}

impl<T, const N: usize> Interleave<N> for T
where
    T: InterleaveOutput<N>,
    T: AsPrimitive<<Self as InterleaveOutput<N>>::Output>,
    T: BitCount + PrimInt,
{
    type Output = <Self as InterleaveOutput<N>>::Output;

    #[inline]
    fn interleave(self) -> Self::Output {
        let mut x = self.as_();

        for i in (0..<Self as BitCount>::BITS_ILOG2).rev() {
            let mask = interleave_mask(N as u32, 1 << i);
            let shift_count = interleave_shift(N as u32, i);

            x = (x | x.unsigned_shl(shift_count)) & mask;
        }

        x
    }
}

/// Used to determine the minimum width output type which
/// fits the given input type `N` (dimensions) number of times.
pub trait InterleaveOutput<const N: usize>: private::Sealed {
    type Output: BitCount + PrimInt;
}

macro_rules! impl_interleave_output {
    ($($dim:expr, $impl_type:ty => $out_type:ty);*) => {
        $(
            impl InterleaveOutput<$dim> for $impl_type {
                type Output = $out_type;
            }
        )*
    };
}

impl_interleave_output! {
    2, u8 => u16;
    3, u8 => u32;
    4, u8 => u32;
    5, u8 => u64;
    6, u8 => u64;
    7, u8 => u64;
    8, u8 => u64;
    9, u8 => u128;
    10, u8 => u128;
    11, u8 => u128;
    12, u8 => u128;
    13, u8 => u128;
    14, u8 => u128;
    15, u8 => u128;
    16, u8 => u128;
    2, u16 => u32;
    3, u16 => u64;
    4, u16 => u64;
    5, u16 => u128;
    6, u16 => u128;
    7, u16 => u128;
    8, u16 => u128;
    2, u32 => u64;
    3, u32 => u128;
    4, u32 => u128;
    2, u64 => u128
}

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleave_dim2_u8() {
        let idx = <u8 as Interleave<2>>::interleave(u8::MAX);
        assert_eq!(idx, 0b01_01_01_01_01_01_01_01);
    }

    #[test]
    fn interleave_dim3_u8() {
        let idx = <u8 as Interleave<3>>::interleave(u8::MAX);
        assert_eq!(idx, 0b001_001_001_001_001_001_001_001);
    }

    #[test]
    fn interleave_dim4_u8() {
        let idx = <u8 as Interleave<4>>::interleave(u8::MAX);
        assert_eq!(idx, 0b0001_0001_0001_0001_0001_0001_0001_0001);
    }

    #[test]
    fn interleave_dim4_u8_half() {
        let idx = <u8 as Interleave<4>>::interleave(0xF0);
        assert_eq!(idx, 0b0001_0001_0001_0001_0000_0000_0000_0000);
    }

    #[test]
    fn interleave_dim2_u64() {
        let idx = <u64 as Interleave<2>>::interleave(u64::MAX);
        assert_eq!(idx, 0x5555_5555_5555_5555_5555_5555_5555_5555);
    }

    #[test]
    fn odd_large_dimension_interleave() {
        let idx = <u8 as Interleave<13>>::interleave(u8::MAX);
        assert_eq!(idx, 0x0000_0000_0800_4002_0010_0080_0400_2001);
    }
}
