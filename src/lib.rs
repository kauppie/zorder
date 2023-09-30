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
//!         let idx = unsafe { index_of((1, 1)) };
//!         assert_eq!(idx, 3);
//!
//!         let coord = unsafe { coord_of(idx) };
//!         assert_eq!(coord, (1, 1));
//!     }
//! }
//! ```
//!
//! There exists also functions for wider 64-bit indices with '_64' postfix:
//!
//! ```
//! use zorder::{index_of, coord_of};
//!
//! let idx = index_of([1u32, 1u32]);
//! assert_eq!(idx, 3u64);
//!
//! let coord = coord_of(idx);
//! assert_eq!(coord, [1u32, 1u32]);
//!
//! #[cfg(target_arch = "x86_64")]
//! {
//!     use zorder::bmi2::{index_of_64, coord_of_64};
//!
//!     if is_x86_feature_detected!("bmi2") {
//!         let idx = unsafe { index_of_64((1, 1)) };
//!         assert_eq!(idx, 3);
//!
//!         let coord = unsafe { coord_of_64(idx) };
//!         assert_eq!(coord, (1, 1));
//!     }
//! }
//! ```

#![no_std]

mod deinterleave;
mod interleave;
mod mask;

use num_traits::Zero;

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
///
/// let idx = index_of([3u32, 7u32]);
/// assert_eq!(idx, 0b101_111u64);
/// ```
#[inline]
pub fn index_of<I, const N: usize>(array: [I; N]) -> <I as Interleave<N>>::Output
where
    I: Interleave<N>,
{
    array
        .into_iter()
        .map(Interleave::interleave)
        .enumerate()
        .fold(<I as Interleave<N>>::Output::zero(), |acc, (i, n)| {
            acc | (n << i)
        })
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
///
/// let coord = coord_of(0b101_111u64);
/// assert_eq!(coord, [3u32, 7u32]);
/// ```
#[inline]
pub fn coord_of<I, const N: usize>(index: I) -> [<I as Deinterleave<N>>::Output; N]
where
    I: Deinterleave<N> + Copy,
{
    core::array::from_fn(|i| index.deinterleave(i))
}

#[cfg(target_arch = "x86_64")]
pub mod bmi2 {
    /// Returns the Z-order curve index of the given 16-bit 2D coordinates.
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
    /// use zorder::bmi2::index_of;
    ///
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let idx = unsafe { index_of((1, 1)) };
    ///         assert_eq!(idx, 3);
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn index_of((x, y): (u16, u16)) -> u32 {
        use core::arch::x86_64::_pdep_u32;

        let x = _pdep_u32(x as u32, 0x55555555);
        let y = _pdep_u32(y as u32, 0xAAAAAAAA);
        x | y
    }

    /// Returns the Z-order curve index of the given 32-bit 2D coordinates.
    ///
    /// This function operates on wider indices than [`bmi2::index_of`](crate::bmi2::index_of).
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
    /// use zorder::bmi2::index_of_64;
    ///
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let idx = unsafe { index_of_64((1, 1)) };
    ///         assert_eq!(idx, 3);
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn index_of_64((x, y): (u32, u32)) -> u64 {
        use core::arch::x86_64::_pdep_u64;

        let x = _pdep_u64(x as u64, 0x5555555555555555);
        let y = _pdep_u64(y as u64, 0xAAAAAAAAAAAAAAAA);
        x | y
    }

    /// Returns the 2D coordinates of the given 32-bit Z-order curve index.
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
    /// use zorder::bmi2::coord_of;
    ///
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let coord = unsafe { coord_of(3) };
    ///         assert_eq!(coord, (1, 1));
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn coord_of(idx: u32) -> (u16, u16) {
        use core::arch::x86_64::_pext_u32;

        let x = _pext_u32(idx, 0x55555555);
        let y = _pext_u32(idx, 0xAAAAAAAA);
        (x as u16, y as u16)
    }

    /// Returns the 2D coordinates of the given 64-bit Z-order curve index.
    ///
    /// This function operates on wider indices than [`bmi2::coord_of`](crate::bmi2::coord_of).
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
    /// use zorder::bmi2::coord_of_64;
    ///
    /// #[cfg(target_arch = "x86_64")]
    /// {
    ///     if is_x86_feature_detected!("bmi2") {
    ///         let coord = unsafe { coord_of_64(3) };
    ///         assert_eq!(coord, (1, 1));
    ///     }
    /// }
    /// ```
    #[inline]
    #[target_feature(enable = "bmi2")]
    pub unsafe fn coord_of_64(idx: u64) -> (u32, u32) {
        use core::arch::x86_64::_pext_u64;

        let x = _pext_u64(idx, 0x5555555555555555);
        let y = _pext_u64(idx, 0xAAAAAAAAAAAAAAAA);
        (x as u32, y as u32)
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
