use std::sync::{Arc, Mutex};

static MY_STRING1: Mutex<String> = Mutex::new(String::new());

fn main() {
    let my_string2 = Arc::new(Mutex::new(String::new()));
    let joiner = {
        let my_string2 = Arc::clone(&my_string2);
        std::thread::spawn(move || {
            MY_STRING1.lock().unwrap().push_str("world ");
            my_string2.lock().unwrap().push_str("moon ");
        })
    };
    MY_STRING1.lock().unwrap().push_str("hello ");
    my_string2.lock().unwrap().push_str("goodnight ");
    joiner.join().unwrap();
    println!("{}", MY_STRING1.lock().unwrap());
    println!("{}", my_string2.lock().unwrap());
}
