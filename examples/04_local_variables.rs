fn main() {
    let mut my_string = String::new();
    let ref1 = &mut my_string;
    let ref2 = &mut my_string;
    ref1.push_str("hello ");
    ref2.push_str("world");
    println!("{}", my_string);

    let mut my_int = 42;
    let ref1 = &mut my_int;
    let ref2 = &mut my_int;
    *ref1 += 1;
    *ref2 += 1;
    println!("{}", my_int);
}
