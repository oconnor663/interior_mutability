use std::sync::Mutex;

static MY_STRING: Mutex<String> = Mutex::new(String::new());

fn thread1() {
    MY_STRING.lock().unwrap().push_str("hello ");
}

fn thread2() {
    MY_STRING.lock().unwrap().push_str("world ");
}

fn main() {
    let join1 = std::thread::spawn(thread1);
    let join2 = std::thread::spawn(thread2);
    join1.join().unwrap();
    join2.join().unwrap();
    println!("{}", MY_STRING.lock().unwrap());
}
