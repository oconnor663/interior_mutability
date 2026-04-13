use std::cell::{Cell, UnsafeCell};
use std::ops::{Deref, DerefMut};

pub struct GoodRefCell<T> {
    value: UnsafeCell<T>,
    locked: Cell<bool>,
}

pub struct GoodRefMut<'a, T>(&'a GoodRefCell<T>);

impl<T> GoodRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            locked: Cell::new(false),
        }
    }

    pub fn lock(&self) -> GoodRefMut<'_, T> {
        assert!(!self.locked.get());
        self.locked.set(true);
        GoodRefMut(self)
    }
}

impl<'a, T> Drop for GoodRefMut<'a, T> {
    fn drop(&mut self) {
        self.0.locked.set(false);
    }
}

impl<'a, T> Deref for GoodRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.value.get() }
    }
}

impl<'a, T> DerefMut for GoodRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.value.get() }
    }
}

fn add_one(cell: &GoodRefCell<i32>) {
    *cell.lock() += 1;
}

fn main() {
    let cell = GoodRefCell::new(0);
    add_one(&cell);
    assert_eq!(*cell.lock(), 1);
}
