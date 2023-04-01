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
//! supported by modern x86 CPUs.
//!
//!
//! # Examples
//!
//! Basic usage with software implementation:
//!
//! ```
//! use zorder::{index_of, coord_of};
//!
//! let idx = index_of((1, 1));
//! assert_eq!(idx, 3);
//!
//! let coord = coord_of(idx);
//! assert_eq!(coord, (1, 1));
//! ```
//!
//! Basic usage with bmi2 implementation:
//!
//! ```
//! use zorder::bmi2::{index_of, coord_of};
//!
//! if is_x86_feature_detected!("bmi2") {
//!     let idx = unsafe { index_of((1, 1)) };
//!     assert_eq!(idx, 3);
//!
//!     let coord = unsafe { coord_of(idx) };
//!     assert_eq!(coord, (1, 1));
//! }
//! ```

#![no_std]

/// Returns the Z-order curve index of the given 2D coordinates.
///
/// # Examples
///
/// ```
/// use zorder::index_of;
///
/// let idx = index_of((1, 1));
/// assert_eq!(idx, 3);
/// ```
#[inline]
pub fn index_of((x, y): (u16, u16)) -> u32 {
    let x_mask = interleave(x);
    let y_mask = interleave(y) << 1;
    x_mask | y_mask
}

/// Returns the 2D coordinates of the given Z-order curve index.
///
/// # Examples
///
/// ```
/// use zorder::coord_of;
///
/// let coord = coord_of(3);
/// assert_eq!(coord, (1, 1));
/// ```
#[inline]
pub fn coord_of(idx: u32) -> (u16, u16) {
    let x = deinterleave(idx & 0x55555555);
    let y = deinterleave((idx & 0xAAAAAAAA) >> 1);
    (x, y)
}

#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), feature = "bmi2"))]
pub mod bmi2 {
    /// Returns the Z-order curve index of the given 2D coordinates.
    ///
    /// This function requires the bmi2 instruction set, but is much faster
    /// than the software implementation.
    ///
    /// # Safety
    ///
    /// This function is safe to call only if the `bmi2` x86 feature is
    /// supported by the CPU. This can be checked at runtime:
    ///
    /// ```
    /// if is_x86_feature_detected!("bmi2") {
    ///    // ...
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zorder::bmi2::index_of;
    ///
    /// if is_x86_feature_detected!("bmi2") {
    ///     let idx = unsafe { index_of((1, 1)) };
    ///     assert_eq!(idx, 3);
    /// }
    /// ```
    #[inline]
    pub unsafe fn index_of((x, y): (u16, u16)) -> u32 {
        use core::arch::x86_64::_pdep_u32;

        let x = _pdep_u32(x as u32, 0x55555555);
        let y = _pdep_u32(y as u32, 0xAAAAAAAA);
        x | y
    }

    /// Returns the 2D coordinates of the given Z-order curve index.
    ///
    /// This function requires the bmi2 instruction set, but is much faster
    /// than the software implementation.
    ///
    /// # Safety
    ///
    /// This function is safe to call only if the `bmi2` x86 feature is
    /// supported by the CPU. This can be checked at runtime:
    ///
    /// ```
    /// if is_x86_feature_detected!("bmi2") {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use zorder::bmi2::coord_of;
    ///
    /// if is_x86_feature_detected!("bmi2") {
    ///     let coord = unsafe { coord_of(3) };
    ///     assert_eq!(coord, (1, 1));
    /// }
    /// ```
    #[inline]
    pub unsafe fn coord_of(idx: u32) -> (u16, u16) {
        use core::arch::x86_64::_pext_u32;

        let x = _pext_u32(idx, 0x55555555);
        let y = _pext_u32(idx, 0xAAAAAAAA);
        (x as u16, y as u16)
    }
}

/// Software implementation of interleaving bits.
#[inline]
fn interleave(num: u16) -> u32 {
    // http://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN

    let num = num as u32;
    let first = (num | (num << 8)) & 0x00FF00FF;
    let second = (first | (first << 4)) & 0x0F0F0F0F;
    let third = (second | (second << 2)) & 0x33333333;
    (third | (third << 1)) & 0x55555555
}

/// Software implementation of deinterleaving bits.
/// `num` has to be interleaved, otherwise may return unexpected results.
#[inline]
fn deinterleave(num: u32) -> u16 {
    // https://stackoverflow.com/questions/4909263/how-to-efficiently-de-interleave-bits-inverse-morton

    let first = (num | (num >> 1)) & 0x33333333;
    let second = (first | (first >> 2)) & 0x0F0F0F0F;
    let third = (second | (second >> 4)) & 0x00FF00FF;
    (third | (third >> 8)) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_of_and_coord_of() {
        for i in 0..10_000 {
            let xy = coord_of(i);
            assert_eq!(index_of(xy), i, "i = {}", i);
        }
    }
}
