use mightrix::{Reftrix, RowPrio, Stacktrix};

// A Row first Matrix
// 01-01-01-01
// 02-02-02-02
// 03-03-03-03
// 04-04-04-04
#[test]
fn row_first_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut values);
    assert_eq!(*m.get((0, 0)), 1);
    assert_eq!(*m.get((1, 0)), 2);
    assert_eq!(*m.get((2, 0)), 3);
    assert_eq!(*m.get((3, 0)), 4);
}

#[test]
fn row_first_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut values);
    assert_eq!(*m.get((0, 0)), 1);
    assert_eq!(*m.get((1, 0)), 2);
    assert_eq!(*m.get((2, 0)), 3);
    assert_eq!(*m.get((3, 0)), 4);
}

#[test]
#[should_panic]
fn col_out_of_bounds_row_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut values);
    m.get_column(4);
}

#[test]
#[should_panic]
fn col_out_of_bounds_row_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut values);
    m.get_column(4);
}

#[test]
#[should_panic]
fn row_out_of_bounds_row_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Stacktrix::<16, 4, 4, RowPrio, u8>::from_values(&mut values);
    m.get_row(4);
}

#[test]
#[should_panic]
fn row_out_of_bounds_row_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, RowPrio, u8>::from_values(&mut values);
    m.get_row(4);
}
