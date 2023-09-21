use num_traits::PrimInt;

use crate::mask::{mask, num_cast, BitCount};

/// Interleaves the bits of the given number, while taking output dimension
/// into account.
///
/// Naive implementation of this algorithm would be O(n) where n is the
/// number of bits in the number. Implementations seen here are O(log n).
/// They are extrapolated and generalized from the algorithm described here:
/// http://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN.
pub trait Interleave<const N: usize>: private::Sealed {
    type Output: PrimInt;

    // NOTE: This is a workaround to not need type conversions in runtime code.
    const N_U32: u32 = N as u32;

    fn interleave(self) -> Self::Output;
}

impl<T, const N: usize> Interleave<N> for T
where
    T: InterleaveOutput<N> + BitCount + PrimInt,
{
    type Output = <T as InterleaveOutput<N>>::Output;

    #[inline(always)]
    fn interleave(self) -> Self::Output {
        // SAFETY: casts between unsigned integers always succeed.
        let mut x: Self::Output = unsafe { num_cast(self) };

        for i in (0..<Self as BitCount>::BITS_ILOG2).rev() {
            let mask = interleave_mask(Self::N_U32, 1 << i);
            let shift_count = interleave_shift(i, Self::N_U32);

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
  ($($impl_type:ty, $dim:expr => $out_type:ty);*) => {
      $(
          impl InterleaveOutput<$dim> for $impl_type {
              type Output = $out_type;
          }
      )*
  };
}

impl_interleave_output! {
  u8, 2 => u16;
  u8, 3 => u32;
  u8, 4 => u32;
  u8, 5 => u64;
  u8, 6 => u64;
  u8, 7 => u64;
  u8, 8 => u64;
  u8, 9 => u128;
  u8, 10 => u128;
  u8, 11 => u128;
  u8, 12 => u128;
  u8, 13 => u128;
  u8, 14 => u128;
  u8, 15 => u128;
  u8, 16 => u128;
  u16, 2 => u32;
  u16, 3 => u64;
  u16, 4 => u64;
  u16, 5 => u128;
  u16, 6 => u128;
  u16, 7 => u128;
  u16, 8 => u128;
  u32, 2 => u64;
  u32, 3 => u128;
  u32, 4 => u128;
  u64, 2 => u128
}

/// Calculates the shift amount for the given interleave step and dimension.
#[inline(always)]
const fn interleave_shift(i: u32, n: u32) -> u32 {
    (1 << i) * (n - 1)
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

#[cfg(test)]
mod tests {
    use super::*;

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
