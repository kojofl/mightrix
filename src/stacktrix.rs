use crate::{
    Column, ColumnMut, ColumnPrio, ColumnPrioMatrix, IterRows, IterRowsMut, IterSlices,
    IterSlicesMut, Position, Row, RowMut, RowPrio, RowPrioMatrix,
};
use std::{fmt::Debug, marker::PhantomData, mem::MaybeUninit};

/// Stacktrix allows a stack based array to be used as a Matrix.
///
/// A Stacktrix matrix operates on a a stack based array. The number of rows is indicated by R the number
/// of columns by C, S indicates the entire size this is necessary since const expressions are
/// still nightly only. MemoryPriority indicates how the underlying memory is interpreted. (see
/// [`ColumnPrio`], [`RowPrio`])
pub struct Stacktrix<const S: usize, const R: usize, const C: usize, MemoryPrio, T> {
    inner: [T; S],
    _prio: PhantomData<MemoryPrio>,
}

impl<'a, const S: usize, const R: usize, const C: usize, MemoryPriority, T>
    Stacktrix<S, R, C, MemoryPriority, T>
where
    T: Copy + Sized,
{
    /// Constructs a Stacktrix from a slice with a [`ColumnPrio`] memory interpretation.
    ///
    /// # Panics
    ///
    /// The function will panic if the given slice is not equal to the size of the to be created
    /// matrix R * C or if S != R * C.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, ColumnPrio };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let reftrix = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&data[..]);
    /// ```
    pub fn from_values(inner_values: &[T]) -> Self {
        assert!(inner_values.len() == R * C);
        assert!(S == R * C);
        let mut inner: [MaybeUninit<T>; S] = unsafe { [MaybeUninit::uninit().assume_init(); S] };
        // Safety:
        // inner and inner_values are valid pointers and do not overlap.
        unsafe {
            std::ptr::copy_nonoverlapping(inner_values.as_ptr(), inner.as_mut_ptr().cast::<T>(), S)
        };
        // Safety:
        // T and MaybeUninit<T> have the same size.
        // All elements in inner have been initialized.
        Self {
            inner: unsafe { (&inner as *const _ as *const [T; S]).read() },
            _prio: PhantomData,
        }
    }
}

impl<'a, const S: usize, const R: usize, const C: usize, T> ColumnPrioMatrix<'a, R, C, T>
    for Stacktrix<S, R, C, ColumnPrio, T>
where
    Self: 'a,
    T: Copy + Default + Debug,
{
    /// Inserts a value at position (x, y) inside the matrix.
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// m.insert((3, 0), 0);
    /// assert_eq!(m.get((3,0)), &0);
    /// ```
    fn insert(&mut self, location: Position, value: T) {
        self.get_mut_column(location.1)[location.0] = value;
    }
    /// Get a immutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(m.get((0, 2)), &3);
    /// ```
    fn get(&'a self, location: Position) -> &'a T {
        &self.get_column(location.1)[location.0]
    }

    /// Get a mutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    fn get_mut(&'a mut self, location: Position) -> &'a mut T {
        &mut self.get_mut_column(location.1)[location.0]
    }

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
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// m.fill_col(1, &[7,7,7,7]);
    /// assert_eq!(m.get_column(1), &[7,7,7,7]);
    /// ```
    fn fill_col(&mut self, col: usize, data: &[T]) {
        assert_eq!(data.len(), C);
        let start = col * C;
        self.inner[start..start + C].copy_from_slice(data);
    }

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
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// m.fill_row(1, &[7,7,7,7]);
    /// assert_eq!(m.get((1,0)), &7);
    /// assert_eq!(m.get((1,1)), &7);
    /// assert_eq!(m.get((1,2)), &7);
    /// assert_eq!(m.get((1,3)), &7);
    /// ```
    fn fill_row(&mut self, row: usize, data: &[T]) {
        assert_eq!(data.len(), R);
        for (dst, src) in self.get_mut_row(row).into_iter().zip(data.iter()) {
            *dst = *src;
        }
    }

    /// Retrieves a immutable slice that represents the column.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(m.get_column(0), &[1,1,1,1]);
    /// ```
    fn get_column(&self, col: usize) -> &[T] {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        let start = col * C;
        &self.inner[start..start + C]
    }

    /// Retrieves a mutable slice that represents the column.
    ///
    /// # Panics
    ///
    /// If the column is out of bounds.
    fn get_mut_column(&mut self, col: usize) -> &mut [T] {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        let start = col * C;
        &mut self.inner[start..start + C]
    }

    /// Retrieves a [`Row`].
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_row(&self, row: usize) -> Row<'_, R, C, T> {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        Row {
            start: &self.inner[row],
        }
    }

    /// Retrieves a [`RowMut`].
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    fn get_mut_row(&mut self, row: usize) -> RowMut<'_, R, C, T> {
        assert!(
            row < R,
            "Row: {} out of bounds {}, be carefull rows are 0 indexed.",
            row,
            R
        );
        RowMut {
            start: &mut self.inner[row],
        }
    }
    fn rows(&self) -> IterRows<'_, R, C, T> {
        IterRows {
            row: 0,
            matrix_buffer: &self.inner,
        }
    }
    fn rows_mut(&mut self) -> IterRowsMut<'_, R, C, T> {
        IterRowsMut {
            row: 0,
            matrix_buffer: &mut self.inner,
        }
    }
    fn cols(&self) -> IterSlices<'_, R, C, T> {
        IterSlices {
            matrix_buffer: &self.inner[..],
        }
    }

    fn cols_mut(&mut self) -> IterSlicesMut<'_, R, C, T> {
        IterSlicesMut {
            matrix_buffer: &mut self.inner[..],
        }
    }

    /// Applies a function on all elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, ColumnPrio, ColumnPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut data[..]);
    /// m.apply_all(|el| *el *= 2);
    /// assert_eq!(m.get_column(0), &[2,2,2,2]);
    /// assert_eq!(m.get_column(1), &[4,4,4,4]);
    /// assert_eq!(m.get_column(2), &[6,6,6,6]);
    /// assert_eq!(m.get_column(3), &[8,8,8,8]);
    /// ```
    fn apply_all(&mut self, f: fn(&mut T)) {
        for el in self.inner.iter_mut() {
            f(el);
        }
    }

    /// Prints out the matrix, this is only usefull for numeric types.
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

