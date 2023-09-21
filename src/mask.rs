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

/// Casts numeric types without checking for success.
#[inline(always)]
pub(crate) unsafe fn num_cast<I, O>(input: I) -> O
where
    I: num_traits::ToPrimitive,
    O: num_traits::NumCast,
{
    <O as num_traits::NumCast>::from(input).unwrap_unchecked()
}

/// # Panics
///
/// If any of the following conditions are met:
///
/// - `bits` == 0
/// - `bits` > `T::BITS`.
#[inline(always)]
pub(crate) fn mask<T: num_traits::PrimInt + BitCount>(bits: u32) -> T {
    <T as num_traits::Bounded>::max_value().unsigned_shr(<T as BitCount>::BITS - bits)
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
    fn ilog2() {
        assert_eq!(u8::BITS_ILOG2, 3);
        assert_eq!(u16::BITS_ILOG2, 4);
        assert_eq!(u32::BITS_ILOG2, 5);
        assert_eq!(u64::BITS_ILOG2, 6);
        assert_eq!(u128::BITS_ILOG2, 7);
    }
}
