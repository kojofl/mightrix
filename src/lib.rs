#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
//! This library does not aim to be a math library and therefore does not implement
//! common matrix operations, though they might be implemented over time.
//!
//! This crate is currently used to implement the aes algorithm. In that algorithm
//! the state is represented as a column first [`ColumnPrio`] matrix, and all operations
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
use std::ops::{Index, IndexMut};

#[doc(hidden)]
pub mod reftrix;
#[doc(hidden)]
pub mod stacktrix;

type Position = (usize, usize);

/// Matrices ([`Reftrix`], [`Stacktrix`]) with Columnprio use a column first memory representation.
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
pub struct ColumnPrio;

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

/// ColumnPrioMatrix encapsulates all functionality a matrix has that uses the memory
/// interpretation ColumnPrio.
pub trait ColumnPrioMatrix<'a, const R: usize, const C: usize, T> {
    /// Inserts a value at position (x, y) inside the matrix.
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// reftrix.insert((3, 0), 0);
    /// assert_eq!(reftrix.get((3, 0)), &0);
    /// assert_eq!(data[3], 0);
    /// ```
    fn insert(&mut self, location: (usize, usize), value: T);
    /// Get a immutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(reftrix.get((0, 2)), &3);
    /// ```
    fn get(&'a self, location: (usize, usize)) -> &'a T;
    /// Get a mutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    fn get_mut(&'a mut self, location: (usize, usize)) -> &'a mut T;
    /// Fills an entire column with the given data.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    ///
    /// If the data is not the size of a column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// reftrix.fill_col(1, &[7,7,7,7]);
    /// assert_eq!(&data[4..8], &[7,7,7,7]);
    /// ```
    fn fill_col(&mut self, col: usize, data: &[T]);
    /// Fills an entire row with the given data.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    ///
    /// If the data is not the size of a row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// reftrix.fill_row(1, &[7,7,7,7]);
    /// assert_eq!(reftrix.get((1,0)), &7);
    /// assert_eq!(reftrix.get((1,1)), &7);
    /// assert_eq!(reftrix.get((1,2)), &7);
    /// assert_eq!(reftrix.get((1,3)), &7);
    /// assert_eq!(data[1], 7);
    /// assert_eq!(data[5], 7);
    /// assert_eq!(data[9], 7);
    /// assert_eq!(data[13], 7);
    /// ```
    fn fill_row(&mut self, row: usize, data: &[T]);
    /// Retrieves a immutable slice that represents the column.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(reftrix.get_column(0), &[1,1,1,1]);
    /// ```
    fn get_column(&self, col: usize) -> &[T];
    /// Retrieves a mutable slice that represents the column.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    fn get_mut_column(&mut self, col: usize) -> &mut [T];
    /// Retrieves a [`Row`].
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_row(&self, row: usize) -> Row<'_, R, C, T>;
    /// Retrieves a [`RowMut`].
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_mut_row(&mut self, row: usize) -> RowMut<'_, R, C, T>;
    /// Applies a function on all elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// reftrix.apply_all(|el| *el *= 2);
    /// assert_eq!(&data[..], &[2,2,2,2,4,4,4,4,6,6,6,6,8,8,8,8]);
    /// ```
    fn apply_all(&mut self, f: fn(_: &mut T));
    /// Prints out the matrix, this is only usefull for numeric types.
    fn pretty_print(&self);
}

