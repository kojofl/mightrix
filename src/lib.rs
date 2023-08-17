#![warn(missing_docs)]
//! The mightrix crate exposes matrix types that let continuous memory be used as
//! if it where a matrix. The dimensions of the matrix is asserted through const
//! generics. This way the owned variant of the matrix ([`Stacktrix`]) can use
//! a fixed size array on the stack.
//!
//! This library does not aim to be a math library and therefore does not implement
//! common matrix operations, though they might be implemented over time.
//!
//! This crate is currently used to implement the aes algorithm. In that algorithm
//! the state is represented as a collumn first [`CollumPrio`] matrix, and all operations
//! are done on that Matrix.
//!
//! Currently there are two matrix types:
//!
//! * [`Reftrix`]:
//! This matrix uses a mutable slice and therefore manipulates the data directly.
//!
//! * [`Stacktrix`]:
//! This matrix copies the data and uses a fixed size array on the stack, this way the original
//! data is not manipulated.
use std::{
    mem::size_of,
    ops::{Index, IndexMut},
};

#[doc(hidden)]
pub mod reftrix;
#[doc(hidden)]
pub mod stacktrix;

type Position = (usize, usize);

/// Matrices ([`Reftrix`], [`Stacktrix`]) with Columnprio use a collumn first memory representation.
///
/// An array of [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]
/// is therefor represented as the Matrix:
///
/// |          | Col0    | Col1    | Col2    | Col3    |
/// |----------|---------|---------|---------|---------|
/// |Row0      | 1       | 2       | 3       | 4       |
/// |Row1      | 1       | 2       | 3       | 4       |
/// |Row2      | 1       | 2       | 3       | 4       |
/// |Row3      | 1       | 2       | 3       | 4       |
pub struct CollumPrio;

/// Matrices ([`Reftrix`], [`Stacktrix`]) with RowPrio use a row first memory representation.
///
/// An array of [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4]
/// is therefor represented as the Matrix:
///
/// |          | Col0    | Col1    | Col2    | Col3    |
/// |----------|---------|---------|---------|---------|
/// |Row0      | 1       | 1       | 1       | 1       |
/// |Row1      | 2       | 2       | 2       | 2       |
/// |Row2      | 3       | 3       | 3       | 3       |
/// |Row3      | 4       | 4       | 4       | 4       |
pub struct RowPrio;

pub use reftrix::Reftrix;
pub use stacktrix::Stacktrix;

/// The Row struct represents a imutable matrix row in all [`CollumPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the row
/// struct. It can however be indexed and iterated over.
pub struct Row<'a, const L: usize, const S: usize, T> {
    start: &'a T,
}

impl<'a, const L: usize, const S: usize, T> Index<usize> for Row<'a, L, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.start as *const T).add(index * S * size_of::<T>())) }
    }
}

/// The RowMut struct represents a mutable matrix row in all [`CollumPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the RowMut
/// struct. It can however be indexed and iterated over.
pub struct RowMut<'a, const L: usize, const S: usize, T> {
    start: &'a mut T,
}

impl<'a, const L: usize, const S: usize, T> Index<usize> for RowMut<'a, L, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.start as *const T).add(index * S * size_of::<T>())) }
    }
}

impl<'a, const L: usize, const S: usize, T> IndexMut<usize> for RowMut<'a, L, S, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *((self.start as *mut T).add(index * S * size_of::<T>())) }
    }
}

#[doc(hidden)]
pub struct RowMutIntoItterator<'a, const L: usize, const S: usize, T> {
    row: RowMut<'a, L, S, T>,
    index: usize,
}

impl<'a, const L: usize, const S: usize, T> IntoIterator for RowMut<'a, L, S, T> {
    type Item = &'a mut T;

    type IntoIter = RowMutIntoItterator<'a, L, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowMutIntoItterator {
            row: self,
            index: 0,
        }
    }
}

impl<'a, const L: usize, const S: usize, T> Iterator for RowMutIntoItterator<'a, L, S, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= L {
            return None;
        }
        unsafe {
            let next = &mut *((self.row.start as *mut T).add(self.index * S * size_of::<T>()));
            self.index += 1;
            Some(next)
        }
    }
}

#[doc(hidden)]
pub struct RowIntoItterator<'a, const L: usize, const S: usize, T> {
    row: Row<'a, L, S, T>,
    index: usize,
}

impl<'a, const L: usize, const S: usize, T> IntoIterator for Row<'a, L, S, T> {
    type Item = &'a T;

    type IntoIter = RowIntoItterator<'a, L, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowIntoItterator {
            row: self,
            index: 0,
        }
    }
}

impl<'a, const L: usize, const S: usize, T> Iterator for RowIntoItterator<'a, L, S, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= L {
            return None;
        }
        unsafe {
            let next = &*((self.row.start as *const T).add(self.index * S * size_of::<T>()));
            self.index += 1;
            Some(next)
        }
    }
}

/// The Collumn struct represents a mutable matrix collumn in all [`RowPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the Collumn
/// struct. It can however be indexed and iterated over.
pub struct Collumn<'a, const L: usize, const S: usize, T> {
    start: &'a T,
}

impl<'a, const L: usize, const S: usize, T> Index<usize> for Collumn<'a, L, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.start as *const T).add(index * S * size_of::<T>())) }
    }
}

/// The Collumn struct represents a mutable matrix collumn in all [`RowPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the
/// CollumnMut struct. It can however be indexed and iterated over.
pub struct CollumnMut<'a, const L: usize, const S: usize, T> {
    start: &'a mut T,
}

impl<'a, const L: usize, const S: usize, T> Index<usize> for CollumnMut<'a, L, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*((self.start as *const T).add(index * S * size_of::<T>())) }
    }
}

impl<'a, const L: usize, const S: usize, T> IndexMut<usize> for CollumnMut<'a, L, S, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *((self.start as *mut T).add(index * S * size_of::<T>())) }
    }
}

#[doc(hidden)]
pub struct CollumnMutIntoItterator<'a, const L: usize, const S: usize, T> {
    collumn: CollumnMut<'a, L, S, T>,
    index: usize,
}

impl<'a, const L: usize, const S: usize, T> IntoIterator for CollumnMut<'a, L, S, T> {
    type Item = &'a mut T;

    type IntoIter = CollumnMutIntoItterator<'a, L, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        CollumnMutIntoItterator {
            collumn: self,
            index: 0,
        }
    }
}

impl<'a, const L: usize, const S: usize, T> Iterator for CollumnMutIntoItterator<'a, L, S, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= L {
            return None;
        }
        unsafe {
            let next = &mut *((self.collumn.start as *mut T).add(self.index * S * size_of::<T>()));
            self.index += 1;
            Some(next)
        }
    }
}

#[doc(hidden)]
pub struct CollumnIntoItterator<'a, const L: usize, const S: usize, T> {
    collumn: Collumn<'a, L, S, T>,
    index: usize,
}

impl<'a, const L: usize, const S: usize, T> IntoIterator for Collumn<'a, L, S, T> {
    type Item = &'a T;

    type IntoIter = CollumnIntoItterator<'a, L, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        CollumnIntoItterator {
            collumn: self,
            index: 0,
        }
    }
}

impl<'a, const L: usize, const S: usize, T> Iterator for CollumnIntoItterator<'a, L, S, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= L {
            return None;
        }
        unsafe {
            let next = &*((self.collumn.start as *const T).add(self.index * S * size_of::<T>()));
            self.index += 1;
            Some(next)
        }
    }
}
