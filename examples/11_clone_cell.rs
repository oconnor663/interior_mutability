use std::cell::Cell;
use std::rc::Rc;

// XXX: This is unsound.
fn clone_cell<T: Clone>(cell: &Cell<T>) -> Cell<T> {
    unsafe { Cell::new((*cell.as_ptr()).clone()) }
}

#[derive(Default)]
struct EvilCloner {
    data: Option<&'static u64>,
    friends: Vec<Rc<Cell<EvilCloner>>>,
}

impl Clone for EvilCloner {
    fn clone(&self) -> Self {
        // Take a pointer *into* my data for some reason.
        let data_ptr_ptr: &&u64 = self.data.as_ref().unwrap();
        // I'm feeling evil today! Clear one of my friends.
        self.friends.first().unwrap().set(EvilCloner::default());
        // Use that pointer...SEGFAULT!
        println!("my data: {}", data_ptr_ptr);
        todo!()
    }
}

fn main() {
    let cell = Rc::new(Cell::new(EvilCloner::default()));
    cell.set(EvilCloner {
        data: Some(&42),
        friends: vec![Rc::clone(&cell)],
    });
    _ = clone_cell(&cell);
}
