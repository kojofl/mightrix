# mightrix

The mightrix crate exposes matrix types that let continuous memory be used as
if it where a matrix. The dimensions of the matrix is asserted through const
generics. This way the owned variant of the matrix `Stacktrix` can use
a fixed size array on the stack.

# Example

```rust
use mightrix::{ Reftrix, ColumnPrio, ColumnPrioMatrix };

fn main() {
    let mut data = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];

    let mut matrix = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut data[..]);

    for el in matrix.get_mut_row(0) {
        *el *= 2;
    }

    assert_eq!(&data[..], &[2, 1, 1, 1, 4, 2, 2, 2, 6, 3, 3, 3, 8, 4, 4, 4]);
}
```


