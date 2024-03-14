/// Used to determine the number of bits in the given type.
pub trait BitCount {
    /// The number of bits in the type.
    const BITS: u32;

    /// The base-2 logarithm of the number of bits in the type.
    // HACK: only works for powers of 2.
    const BITS_ILOG2: u32 = Self::BITS.trailing_zeros();
}

/// Used to implement [`BitCount`] for many types.
macro_rules! impl_bit_count {
    ($($t:ty),*) => {
        $(
            impl BitCount for $t {
                const BITS: u32 = Self::BITS;
            }
        )*
    };
}

// Implement [`BitCount`] for all unsigned integer types.
impl_bit_count! {
    u8, u16, u32, u64, u128
}

/// Calculates the shift amount for the given interleave step and dimension.
#[inline]
pub(crate) const fn interleave_shift(dim: u32, i: u32) -> u32 {
    (dim - 1) << i
}

/// Calculates the mask for one step in interleaving and deinterleaving bits of a number.
///
/// `bits` determines the number of consecutive set bits, before consecutive unset bits.
/// `dim` determines the number of times the pattern of unset bits repeats.
///
/// The least significant bit is always set.
///
/// # Panics
///
/// Panic behavior is the same as [`bit_mask`] for the `bits` parameter.
#[inline]
pub(crate) fn interleave_mask<T: num_traits::PrimInt + BitCount>(dim: u32, bits: u32) -> T {
    let mut acc = <T as num_traits::Zero>::zero();
    let mask = bit_mask::<T>(bits);

    let ceil_div_dim = <T as BitCount>::BITS.div_ceil(dim);
    let ceil_div_bits = ceil_div_dim.div_ceil(bits);

    for i in 0..ceil_div_bits {
        acc = acc | mask.unsigned_shl(i * dim * bits);
    }

    acc
}

/// Set the `n` least significant bits of given type.
///
/// # Panics
///
/// If any of the following conditions are met:
///
/// - `bits` == 0
/// - `bits` > `T::BITS`.
#[inline]
pub(crate) fn bit_mask<T: num_traits::PrimInt + BitCount>(bits: u32) -> T {
    <T as num_traits::Bounded>::max_value().unsigned_shr(<T as BitCount>::BITS - bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_masks() {
        assert_eq!(bit_mask::<u8>(4), 0xF);
        assert_eq!(bit_mask::<u16>(4), 0xF);
        assert_eq!(bit_mask::<u32>(4), 0xF);
        assert_eq!(bit_mask::<u64>(4), 0xF);
        assert_eq!(bit_mask::<u128>(4), 0xF);

        assert_eq!(bit_mask::<u8>(8), 0xFF);
        assert_eq!(bit_mask::<u16>(16), 0xFFFF);
        assert_eq!(bit_mask::<u32>(32), 0xFFFF_FFFF);
        assert_eq!(bit_mask::<u64>(64), 0xFFFF_FFFF_FFFF_FFFF);
        assert_eq!(
            bit_mask::<u128>(128),
            0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF
        );
    }

    #[test]
    #[should_panic]
    fn zero_mask() {
        bit_mask::<u8>(0);
    }

    #[test]
    #[should_panic]
    fn too_large_mask() {
        bit_mask::<u8>(u8::BITS + 1);
    }

    #[test]
    fn ilog2() {
        assert_eq!(u8::BITS_ILOG2, 3);
        assert_eq!(u16::BITS_ILOG2, 4);
        assert_eq!(u32::BITS_ILOG2, 5);
        assert_eq!(u64::BITS_ILOG2, 6);
        assert_eq!(u128::BITS_ILOG2, 7);
    }

    #[test]
    fn interleave_mask_dim2() {
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
    fn interleave_mask_dim3() {
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
    fn interleave_mask_dim4() {
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
    fn odd_large_dimension_mask() {
        assert_eq!(
            interleave_mask::<u32>(13, 1),
            0b1_0000_0000_0000_1_0000_0000_0000_1
        )
    }
}
