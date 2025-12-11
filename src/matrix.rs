use crate::{
    ColumnPrio, ColumnPrioMatrix, IntermittentSlice, IntermittentSliceMut, IterIntermittentSlices,
    IterMutIntermittentSlices, IterSlices, IterSlicesMut, RowPrio, RowPrioMatrix,
};
use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// A Matrix allocated on the heap.
///
/// A Matrix operates on a Vec. MemoryPriority indicates how the underlying memory is interpreted. (see
/// [`ColumnPrio`], [`RowPrio`])
#[derive(Debug, Clone)]
pub struct Matrix<MemoryPriority, T> {
    inner: Vec<T>,
    rows: usize,
    cols: usize,
    _prio: PhantomData<MemoryPriority>,
}

#[derive(Debug)]
pub enum MatrixError {
    DimensionError,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl Error for MatrixError {}

impl<MemoryPriority, T: Clone> Matrix<MemoryPriority, T> {
    pub fn new(init: T, rows: usize, cols: usize) -> Self {
        Self {
            inner: vec![init; rows * cols],
            rows,
            cols,
            _prio: PhantomData,
        }
    }
    pub fn from_values(rows: usize, cols: usize, data: &[T]) -> Result<Self, MatrixError> {
        if rows * cols != data.len() {
            return Err(MatrixError::DimensionError);
        }
        Ok(Self {
            inner: Vec::from_iter(data.iter().cloned()),
            rows,
            cols,
            _prio: PhantomData,
        })
    }
}

impl<T> ColumnPrioMatrix<T> for Matrix<ColumnPrio, T>
where
    T: Clone + Default + Debug,
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
        assert_eq!(data.len(), self.rows);
        let start = col * self.rows;
        self.inner[start..start + self.rows].clone_from_slice(data);
    }

    fn fill_row(&mut self, row: usize, data: &[T]) {
        assert_eq!(data.len(), self.cols);
        for (dst, src) in self.get_mut_row(row).into_iter().zip(data.iter()) {
            *dst = src.clone();
        }
    }

    fn get_column(&self, col: usize) -> &[T] {
        assert!(
            col < self.cols,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            self.cols
        );
        let start = col * self.rows;
        &self.inner[start..start + self.rows]
    }

    fn get_mut_column(&mut self, col: usize) -> &mut [T] {
        assert!(
            col < self.cols,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            self.cols
        );
        let start = col * self.rows;
        &mut self.inner[start..start + self.rows]
    }

    fn get_row(&self, row: usize) -> IntermittentSlice<'_, T> {
        assert!(
            row < self.rows,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            self.rows
        );
        IntermittentSlice {
            start: &self.inner[row],
            slices: self.rows,
            len: self.cols,
        }
    }

    fn get_mut_row(&mut self, row: usize) -> IntermittentSliceMut<'_, T> {
        assert!(
            row < self.rows,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            self.rows
        );
        IntermittentSliceMut {
            start: &mut self.inner[row],
            slices: self.rows,
            len: self.cols,
        }
    }

    fn rows(&self) -> IterIntermittentSlices<'_, T> {
        IterIntermittentSlices {
            slice_index: 0,
            matrix_buffer: &self.inner,
            slices: self.rows,
            len: self.cols,
        }
    }

    fn rows_mut(&mut self) -> IterMutIntermittentSlices<'_, T> {
        IterMutIntermittentSlices {
            slice_index: 0,
            matrix_buffer: &mut self.inner,
            slices: self.rows,
            len: self.cols,
        }
    }

    fn cols(&self) -> IterSlices<'_, T> {
        IterSlices {
            matrix_buffer: &self.inner,
            len: self.rows,
        }
    }

    fn cols_mut(&mut self) -> IterSlicesMut<'_, T> {
        IterSlicesMut {
            matrix_buffer: &mut self.inner,
            len: self.rows,
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
                if i != self.cols - 1 {
                    print!("-")
                }
            }
            println!();
        }
    }
}

impl<T> RowPrioMatrix<T> for Matrix<RowPrio, T>
where
    T: Clone + Default + Debug,
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
        assert_eq!(data.len(), self.cols);
        let start = row * self.cols;
        self.inner[start..start + self.cols].clone_from_slice(data);
    }

    fn fill_col(&mut self, col: usize, data: &[T]) {
        assert_eq!(data.len(), self.rows);
        for (dst, src) in self.get_mut_column(col).into_iter().zip(data.iter()) {
            *dst = src.clone();
        }
    }

    fn get_column(&self, col: usize) -> IntermittentSlice<'_, T> {
        assert!(
            col < self.cols,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            self.cols
        );
        IntermittentSlice {
            start: &self.inner[col],
            slices: self.rows,
            len: self.cols,
        }
    }

    fn get_mut_column(&mut self, col: usize) -> IntermittentSliceMut<'_, T> {
        assert!(
            col < self.cols,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            self.cols
        );
        IntermittentSliceMut {
            start: &mut self.inner[col],
            slices: self.cols,
            len: self.rows,
        }
    }

    fn get_row(&self, row: usize) -> &[T] {
        assert!(
            row < self.rows,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            self.rows
        );
        let start = row * self.cols;
        &self.inner[start..start + self.cols]
    }

    fn get_mut_row(&mut self, row: usize) -> &mut [T] {
        assert!(
            row < self.rows,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            self.rows
        );
        let start = row * self.cols;
        &mut self.inner[start..start + self.cols]
    }

    fn rows(&self) -> IterSlices<'_, T> {
        IterSlices {
            matrix_buffer: &self.inner,
            len: self.cols,
        }
    }

    fn rows_mut(&mut self) -> IterSlicesMut<'_, T> {
        IterSlicesMut {
            matrix_buffer: &mut self.inner,
            len: self.cols,
        }
    }

    fn cols(&self) -> IterIntermittentSlices<'_, T> {
        IterIntermittentSlices {
            slice_index: 0,
            matrix_buffer: &self.inner,
            slices: self.cols,
            len: self.rows,
        }
    }

    fn cols_mut(&mut self) -> IterMutIntermittentSlices<'_, T> {
        IterMutIntermittentSlices {
            slice_index: 0,
            matrix_buffer: &mut self.inner,
            slices: self.cols,
            len: self.rows,
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
        for _ in 0..self.rows {
            for i in 0..self.cols {
                print!("{}", strings[index]);
                if i != self.cols - 1 {
                    print!("-")
                }
                index += 1;
            }
            println!();
        }
    }
}
