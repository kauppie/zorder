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
//! [^1]: Maximum number of dimensions is limited by the largest unsigned integer type, `u128`, which is able to store 16 8-bit coordinates. `bmi2` based approach is limited to `u64`.
//!
//! # Examples
//!
//! Basic usage with software implementation:
//!
//! ```
//! use zorder::{coord_of, index_of};
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
//! use zorder::bmi2::{coord_of, coord_of_unchecked, HardwareSupportToken, index_of, index_of_unchecked};
//!
//! // Safe interface with hardware support token.
//! let support_token = HardwareSupportToken::new();
//! if let Some(support_token) = support_token {
//!     let idx = index_of([1u16, 1u16], support_token);
//!     assert_eq!(idx, 3u32);
//!
//!     let coord = coord_of(idx, support_token);
//!     assert_eq!(coord, [1u16, 1u16]);
//! }
//!
//! // Unsafe interface with hardware support check.
//! // Only works on x86_64 CPUs.
//! if zorder::bmi2::has_hardware_support() {
//!     let idx = unsafe { index_of_unchecked([1u16, 1u16]) };
//!     assert_eq!(idx, 3u32);
//!
//!     let coord = unsafe { coord_of_unchecked(idx) };
//!     assert_eq!(coord, [1u16, 1u16]);
//! }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod deinterleave;
mod interleave;
mod mask;

pub use deinterleave::Deinterleave;
pub use interleave::Interleave;

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

/// `bmi2` module provides Z-order curve index and coordinate calculations
/// using the bmi2 instruction set.
///
/// This module provides unsafe `bmi2` accelerated functions and safe wrappers
/// around them. Safety of calling [`index_of_unchecked`](crate::bmi2::index_of_unchecked)
/// and [`coord_of_unchecked`](crate::bmi2::coord_of_unchecked) can be validated at
/// runtime using [`has_hardware_support`](crate::bmi2::has_hardware_support). Unchecked
/// functions are only available on x86_64 CPUs.
/// Optionally, you can acquire a [`HardwareSupportToken`](crate::bmi2::HardwareSupportToken), and then call
/// [`index_of`](crate::bmi2::index_of) and [`coord_of`](crate::bmi2::coord_of) without unsafe.
///
/// Note that the bmi2 instruction set is supported only on modern x86_64 CPUs.
///
/// # Examples
///
/// ```
/// # use zorder::bmi2;
/// if bmi2::has_hardware_support() {
///     // Only works on x86_64 CPUs.
///     let idx = unsafe { bmi2::index_of_unchecked([3u32, 7u32]) };
///     assert_eq!(idx, 0b101_111u64);
/// }
///
/// let support_token = bmi2::HardwareSupportToken::new();
/// if let Some(support_token) = support_token {
///     let idx = bmi2::index_of([3u32, 7u32], support_token);
///     assert_eq!(idx, 0b101_111u64);
/// }
/// ```
pub mod bmi2 {
    pub use crate::{deinterleave::DeinterleaveBMI2, interleave::InterleaveBMI2};

    use crate::{Deinterleave, Interleave};

    /// Returns true if the CPU supports the bmi2 instruction set.
    ///
    /// You can use this function to validate that [`index_of_unchecked`] and
    /// [`coord_of_unchecked`] can be safely called.
    /// Optionally, you can acquire a [`HardwareSupportToken`] to ensure that
    /// the CPU supports the bmi2 instruction set at runtime, and then call
    /// [`index_of`] and [`coord_of`] without unsafe.
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

