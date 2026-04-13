use std::cell::Cell;

fn main() {
    // The basics: .get() and .set()
    let cell = Cell::new(42);
    let ref1 = &cell;
    let ref2 = &cell;
    ref1.set(ref2.get() + 1);
    ref2.set(ref1.get() + 1);
    assert_eq!(cell.get(), 44);

    // Cells of non-Copy types
    let string_cell = Cell::new(String::from("one"));
    // _ = string_cell.get();    // not supported
    // _ = string_cell.clone();  // not supported
    string_cell.set(String::from("two"));
    let other_string_cell = Cell::new(String::from("three"));
    string_cell.swap(&other_string_cell);
    assert_eq!(string_cell.into_inner(), "three");
    // _ = &string_cell;  // borrow of moved value

    // Mutable references to Cell contents!
    let mut x_cell = Cell::new(42);
    let x: &mut i32 = x_cell.get_mut();
    *x += 1;
    assert_eq!(x_cell.get(), 43);

    // Cell pointer casts!
    let mut y: i32 = 42;
    let y_cell: &Cell<i32> = Cell::from_mut(&mut y);
    let other_y_cell: &Cell<i32> = y_cell;
    y_cell.set(other_y_cell.get() + 1);
    other_y_cell.set(y_cell.get() + 1);
    assert_eq!(y, 44);
}
