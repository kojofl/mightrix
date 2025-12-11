use crate::{
    ColumnPrio, ColumnPrioMatrix, IntermittentSlice, IntermittentSliceMut, IterIntermittentSlices,
    IterMutIntermittentSlices, IterSlices, IterSlicesMut, RowPrio, RowPrioMatrix,
};
use std::{fmt::Debug, marker::PhantomData};

/// Reftrix allows a mutable slice to be used as a Matrix.
///
/// A Reftrix matrix operates on a mutable slice. The number of rows is indicated by R the number
/// of columns by C. MemoryPriority indicates how the underlying memory is interpreted. (see
/// [`ColumnPrio`], [`RowPrio`])
pub struct Reftrix<'a, const R: usize, const C: usize, MemoryPriority, T> {
    inner: &'a mut [T],
    _prio: PhantomData<MemoryPriority>,
}

impl<'a, const R: usize, const C: usize, MemoryPriority, T> Reftrix<'a, R, C, MemoryPriority, T> {
    /// Constructs a Reftrix from a mutable slice with a [`ColumnPrio`] memory interpretation.
    ///
    /// # Panics
    ///
    /// The function will panic if the given slice is not equal to the size of the to be created
    /// matrix R * C.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let reftrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// ```
    pub fn from_values(inner_values: &'a mut [T]) -> Self {
        assert!(inner_values.len() == R * C);
        Self {
            inner: inner_values,
            _prio: PhantomData,
        }
    }
}

impl<const R: usize, const C: usize, T> ColumnPrioMatrix<T> for Reftrix<'_, R, C, ColumnPrio, T>
where
    T: Copy + Default + Debug,
{
    fn insert(&mut self, row: usize, col: usize, value: T) {
        self.get_mut_column(col)[row] = value;
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.get_column(col)[row]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.get_mut_column(col)[row]
    }

    fn fill_col(&mut self, col: usize, data: &[T]) {
        assert_eq!(data.len(), R);
        let start = col * R;
        self.inner[start..start + R].copy_from_slice(data);
    }

    fn fill_row(&mut self, row: usize, data: &[T]) {
        assert_eq!(data.len(), C);
        for (dst, src) in self.get_mut_row(row).into_iter().zip(data.iter()) {
            *dst = *src;
        }
    }

    fn get_column(&self, col: usize) -> &[T] {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        let start = col * R;
        &self.inner[start..start + R]
    }

    fn get_mut_column(&mut self, col: usize) -> &mut [T] {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        let start = col * R;
        &mut self.inner[start..start + R]
    }

    fn get_row(&self, row: usize) -> IntermittentSlice<'_, T> {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        IntermittentSlice {
            start: &self.inner[row],
            slices: R,
            len: C,
        }
    }

    fn get_mut_row(&mut self, row: usize) -> IntermittentSliceMut<'_, T> {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        IntermittentSliceMut {
            start: &mut self.inner[row],
            slices: R,
            len: C,
        }
    }

    fn rows(&self) -> IterIntermittentSlices<'_, T> {
        IterIntermittentSlices {
            slice_index: 0,
            matrix_buffer: self.inner,
            slices: R,
            len: C,
        }
    }

    fn rows_mut(&mut self) -> IterMutIntermittentSlices<'_, T> {
        IterMutIntermittentSlices {
            slice_index: 0,
            matrix_buffer: self.inner,
            slices: R,
            len: C,
        }
    }

    fn cols(&self) -> IterSlices<'_, T> {
        IterSlices {
            matrix_buffer: self.inner,
            len: R,
        }
    }

    fn cols_mut(&mut self) -> IterSlicesMut<'_, T> {
        IterSlicesMut {
            matrix_buffer: self.inner,
            len: R,
        }
    }

    fn apply_all(&mut self, f: fn(&mut T)) {
        for el in self.inner.iter_mut() {
            f(el);
        }
    }

    fn pretty_print(&self) {
        let strings: Vec<Vec<String>> = (0..4)
            .map(|i| {
                self.get_row(i)
                    .into_iter()
                    .map(|el| format!("{:02x?}", el))
                    .collect::<Vec<String>>()
            })
            .collect();
        for v in strings {
            for (i, s) in v.iter().enumerate() {
                print!("{}", s);
                if i != C - 1 {
                    print!("-")
                }
            }
            println!();
        }
    }
}

impl<const R: usize, const C: usize, T> RowPrioMatrix<T> for Reftrix<'_, R, C, RowPrio, T>
where
    T: Copy + Default + Debug,
{
    fn insert(&mut self, row: usize, col: usize, value: T) {
        self.get_mut_row(row)[col] = value;
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.get_row(row)[col]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.get_mut_row(row)[col]
    }

    fn fill_row(&mut self, row: usize, data: &[T]) {
        assert_eq!(data.len(), C);
        let start = row * C;
        self.inner[start..start + C].copy_from_slice(data);
    }

    fn fill_col(&mut self, col: usize, data: &[T]) {
        assert_eq!(data.len(), R);
        for (dst, src) in self.get_mut_column(col).into_iter().zip(data.iter()) {
            *dst = *src;
        }
    }

    fn get_column(&self, col: usize) -> IntermittentSlice<'_, T> {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        IntermittentSlice {
            start: &self.inner[col],
            slices: R,
            len: C,
        }
    }

    fn get_mut_column(&mut self, col: usize) -> IntermittentSliceMut<'_, T> {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        IntermittentSliceMut {
            start: &mut self.inner[col],
            slices: C,
            len: R,
        }
    }

    fn get_row(&self, row: usize) -> &[T] {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        let start = row * C;
        &self.inner[start..start + C]
    }

    fn get_mut_row(&mut self, row: usize) -> &mut [T] {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        let start = row * C;
        &mut self.inner[start..start + C]
    }

    fn rows(&self) -> IterSlices<'_, T> {
        IterSlices {
            matrix_buffer: self.inner,
            len: C,
        }
    }

    fn rows_mut(&mut self) -> IterSlicesMut<'_, T> {
        IterSlicesMut {
            matrix_buffer: self.inner,
            len: C,
        }
    }

    fn cols(&self) -> IterIntermittentSlices<'_, T> {
        IterIntermittentSlices {
            slice_index: 0,
            matrix_buffer: self.inner,
            slices: C,
            len: R,
        }
    }

    fn cols_mut(&mut self) -> IterMutIntermittentSlices<'_, T> {
        IterMutIntermittentSlices {
            slice_index: 0,
            matrix_buffer: self.inner,
            slices: C,
            len: R,
        }
    }
    fn apply_all(&mut self, f: fn(&mut T)) {
        for el in self.inner.iter_mut() {
            f(el);
        }
    }

    fn pretty_print(&self) {
        let strings: Vec<String> = self.inner.iter().map(|el| format!("{:02x?}", el)).collect();
        let _column_width = strings.iter().map(|el| el.len()).max();
        let mut index = 0;
        for _ in 0..R {
            for i in 0..C {
                print!("{}", strings[index]);
                if i != C - 1 {
                    print!("-")
                }
                index += 1;
            }
            println!();
        }
    }
}
