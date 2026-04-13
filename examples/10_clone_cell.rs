use std::cell::UnsafeCell;

struct BadCell<T>(UnsafeCell<T>);

impl<T> BadCell<T> {
    fn new(value: T) -> Self {
        BadCell(UnsafeCell::new(value))
    }

    fn set(&self, value: T) {
        unsafe {
            *self.0.get() = value;
        }
    }
}

impl<T: Clone> Clone for BadCell<T> {
    fn clone(&self) -> Self {
        let value = unsafe { &*self.0.get() }.clone();
        BadCell::new(value)
    }
}

#[derive(Copy)]
struct EvilCloner {
    data: Option<&'static u64>,
}

thread_local! {
    static EVIL: BadCell<EvilCloner> = BadCell::new(EvilCloner {
        data: Some(&42),
    });
}

impl Clone for EvilCloner {
    fn clone(&self) -> Self {
        let ptr_ptr: Option<&&u64> = match &self.data {
            Some(ptr) => Some(ptr),
            None => None,
        };
        EVIL.with(|evil| {
            evil.set(EvilCloner { data: None });
        });
        if let Some(ptr_ptr) = ptr_ptr {
            dbg!(**ptr_ptr);
        }
        *self
    }
}

fn main() {
    EVIL.with(|evil| {
        _ = evil.clone();
    });
}