    /// A token that guarantees that the CPU supports the bmi2 instruction set.
    ///
    /// You can freely copy and move this token, but you cannot create an instance
    /// directly. Instead, [`HardwareSupportToken::new`] returns an instance if the
    /// CPU supports the bmi2 instruction set.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct HardwareSupportToken {
        _private: (),
    }

    impl HardwareSupportToken {
        /// Returns a new instance if the CPU supports the bmi2 instruction set.
        pub fn new() -> Option<Self> {
            has_hardware_support().then(|| Self { _private: () })
        }
    }

    /// Safe wrapper around [`index_of_unchecked`] that requires a
    /// [`HardwareSupportToken`] to guarantee that the bmi2 instruction set is
    /// supported by the CPU.
    #[inline]
    pub fn index_of<I, const N: usize>(
        array: [I; N],
        _support_token: HardwareSupportToken,
    ) -> <I as Interleave<N>>::Output
    where
        I: InterleaveBMI2<N>,
    {
        // SAFETY: Having an instance of `HardwareSupportToken` guarantees that
        // the `bmi2` instruction set is supported by the CPU and that it is safe
        // to call `index_of_unchecked`.
        #[cfg(target_arch = "x86_64")]
        unsafe {
            index_of_unchecked(array)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = array;
            unreachable!("HardwareSupportToken cannot be created on non-x86_64 platforms")
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
    /// if zorder::bmi2::has_hardware_support() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// if zorder::bmi2::has_hardware_support() {
    ///     // Only works on x86_64 CPUs.
    ///     let idx = unsafe { index_of_unchecked([3u32, 7u32]) };
    ///     assert_eq!(idx, 0b101_111u64);
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn index_of_unchecked<I, const N: usize>(
        array: [I; N],
    ) -> <I as Interleave<N>>::Output
    where
        I: InterleaveBMI2<N>,
    {
        crate::util::generic_index_of(array, |idx| idx.interleave_bmi2())
    }

    /// Safe wrapper around [`coord_of_unchecked`] that requires a
    /// [`HardwareSupportToken`] to guarantee that the bmi2 instruction set is
    /// supported by the CPU.
    #[inline]
    pub fn coord_of<I, const N: usize>(
        index: I,
        _support_token: HardwareSupportToken,
    ) -> [<I as Deinterleave<N>>::Output; N]
    where
        I: DeinterleaveBMI2<N> + Copy,
    {
        // SAFETY: Having an instance of `HardwareSupportToken` guarantees that
        // the `bmi2` instruction set is supported by the CPU and that it is safe
        // to call `coord_of_unchecked`.
        #[cfg(target_arch = "x86_64")]
        unsafe {
            coord_of_unchecked(index)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = index;
            unreachable!("HardwareSupportToken cannot be created on non-x86_64 platforms")
        }
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
    /// # use zorder::bmi2;
    /// if bmi2::has_hardware_support() {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use zorder::bmi2;
    /// if bmi2::has_hardware_support() {
    ///     // Only works on x86_64 CPUs.
    ///     let coord = unsafe { coord_of_unchecked(0b101_111u64) };
    ///     assert_eq!(coord, [3u32, 7u32]);
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn coord_of_unchecked<I, const N: usize>(
        index: I,
    ) -> [<I as Deinterleave<N>>::Output; N]
    where
        I: DeinterleaveBMI2<N> + Copy,
    {
        crate::util::generic_coord_of(index, |idx, i| idx.deinterleave_bmi2(i))
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
    fn index_and_back_16() {
        for i in 0..10_000u16 {
            let array: [_; 2] = coord_of(i);
            assert_eq!(index_of(array), i);
        }
    }

    #[test]
    fn index_and_back_32() {
        for i in 0..10_000u32 {
            let array: [_; 2] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u32 {
            let array: [_; 4] = coord_of(i);
            assert_eq!(index_of(array), i);
        }
    }

    #[test]
    fn index_and_back_64() {
        for i in 0..10_000u64 {
            let array: [_; 2] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u64 {
            let array: [_; 3] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u64 {
            let array: [_; 5] = coord_of(i);
            assert_eq!(index_of(array), i);
        }
    }

    #[test]
    fn index_and_back_128() {
        for i in 0..10_000u128 {
            let array: [_; 2] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u128 {
            let array: [_; 3] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u128 {
            let array: [_; 5] = coord_of(i);
            assert_eq!(index_of(array), i);
        }

        for i in 0..10_000u128 {
            let array: [_; 9] = coord_of(i);
            assert_eq!(index_of(array), i);
        }
    }
}
