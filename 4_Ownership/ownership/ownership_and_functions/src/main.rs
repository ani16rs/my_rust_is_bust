fn main() {
    let s = String::from("hello");          // s comes into scope
    takes_ownership(s);                     // s's ownership is passed to fn; invalid now.

    let x = 5;
    makes_copy(x);                          // x is i32 - a "copy trait"; x can be used afterwards
}

fn takes_ownership(some_string: String) {   // some_string comes into scope
    println!("{}", some_string);
}                                           // some_string goes out of scope; "drop" called. memory freed

fn makes_copy(some_integer: i32) {          // some_integer comes into scope
    println!("{}", some_integer);
}                                           // some_integer goes out of scope. nothing happens
