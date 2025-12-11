# mightrix

The mightrix crate exposes matrix types that let continuous memory be used as
if it were a matrix. The dimensions of the matrix, if not heap allocated, 
asserted through const generics. This way the owned variant of the matrix `Stacktrix` can use
a fixed size array on the stack. For the heap allocated `Matrix` the dimensions are provided
in the constructor.

# Example

```rust
use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };

fn main() {
    let mut data = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];

    let mut matrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);

    for el in matrix.get_mut_row(0) {
        *el *= 2;
    }


    for col in matrix.cols_mut() {
        for (i, cell) in col.into_iter().enumerate() {
            *cell += i as u8;
        }
    }
    assert_eq!(&data[..], &[2, 2, 3, 4, 4, 3, 4, 5, 6, 4, 5, 6, 8, 5, 6, 7]);
}
```
Matrix before: 

|          | Col0    | Col1    | Col2    | Col3    |
|----------|---------|---------|---------|---------|
|Row0      | 1       | 2       | 3       | 4       |
|Row1      | 1       | 2       | 3       | 4       |
|Row2      | 1       | 2       | 3       | 4       |
|Row3      | 1       | 2       | 3       | 4       |

Matrix after:

|          | Col0    | Col1    | Col2    | Col3    |
|----------|---------|---------|---------|---------|
|Row0      | 2       | 4       | 6       | 8       |
|Row1      | 2       | 3       | 4       | 5       |
|Row2      | 3       | 4       | 5       | 6       |
|Row3      | 4       | 5       | 6       | 7       |


