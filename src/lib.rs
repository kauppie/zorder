//! # zorder
//!
//! This crate provides functions to convert between 2D coordinates and a Z-order curve index.
//! The Z-order curve is a space-filling curve that maps 2D coordinates to a 1D index.
//! The Z-order curve is also known as Morton code. Advantage of the Z-order curve is that
//! is may be used to efficiently store 2D data in a 1D array as it preserves locality, and
//! is therefore cache-friendly.
//!
//! This crate provides two implementations of the Z-order curve, one using a software
//! implementation supported by all platforms and one using bmi2 instructions
//! supported by modern x86_64 CPUs.
//!
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
//! #[cfg(target_arch = "x86_64")]
//! {
//!     use zorder::bmi2::{index_of, coord_of};
//!
//!     if is_x86_feature_detected!("bmi2") {
//!         let idx = unsafe { index_of([1u16, 1u16]) };
//!         assert_eq!(idx, 3);
//!
//!         let coord = unsafe { coord_of(idx) };
//!         assert_eq!(coord, (1, 1));
//!     }
//! }
//! ```

#![no_std]

mod deinterleave;
mod interleave;
mod mask;

pub use deinterleave::Deinterleave;
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

#[cfg(target_arch = "x86_64")]
pub mod bmi2 {
    use crate::{
        deinterleave::DeinterleaveBMI2, interleave::InterleaveBMI2, util, Deinterleave, Interleave,
    };

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