/// RowPrioMatrix encapsulates all functionality a matrix has that uses the memory
/// interpretation RowPrio.
pub trait RowPrioMatrix<'a, const R: usize, const C: usize, T> {
    ///
    /// Inserts a value at position (x, y) inside the matrix.
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// reftrix.insert((3, 1), 0);
    /// assert_eq!(data[13], 0);
    /// ```
    fn insert(&mut self, location: (usize, usize), value: T);
    /// Get a immutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(reftrix.get((0, 2)), &1);
    /// ```
    fn get(&self, location: (usize, usize)) -> &T;
    /// Get a mutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    fn get_mut(&mut self, location: (usize, usize)) -> &mut T;
    /// Fills an entire row with the given data.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    ///
    /// If the data is not the size of a row.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// reftrix.fill_row(1, &[7,7,7,7]);
    /// assert_eq!(&data[4..8], &[7,7,7,7]);
    /// ```
    fn fill_row(&mut self, row: usize, data: &[T]);
    /// Fills an entire column with the given data.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    ///
    /// If the data is not the size of a column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// reftrix.fill_col(1, &[7,7,7,7]);
    /// assert_eq!(data[1], 7);
    /// assert_eq!(data[5], 7);
    /// assert_eq!(data[9], 7);
    /// assert_eq!(data[13], 7);
    /// ```
    fn fill_col(&'a mut self, col: usize, data: &[T]);
    /// Retrieves a [`Column`].
    ///
    /// # Panics
    ///
    /// If the Columns is out of bounds.
    fn get_column(&self, col: usize) -> Column<'_, R, C, T>;
    /// Retrieves a [`ColumnMut`].
    ///
    /// # Panics
    ///
    /// If the Columns is out of bounds.
    fn get_mut_column(&mut self, col: usize) -> ColumnMut<'_, R, C, T>;
    /// Retrieves a immutable slice that represents the row.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_row(&self, row: usize) -> &[T];
    /// Retrieves a mutable slice that represents the row.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_mut_row(&mut self, row: usize) -> &mut [T];
    /// Applies a function on all elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, RowPrio, RowPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut reftrix = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// reftrix.apply_all(|el| *el *= 2);
    /// assert_eq!(&data[..], &[2,2,2,2,4,4,4,4,6,6,6,6,8,8,8,8]);
    /// ```
    fn apply_all(&mut self, f: fn(_: &mut T));
    /// Prints out the matrix, this is only usefull for numeric types.
    fn pretty_print(&self);
}

/// The Row struct represents a imutable matrix row in all [`ColumnPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the row
/// struct. It can however be indexed and iterated over.
pub struct Row<'a, const R: usize, const S: usize, T> {
    start: &'a T,
}

impl<'a, const R: usize, const S: usize, T> Index<usize> for Row<'a, R, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= R {
            panic!("Index {index} out of bounds {}", R);
        }
        unsafe { &*((self.start as *const T).add(index * S)) }
    }
}

/// The RowMut struct represents a mutable matrix row in all [`ColumnPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the RowMut
/// struct. It can however be indexed and iterated over.
pub struct RowMut<'a, const R: usize, const S: usize, T> {
    start: &'a mut T,
}

impl<'a, const R: usize, const S: usize, T> Index<usize> for RowMut<'a, R, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= R {
            panic!("Index {index} out of bounds {}", R);
        }
        unsafe { &*((self.start as *const T).add(index * R)) }
    }
}

impl<'a, const R: usize, const S: usize, T> IndexMut<usize> for RowMut<'a, R, S, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= R {
            panic!("Index {index} out of bounds {}", R);
        }
        unsafe { &mut *((self.start as *mut T).add(index * R)) }
    }
}

#[doc(hidden)]
pub struct RowMutIntoItterator<'a, const R: usize, const S: usize, T> {
    row: RowMut<'a, R, S, T>,
    index: usize,
}

impl<'a, const R: usize, const S: usize, T> IntoIterator for RowMut<'a, R, S, T> {
    type Item = &'a mut T;

    type IntoIter = RowMutIntoItterator<'a, R, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowMutIntoItterator {
            row: self,
            index: 0,
        }
    }
}

impl<'a, const R: usize, const S: usize, T> Iterator for RowMutIntoItterator<'a, R, S, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S {
            return None;
        }
        unsafe {
            let next = &mut *((self.row.start as *mut T).add(self.index * R));
            self.index += 1;
            Some(next)
        }
    }
}

#[doc(hidden)]
pub struct RowIntoItterator<'a, const R: usize, const S: usize, T> {
    row: Row<'a, R, S, T>,
    index: usize,
}

