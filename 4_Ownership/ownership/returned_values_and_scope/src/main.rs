fn main() {
    let s1 = gives_ownership();             // s1 comes into scope. ownership recieved
    let s2 = String::from("hello");         // s2 comes into scope.
    let s3 = takes_and_gives_it_back(s2);   // s2 moved to fn, moved to s3

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
}
// s3 goes out of scope and "dropped"
// s2 goes out of scope. nothing happens as it was moved
// s1 goes out of scope and "dropped"

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_it_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, i32) {
    let length = s.len();
    (s, len)
}
