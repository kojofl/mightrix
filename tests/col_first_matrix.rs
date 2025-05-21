use mightrix::{ColumnPrio, ColumnPrioMatrix, Matrix, Reftrix, Stacktrix};

// A Col first Matrix
// 01-02-03-04
// 01-02-03-04
// 01-02-03-04
// 01-02-03-04
#[test]
fn col_first_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let mut m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut values);
    m.get_mut_row(1);
    assert_eq!(*m.get(0, 0), 1);
    assert_eq!(*m.get(1, 0), 1);
    assert_eq!(*m.get(2, 0), 1);
    assert_eq!(*m.get(3, 0), 1);
}

#[test]
fn col_first_uneven() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
    let mut m = Stacktrix::<12, 4, 3, ColumnPrio, u8>::from_values(&mut values);
    m.get_mut_row(1);
    assert_eq!(*m.get(0, 0), 1);
    assert_eq!(*m.get(0, 1), 2);
    assert_eq!(*m.get(0, 2), 3);
    assert_eq!(*m.get(1, 0), 1);
}

#[test]
fn col_first_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    assert_eq!(*m.get(0, 0), 1);
    assert_eq!(*m.get(1, 0), 1);
    assert_eq!(*m.get(2, 0), 1);
    assert_eq!(*m.get(3, 0), 1);
}

#[test]
fn col_first_heap() {
    let values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Matrix::<ColumnPrio, u8>::from_values(4, 4, &values).expect("correct dimensions");
    assert_eq!(*m.get(0, 0), 1);
    assert_eq!(*m.get(1, 0), 1);
    assert_eq!(*m.get(2, 0), 1);
    assert_eq!(*m.get(3, 0), 1);
}

#[test]
#[should_panic]
fn col_out_of_bounds_col_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut values);
    m.get_column(4);
}

#[test]
#[should_panic]
fn col_out_of_bounds_col_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    m.get_column(4);
}

#[test]
#[should_panic]
fn col_out_of_bounds_col_heap() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Matrix::<ColumnPrio, u8>::from_values(4, 4, &mut values).unwrap();
    m.get_column(4);
}

#[test]
#[should_panic]
fn row_out_of_bounds_col_stack() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Stacktrix::<16, 4, 4, ColumnPrio, u8>::from_values(&mut values);
    m.get_row(4);
}

#[test]
#[should_panic]
fn row_out_of_bounds_col_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    m.get_row(4);
}

#[test]
#[should_panic]
fn row_out_of_bounds_col_heap() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Matrix::<ColumnPrio, u8>::from_values(4, 4, &mut values).unwrap();
    m.get_row(4);
}

#[test]
#[should_panic]
fn row_index_out_of_bounds() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    let row = m.get_row(3);
    let _ = row[5];
}

#[test]
fn iter_rows_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let mut m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    for row in m.rows_mut() {
        for (i, el) in row.into_iter().enumerate() {
            *el += i as u8;
        }
    }
    assert_eq!(
        &values[..],
        &[1, 1, 1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 7, 7, 7, 7]
    );
}

#[test]
fn iter_cols_ref() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let mut m = Reftrix::<4, 4, ColumnPrio, u8>::from_values(&mut values);
    for col in m.cols_mut() {
        for (i, el) in col.into_iter().enumerate() {
            *el += i as u8;
        }
    }
    assert_eq!(
        &values[..],
        &[1, 2, 3, 4, 2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7]
    );
}

#[test]
fn iter_rows_heap() {
    let mut values = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let mut m = Matrix::<ColumnPrio, u8>::from_values(4, 4, &mut values).unwrap();
    for row in m.rows_mut() {
        for (i, el) in row.into_iter().enumerate() {
            *el += i as u8;
        }
    }
    let x: Vec<u8> = m.cols().into_iter().flatten().copied().collect();
    assert_eq!(&x, &[1, 1, 1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 7, 7, 7, 7]);
}
