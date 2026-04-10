use std::sync::Mutex;

static MY_STRING: Mutex<String> = Mutex::new(String::new());

fn main() {
    let t1 = std::thread::spawn(|| {
        MY_STRING.lock().unwrap().push_str("hello ");
    });
    let t2 = std::thread::spawn(|| {
        MY_STRING.lock().unwrap().push_str("world ");
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!("{}", MY_STRING.lock().unwrap());
}