impl<'a, const S: usize, const R: usize, const C: usize, T> RowPrioMatrix<'a, R, C, T>
    for Stacktrix<S, R, C, RowPrio, T>
where
    Self: 'a,
    T: Copy + Default + Debug,
{
    /// Inserts a value at position (x, y) inside the matrix.
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// m.insert((3, 1), 0);
    /// assert_eq!(m.get((3,1)), &0);
    /// ```
    fn insert(&mut self, location: Position, value: T) {
        self.get_mut_row(location.0)[location.1] = value;
    }

    /// Get a immutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(m.get((0, 2)), &1);
    /// ```
    fn get(&self, location: Position) -> &T {
        &self.get_row(location.0)[location.1]
    }

    /// Get a mutable reference to a value in the matrix at location (x, y)
    ///
    /// # Panics
    ///
    /// If the location given is out of bounds in x or y the function panics.
    fn get_mut(&mut self, location: Position) -> &mut T {
        &mut self.get_mut_row(location.0)[location.1]
    }

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
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// m.fill_col(1, &[7,7,7,7]);
    /// assert_eq!(m.get((0,1)), &7);
    /// assert_eq!(m.get((1,1)), &7);
    /// assert_eq!(m.get((2,1)), &7);
    /// assert_eq!(m.get((3,1)), &7);
    /// ```
    fn fill_col(&'a mut self, col: usize, data: &[T]) {
        assert_eq!(data.len(), R);
        for (dst, src) in self.get_mut_column(col).into_iter().zip(data.iter()) {
            *dst = *src;
        }
    }

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
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// m.fill_row(1, &[7,7,7,7]);
    /// assert_eq!(m.get_row(1), &[7,7,7,7]);
    /// ```
    fn fill_row(&mut self, row: usize, data: &[T]) {
        assert_eq!(data.len(), C);
        let start = row * C;
        self.inner[start..start + C].copy_from_slice(data);
    }

    /// Retrieves a [`Column`].
    ///
    /// # Panics
    ///
    /// If the col is out of bounds.
    fn get_column(&self, col: usize) -> Column<'_, R, C, T> {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        Column {
            start: &self.inner[col],
        }
    }

    /// Retrieves a [`ColumnMut`].
    ///
    /// # Panics
    ///
    /// If the col is out of bounds.
    fn get_mut_column(&mut self, col: usize) -> ColumnMut<'_, R, C, T> {
        assert!(
            col < C,
            "Column: {} out of bounds {}, be carefull columns are 0 indexed.",
            col,
            C
        );
        ColumnMut {
            start: &mut self.inner[col],
        }
    }

    /// Retrieves a immutable slice that represents the row.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix};
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// assert_eq!(m.get_row(0), &[1,1,1,1]);
    /// ```
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

    /// Retrieves a mutable slice that represents the row.
    ///
    /// # Panics
    ///
    /// If the row is out of bounds.
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

    /// Applies a function on all elements of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use mightrix::{ Stacktrix, RowPrio, RowPrioMatrix };
    /// let mut data = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4];
    /// let mut m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut data[..]);
    /// m.apply_all(|el| *el *= 2);
    /// assert_eq!(m.get_row(0), &[2,2,2,2]);
    /// assert_eq!(m.get_row(1), &[4,4,4,4]);
    /// assert_eq!(m.get_row(2), &[6,6,6,6]);
    /// assert_eq!(m.get_row(3), &[8,8,8,8]);
    /// ```
    fn apply_all(&mut self, f: fn(&mut T)) {
        for el in self.inner.iter_mut() {
            f(el);
        }
    }

    /// Prints out the matrix, this is only usefull for numeric types.
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
