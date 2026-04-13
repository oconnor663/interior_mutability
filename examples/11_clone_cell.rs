use std::cell::UnsafeCell;
use std::rc::Rc;

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
        unsafe { BadCell::new((*self.0.get()).clone()) }
    }
}

struct EvilCloner {
    data: Option<&'static u64>,
    friends: Vec<Rc<BadCell<EvilCloner>>>,
}

impl Clone for EvilCloner {
    fn clone(&self) -> Self {
        let my_data: Option<&&u64> = match &self.data {
            Some(ptr) => Some(ptr),
            None => None,
        };
        if let Some(friend) = self.friends.first() {
            friend.set(EvilCloner {
                data: None,
                friends: Vec::new(),
            });
        }
        println!("my data: {}", **my_data.unwrap());
        unreachable!()
    }
}

fn main() {
    let cell = Rc::new(BadCell::new(EvilCloner {
        data: Some(&42),
        friends: Vec::new(),
    }));
    cell.set(EvilCloner {
        data: Some(&42),
        friends: vec![Rc::clone(&cell)],
    });
    _ = BadCell::clone(&cell);
}
