//! # zorder
//!
//! This crate provides functions to convert N-dimensional[^1] coordinates to
//! [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve) indexes and back.
//! Z-order curve, also known as Morton code, is a mapping of N-dimensional coordinates
//! to 1D index which preverses locality.
//! It is cache-efficient way of storing N-dimensional data in 1D array.
//!
//! This crate provides two implementations of the Z-order curve, one using a software
//! implementation supported by all platforms and one using bmi2 instructions
//! supported by modern x86_64 CPUs.
//!
//! # Examples
//!
//! Basic usage with software implementation:
//!
//! ```
//! use zorder::{index_of, coord_of};
//!
//! let idx = index_of([1u16, 1u16]);
//! assert_eq!(idx, 3u32);
//!
//! let coord = coord_of(idx);
//! assert_eq!(coord, [1u16, 1u16]);
//! ```
//!
//! Basic usage with bmi2 implementation:
//!
//! ```
//! use zorder::bmi2::{coord_of, index_of};
//!
//! if zorder::bmi2::has_hardware_support() {
//!     let idx = unsafe { index_of([1u16, 1u16]) };
//!     assert_eq!(idx, 3u32);
//!
//!     let coord = unsafe { coord_of(idx) };
//!     assert_eq!(coord, [1u16, 1u16]);
//! }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod deinterleave;
mod interleave;
mod mask;

pub use deinterleave::{Deinterleave, DeinterleaveBMI2};
pub use interleave::{Interleave, InterleaveBMI2};

/// Calculates Z-order curve index for given sequence of coordinates.
///
/// Output type will be the smallest unsigned integer type that can hold all
/// of the given coordinates.
///
/// # Examples
///
/// ```
/// # use zorder::index_of;
/// let idx = index_of([3u32, 7u32]);
/// assert_eq!(idx, 0b101_111u64);
/// ```
#[inline]
pub fn index_of<I, const N: usize>(array: [I; N]) -> <I as Interleave<N>>::Output
where
    I: Interleave<N>,
{
    util::generic_index_of(array, Interleave::interleave)
}

/// Returns the 2D coordinates of the given Z-order curve index.
///
/// Since many different 2D coordinates can be mapped to the same type `I`,
/// you may need to specify the number of dimensions `N` to disambiguate.
///
/// # Examples
///
/// ```
/// # use zorder::coord_of;
/// let coord = coord_of(0b101_111u64);
/// assert_eq!(coord, [3u32, 7u32]);
/// ```
#[inline]
pub fn coord_of<I, const N: usize>(index: I) -> [<I as Deinterleave<N>>::Output; N]
where
    I: Deinterleave<N> + Copy,
{
    util::generic_coord_of(index, Deinterleave::deinterleave)
}

pub mod bmi2 {
    use crate::{
        deinterleave::DeinterleaveBMI2, interleave::InterleaveBMI2, util, Deinterleave, Interleave,
    };

    /// Returns true if the CPU supports the bmi2 instruction set.
    ///
    /// You can use this function to validate that [`zorder::bmi2::index_of`] and
    /// [`zorder::bmi2::coord_of`] can be safely called.
    pub fn has_hardware_support() -> bool {
        #[cfg(all(target_arch = "x86_64", feature = "std"))]
        {
            std::is_x86_feature_detected!("bmi2")
        }
        #[cfg(not(all(target_arch = "x86_64", feature = "std")))]
        {
            false
        }
    }

    /// Calculates Z-order curve index for given sequence of coordinates.
    ///
    /// Output type will be the smallest unsigned integer type that can hold all
    /// of the given coordinates.
    ///
    /// This function requires the `bmi2` instruction set, but it can be
    /// faster than the software implementation.
    ///
    /// # Safety
    ///
    /// This function is safe to call only if the `bmi2` x86_64 feature is
    /// supported by the CPU. This can be checked at runtime:
    ///
    /// ```
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use zorder::bmi2::index_of;
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let idx = index_of([3u32, 7u32]);
    ///         assert_eq!(idx, 0b101_111u64);
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn index_of<I, const N: usize>(array: [I; N]) -> <I as Interleave<N>>::Output
    where
        I: InterleaveBMI2<N>,
    {
        util::generic_index_of(array, |idx| idx.interleave_bmi2())
    }

    /// Returns the 2D coordinates of the given Z-order curve index.
    ///
    /// Since many different 2D coordinates can be mapped to the same type `I`,
    /// you may need to specify the number of dimensions `N` to disambiguate.
    ///
    /// This function requires the bmi2 instruction set, but it can be
    /// faster than the software implementation.
    ///
    /// # Safety
    ///
    /// This function is safe to call only if the `bmi2` x86_64 feature is
    /// supported by the CPU. This can be checked at runtime:
    ///
    /// ```
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         // ...
    ///     }
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use zorder::bmi2::coord_of;
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let coord = coord_of(0b101_111u64);
    ///         assert_eq!(coord, [3u32, 7u32]);
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn coord_of<I, const N: usize>(index: I) -> [<I as Deinterleave<N>>::Output; N]
    where
        I: DeinterleaveBMI2<N> + Copy,
    {
        util::generic_coord_of(index, |idx, i| idx.deinterleave_bmi2(i))
    }
}

mod util {
    use crate::{Deinterleave, Interleave};
    use num_traits::Zero;

    #[inline]
    pub(super) fn generic_index_of<I, const N: usize>(
        array: [I; N],
        interleave: impl Fn(I) -> <I as Interleave<N>>::Output,
    ) -> <I as Interleave<N>>::Output
    where
        I: Interleave<N>,
    {
        array.into_iter().map(interleave).enumerate().fold(
            <I as Interleave<N>>::Output::zero(),
            |acc, (i, interleaved)| acc | (interleaved << i),
        )
    }

    #[inline]
    pub(super) fn generic_coord_of<I, const N: usize>(
        index: I,
        deinterleave: impl Fn(I, usize) -> <I as Deinterleave<N>>::Output,
    ) -> [<I as Deinterleave<N>>::Output; N]
    where
        I: Deinterleave<N> + Copy,
    {
        core::array::from_fn(|i| deinterleave(index, i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_conversions() {
        for i in 0..10_000u32 {
            let array: [_; 2] = coord_of(i);
            assert_eq!(index_of(array), i);
        }
    }

    #[test]
    fn interleave() {
        let x = index_of([7u32, 7u32]);
        assert_eq!(x, 0b111111);
    }
}
