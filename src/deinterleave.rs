use num_traits::PrimInt;

use crate::mask::{interleave_mask, interleave_shift, num_cast, BitCount};

pub trait Deinterleave<const N: usize> {
    type Output;

    // NOTE: This is a workaround to not need type conversions in runtime code.
    const N_U32: u32 = N as u32;

    fn deinterleave(self, shift: usize) -> Self::Output;
}

impl<T, const N: usize> Deinterleave<N> for T
where
    T: DeinterleaveOutput<N> + PrimInt + BitCount,
{
    type Output = <Self as DeinterleaveOutput<N>>::Output;

    fn deinterleave(self, shift: usize) -> <Self as Deinterleave<N>>::Output {
        let mut x = (self >> shift) & interleave_mask(Self::N_U32, 1);

        for i in 0..<Self::Output as BitCount>::BITS_ILOG2 {
            let mask = interleave_mask(Self::N_U32, 1 << (i + 1));
            let shift_count = interleave_shift(i, Self::N_U32);

            x = (x | x.unsigned_shr(shift_count)) & mask;
        }

        // SAFETY: casts between unsigned integers always succeed.
        unsafe { num_cast(x) }
    }
}

pub trait DeinterleaveOutput<const N: usize> {
    type Output: PrimInt + BitCount;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deinterleave() {
        let x = 64u16.deinterleave(0);
        let y = 64u16.deinterleave(1);

        assert_eq!(x, 8);
        assert_eq!(y, 0);
    }
}
