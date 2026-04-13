use std::ops::{Deref, DerefMut};

pub struct BadRefCell<T> {
    // XXX: This ought to be an `UnsafeCell<T>` and a `Cell<bool>`.
    value: T,
    locked: bool,
}

pub struct BadRefMut<'a, T>(&'a mut BadRefCell<T>);

impl<T> BadRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            locked: false,
        }
    }

    pub fn lock(&self) -> BadRefMut<'_, T> {
        assert!(!self.locked);
        let mut_self: &mut Self = unsafe {
            // XXX: Casting `&T` to `&mut T` is *never* correct.
            // As of Rust 1.73 it's a deny-by-default warning.
            #[allow(mutable_transmutes)]
            std::mem::transmute(self)
        };
        mut_self.locked = true;
        BadRefMut(mut_self)
    }
}

impl<'a, T> Drop for BadRefMut<'a, T> {
    fn drop(&mut self) {
        self.0.locked = false;
    }
}

impl<'a, T> Deref for BadRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.value
    }
}

impl<'a, T> DerefMut for BadRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0.value
    }
}

fn add_one(cell: &BadRefCell<i32>) {
    *cell.lock() += 1;
}

fn main() {
    let cell = BadRefCell::new(0);
    add_one(&cell);
    // XXX: This assert fails in --release mode.
    assert_eq!(*cell.lock(), 1);
}
