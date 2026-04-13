use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    name: String,
    friends: Vec<Rc<RefCell<Person>>>,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.into(),
            friends: Vec::new(),
        }
    }

    fn add_friend(&mut self, other: &Rc<RefCell<Person>>) {
        if other.borrow().name != self.name {
            self.friends.push(Rc::clone(other));
        }
    }
}

fn main() {
    let alice = Rc::new(RefCell::new(Person::new("Alice")));
    let bob = Rc::new(RefCell::new(Person::new("Bob")));
    alice.borrow_mut().add_friend(&bob);
    bob.borrow_mut().add_friend(&alice);
    alice.borrow_mut().add_friend(&alice); // panic: already mutably borrowed
}
