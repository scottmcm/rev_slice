#![cfg_attr(not(any(std, test)), no_std)]

#[cfg(any(std, test))]
extern crate core;

use core::ops::{Index, IndexMut};
use core::ops::Range;

pub trait SliceExt {
    type Element;
    fn rev(&self) -> &RevSlice<Self::Element>;
    fn rev_mut(&mut self) -> &mut RevSlice<Self::Element>;
}

impl<T> SliceExt for [T] {
    type Element = T;
    fn rev(&self) -> &RevSlice<Self::Element> {
        unsafe { core::mem::transmute(self) }
    }
    fn rev_mut(&mut self) -> &mut RevSlice<Self::Element> {
        unsafe { core::mem::transmute(self) }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RevSlice<T>([T]);

impl<T> RevSlice<T> {
    pub fn rev(&self) -> &[T] {
        &self.0
    }

    pub fn rev_mut(&mut self) -> &mut [T] {
        &mut self.0
    }

    fn flip(&self, index: usize) -> usize {
        self.len() - (index+1)
    }

    fn flip_range(&self, range: Range<usize>) -> Range<usize> {
        (self.len() - range.end)..(self.len() - range.start)
    }
}

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
        let rmid = self.flip(mid);
        let (a, b) = self.0.split_at(rmid);
        (b.rev(), a.rev())
    }

    pub fn split_at_mut(&mut self, mid: usize) -> (&mut RevSlice<T>, &mut RevSlice<T>) {
        let rmid = self.flip(mid);
        let (a, b) = self.0.split_at_mut(rmid);
        (b.rev_mut(), a.rev_mut())
    }
}

impl<T> Index<usize> for RevSlice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let rindex = self.flip(index);
        &self.0[rindex]
    }
}

impl<T> IndexMut<usize> for RevSlice<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let rindex = self.flip(index);
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

#[cfg(test)]
mod tests {
    use super::SliceExt;

    #[test]
    fn it_works() {
        let mut a = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(a.rev()[1], 6);

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
