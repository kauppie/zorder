use num_traits::{cast::AsPrimitive, PrimInt};

use crate::mask::{interleave_mask, interleave_shift, BitCount};

/// Deinterleave a single number from a set of interleaved numbers. Inverse of
/// [`Interleave`](crate::interleave::Interleave).
pub trait Deinterleave<const N: usize>: private::Sealed {
    /// Smallest unsigned integer type that can hold the deinterleaved bits.
    type Output;

    /// Deinterleaves a number from a set of interleaved numbers starting from
    /// the given least significant bit (`lsb`) index.
    ///
    /// Dimension `N` determines which bits are extracted to form the output number.
    fn deinterleave(self, lsb: usize) -> Self::Output;
}

impl<T, const N: usize> Deinterleave<N> for T
where
    T: DeinterleaveOutput<N>,
    T: AsPrimitive<<Self as DeinterleaveOutput<N>>::Output>,
    T: BitCount + PrimInt,
{
    type Output = <Self as DeinterleaveOutput<N>>::Output;

    #[inline]
    fn deinterleave(self, lsb: usize) -> <Self as Deinterleave<N>>::Output {
        let mut x = (self >> lsb) & interleave_mask(N as u32, 1);

        for i in 0..<Self::Output as BitCount>::BITS_ILOG2 {
            let mask = interleave_mask(N as u32, 1 << (i + 1));
            let shift_count = interleave_shift(N as u32, i);

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

/// Deinterleave a single number from a set of interleaved numbers using BMI2
/// instruction set. Inverse of [`InterleaveBMI2`](crate::interleave::InterleaveBMI2).
pub trait DeinterleaveBMI2<const N: usize>: Deinterleave<N> {
    /// Deinterleave a single number from a set of interleaved numbers using BMI2
    /// instruction set. Inverse of [`InterleaveBMI2`](crate::interleave::InterleaveBMI2).
    ///
    /// # Safety
    ///
    /// This function is safe to call only if the `bmi2` x86_64 feature is
    /// supported by the CPU.
    unsafe fn deinterleave_bmi2(self, lsb: usize) -> <Self as Deinterleave<N>>::Output;
}

macro_rules! impl_deinterleave_bmi2_32 {
    ($($impl_type:ty => $dim:expr);*) => {
        $(
            impl DeinterleaveBMI2<$dim> for $impl_type {
                #[inline]
                unsafe fn deinterleave_bmi2(self, lsb: usize) -> <Self as Deinterleave<$dim>>::Output {
                    #[cfg(target_arch = "x86_64")]
                    {
                        let mask = interleave_mask::<u32>($dim, 1) << lsb;
                        unsafe {
                            core::arch::x86_64::_pext_u32(self.as_(), mask).as_()
                        }
                    }
                    #[cfg(not(target_arch = "x86_64"))]
                    {
                        let _ = lsb;
                        panic!("BMI2 feature is not supported on this architecture")
                    }
                }
            }
        )*
    };
}

macro_rules! impl_deinterleave_bmi2_64 {
    ($($impl_type:ty => $dim:expr);*) => {
        $(
            impl DeinterleaveBMI2<$dim> for $impl_type {
                #[inline]
                unsafe fn deinterleave_bmi2(self, lsb: usize) -> <Self as Deinterleave<$dim>>::Output {
                    #[cfg(target_arch = "x86_64")]
                    {
                        let mask = interleave_mask::<u64>($dim, 1) << lsb;
                        unsafe {
                            core::arch::x86_64::_pext_u64(self.as_(), mask).as_()
                        }
                    }
                    #[cfg(not(target_arch = "x86_64"))]
                    {
                        let _ = lsb;
                        panic!("BMI2 feature is not supported on this architecture")
                    }
                }
            }
        )*
    };
}

impl_deinterleave_bmi2_32! {
    u16 => 2;
    u32 => 2;
    u32 => 3;
    u32 => 4
}

impl_deinterleave_bmi2_64! {
    u64 => 2;
    u64 => 3;
    u64 => 4;
    u64 => 5;
    u64 => 6;
    u64 => 7;
    u64 => 8
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
