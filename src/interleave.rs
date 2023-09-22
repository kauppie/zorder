use num_traits::PrimInt;

use crate::mask::{interleave_mask, interleave_shift, num_cast, BitCount};

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
    T: InterleaveOutput<N> + BitCount + PrimInt,
{
    type Output = <Self as InterleaveOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x: Self::Output = unsafe { num_cast(self) };

        for i in (0..<Self as BitCount>::BITS_ILOG2).rev() {
            let mask = interleave_mask(N as u32, 1 << i);
            let shift_count = interleave_shift(i, N as u32);

            x = (x | x.unsigned_shl(shift_count)) & mask;
        }

        x
    }
}

/// Used to determine the minimum width output type which
/// fits the given input type `N` (dimensions) number of times.
pub trait InterleaveOutput<const N: usize>: private::Sealed {
    type Output: PrimInt + BitCount;
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
