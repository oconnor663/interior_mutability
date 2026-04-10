use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};

fn main() {
    let my_string = Mutex::new(String::new());
    let ref1: &Mutex<String> = &my_string;
    let ref2: &Mutex<String> = &my_string;

    // ref1.lock().unwrap().push_str("hello ");
    let mut guard1: MutexGuard<String> = ref1.lock().unwrap();
    let refmut1: &mut String = guard1.deref_mut();
    String::push_str(refmut1, "hello ");
    drop(guard1);

    // ref2.lock().unwrap().push_str("world");
    let mut guard2: MutexGuard<String> = ref2.lock().unwrap();
    let refmut2: &mut String = guard2.deref_mut();
    String::push_str(refmut2, "world");
    drop(guard2);

    println!("{}", my_string.lock().unwrap());
}