impl<'a, const R: usize, const S: usize, T> IntoIterator for Row<'a, R, S, T> {
    type Item = &'a T;

    type IntoIter = RowIntoItterator<'a, R, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowIntoItterator {
            row: self,
            index: 0,
        }
    }
}

impl<'a, const R: usize, const S: usize, T> Iterator for RowIntoItterator<'a, R, S, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S {
            return None;
        }
        unsafe {
            let next = &*((self.row.start as *const T).add(self.index * R));
            self.index += 1;
            Some(next)
        }
    }
}

/// The Column struct represents a mutable matrix column in all [`RowPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the Column
/// struct. It can however be indexed and iterated over.
pub struct Column<'a, const C: usize, const S: usize, T> {
    start: &'a T,
}

impl<'a, const C: usize, const S: usize, T> Index<usize> for Column<'a, C, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= C {
            panic!("Index {index} out of bounds {}", C);
        }
        unsafe { &*((self.start as *const T).add(index * C)) }
    }
}

/// The Column struct represents a mutable matrix column in all [`RowPrio`] matrices.
///
/// Since the underlying data is not continuous all slice operations are unavailable to the
/// ColumnMut struct. It can however be indexed and iterated over.
pub struct ColumnMut<'a, const C: usize, const S: usize, T> {
    start: &'a mut T,
}

impl<'a, const C: usize, const S: usize, T> Index<usize> for ColumnMut<'a, C, S, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= C {
            panic!("Index {index} out of bounds {}", C);
        }
        unsafe { &*((self.start as *const T).add(index * C)) }
    }
}

impl<'a, const C: usize, const S: usize, T> IndexMut<usize> for ColumnMut<'a, C, S, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= C {
            panic!("Index {index} out of bounds {}", C);
        }
        unsafe { &mut *((self.start as *mut T).add(index * C)) }
    }
}

#[doc(hidden)]
pub struct ColumnMutIntoItterator<'a, const C: usize, const S: usize, T> {
    column: ColumnMut<'a, C, S, T>,
    index: usize,
}

impl<'a, const C: usize, const S: usize, T> IntoIterator for ColumnMut<'a, C, S, T> {
    type Item = &'a mut T;

    type IntoIter = ColumnMutIntoItterator<'a, C, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColumnMutIntoItterator {
            column: self,
            index: 0,
        }
    }
}

impl<'a, const C: usize, const S: usize, T> Iterator for ColumnMutIntoItterator<'a, C, S, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= S {
            return None;
        }
        unsafe {
            let next = &mut *((self.column.start as *mut T).add(self.index * C));
            self.index += 1;
            Some(next)
        }
    }
}

#[doc(hidden)]
pub struct ColumnIntoItterator<'a, const C: usize, const S: usize, T> {
    column: Column<'a, C, S, T>,
    index: usize,
}

impl<'a, const C: usize, const S: usize, T> IntoIterator for Column<'a, C, S, T> {
    type Item = &'a T;

    type IntoIter = ColumnIntoItterator<'a, C, S, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColumnIntoItterator {
            column: self,
            index: 0,
        }
    }
}

impl<'a, const C: usize, const S: usize, T> Iterator for ColumnIntoItterator<'a, C, S, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= C {
            return None;
        }
        unsafe {
            let next = &*((self.column.start as *const T).add(self.index * C));
            self.index += 1;
            Some(next)
        }
    }
}

/// RowPrioMatrix/ IterRows represents an iterator over all rows of a Matrix.
pub struct IterRows<'a, const R: usize, const S: usize, T> {
    row: usize,
    matrix_buffer: &'a [T],
}

impl<'a, const R: usize, const S: usize, T> Iterator for IterRows<'a, R, S, T> {
    type Item = Row<'a, R, S, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= R {
            return None;
        };
        let r = Row {
            start: &self.matrix_buffer[self.row],
        };
        self.row += 1;
        Some(r)
    }
}
