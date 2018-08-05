#![cfg_attr(not(any(std, test)), no_std)]

#![cfg_attr(docs_rs_workarounds, feature(repr_transparent))]

//! Offers a reversed view into a slice.
//!
//! To use, import the `SliceExt` trait to get the `.rev()` and `.rev_mut`
//! extension methods on slices.  Then treat the returned `RevSlice` like
//! you would an ordinary slice: index it, split it, iterate it, whatever.
//!
//! Example:
//!
//! ```
//! extern crate rev_slice;
//! use rev_slice::SliceExt;
//!
//! let r = [1, 2, 4, 9, 16, 25].rev();
//! assert_eq!(r[0], 25);
//! assert_eq!(r[1..3].rev(), &[9, 16]);
//! assert_eq!(r.split_first().unwrap().0, &25);
//!
//! let mut it = r.iter().cloned().skip(2);
//! assert_eq!(it.next(), Some(9));
//! assert_eq!(it.next(), Some(4));
//! assert_eq!(it.next(), Some(2));
//! ```

#[cfg(any(std, test))]
extern crate core;

use core::{iter, slice};
use core::ops::{Index, IndexMut};
use core::ops::Range;

/// Adds `.rev()` and `.rev_mut()` methods to slices.
///
/// There's no reason to implement this yourself.
pub trait SliceExt {
    /// The element type of the slice
    type Element;

    /// Get a proxy providing a reversed view of the slice.
    fn rev(&self) -> &RevSlice<Self::Element>;

    /// Get a proxy providing a mutable reversed view of the mutable slice.
    fn rev_mut(&mut self) -> &mut RevSlice<Self::Element>;

    #[doc(hidden)]
    fn sealed(_: internal::Sealed);
}

mod internal {
    pub struct Sealed;
}

impl<T> SliceExt for [T] {
    type Element = T;
    fn rev(&self) -> &RevSlice<Self::Element> {
        unsafe { core::mem::transmute(self) }
    }
    fn rev_mut(&mut self) -> &mut RevSlice<Self::Element> {
        unsafe { core::mem::transmute(self) }
    }
    fn sealed(_: internal::Sealed) {}
}

/// A DST newtype providing a reversed view of a slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RevSlice<T>([T]);

impl<T> RevSlice<T> {
    /// Provides a reversed view of the reversed slice, aka the original slice.
    pub fn rev(&self) -> &[T] {
        &self.0
    }

    /// Provides a reversed view of the reversed slice, aka the original mutable slice.
    pub fn rev_mut(&mut self) -> &mut [T] {
        &mut self.0
    }

    fn flip_index(&self, index: usize) -> usize {
        self.len() - (index+1)
    }

    fn flip_fencepost(&self, index: usize) -> usize {
        self.len() - index
    }

    fn flip_range(&self, range: Range<usize>) -> Range<usize> {
        self.flip_fencepost(range.end)..self.flip_fencepost(range.start)
    }
}

/// These methods work like their equivalents in `core`.
impl<T> RevSlice<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    pub fn last(&self) -> Option<&T> {
        self.0.first()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.first_mut()
    }

    pub fn split_first(&self) -> Option<(&T, &RevSlice<T>)> {
        let (item, rest) = self.0.split_last()?;
        Some((item, rest.rev()))
    }

    pub fn split_first_mut(&mut self) -> Option<(&T, &RevSlice<T>)> {
        let (item, rest) = self.0.split_last_mut()?;
        Some((item, rest.rev_mut()))
    }

    pub fn split_last(&self) -> Option<(&T, &RevSlice<T>)> {
        let (item, rest) = self.0.split_first()?;
        Some((item, rest.rev()))
    }

    pub fn split_last_mut(&mut self) -> Option<(&T, &RevSlice<T>)> {
        let (item, rest) = self.0.split_first_mut()?;
        Some((item, rest.rev_mut()))
    }

    pub fn split_at(&self, mid: usize) -> (&RevSlice<T>, &RevSlice<T>) {
        let rmid = self.flip_fencepost(mid);
        let (a, b) = self.0.split_at(rmid);
        (b.rev(), a.rev())
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut RevSlice<T>, &mut RevSlice<T>) {
        let rmid = self.flip_fencepost(mid);
        let (a, b) = self.0.split_at_mut(rmid);
        (b.rev_mut(), a.rev_mut())
    }
}

impl<T> Index<usize> for RevSlice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let rindex = self.flip_index(index);
        &self.0[rindex]
    }
}

impl<T> IndexMut<usize> for RevSlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let rindex = self.flip_index(index);
        &mut self.0[rindex]
    }
}

impl<T> Index<Range<usize>> for RevSlice<T> {
    type Output = RevSlice<T>;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        let rindex = self.flip_range(index);
        self.0[rindex].rev()
    }
}

impl<T> IndexMut<Range<usize>> for RevSlice<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        let rindex = self.flip_range(index);
        self.0[rindex].rev_mut()
    }
}

impl<T> RevSlice<T> {
    /// `my_slice.rev().iter()` and `my_slice.iter().rev()` are equivalent.
    pub fn iter(&self) -> iter::Rev<slice::Iter<T>> {
        self.0.iter().rev()
    }

    /// `my_slice.rev().iter_mut()` and `my_slice.iter_mut().rev()` are equivalent.
    pub fn iter_mut(&mut self) -> iter::Rev<slice::IterMut<T>> {
        self.0.iter_mut().rev()
    }
}

impl<'a, T> iter::IntoIterator for &'a RevSlice<T> {
    type Item = &'a T;
    type IntoIter = iter::Rev<slice::Iter<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> iter::IntoIterator for &'a mut RevSlice<T> {
    type Item = &'a mut T;
    type IntoIter = iter::Rev<slice::IterMut<'a, T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::SliceExt;

    #[test]
    fn it_works() {
        let mut a = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.rev()[1], 6);
        assert_eq!(a.rev().iter().nth(1), Some(&6));

        a.rev_mut()[6] = 10;
        assert_eq!(a[0], 10);

        let b = &a.rev()[1..4];
        assert_eq!(b.len(), 3);
        assert_eq!(b[0], 6);
        assert_eq!(b[1], 5);
        assert_eq!(b[2], 4);

        let (x, y) = a.rev().split_at(3);
        assert_eq!(x.len(), 3);
        assert_eq!(y.len(), 4);
        assert_eq!(x.rev(), &[5, 6, 7]);
        assert_eq!(y.rev(), &[10, 2, 3, 4]);
    }

    #[test]
    fn iter_works_too() {
        assert_eq!((0..10).rev().nth(1), Some(8));
    }
}
