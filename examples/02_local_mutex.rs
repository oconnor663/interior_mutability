use std::sync::Mutex;

fn main() {
    let my_string = Mutex::new(String::new());
    let ref1 = &my_string;
    let ref2 = &my_string;
    ref1.lock().unwrap().push_str("hello ");
    ref2.lock().unwrap().push_str("world");
    println!("{}", my_string.lock().unwrap());
}
