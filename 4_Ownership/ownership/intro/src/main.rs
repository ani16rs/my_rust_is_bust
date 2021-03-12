fn main() {
    // VARIABLE SCOPE
    {                       // s is not valid here, not declared yet
        let s = "hello";    // s is valid from this point forwards 
        // do stuff with s
    }                       // scope is over. s no longer valid


    // THE STRING TYPE
    let s1_ = String::from("hello");         // immutable string - stored in heap 
    let mut s2_ = String::from("hello");     // mutable string   - heap
    s2_.push_str(", world");
    println!("{}", s2_);


    // MEMORY and ALLOCATION
    let mut x = 10;
    let y = x;          // simple copy. x, y point to different places
    x = 15;
    println!("{}, {}", x, y);

    let s1 = String::from ("hello");    // s1(ptr, len, capacity) are on stack. ptr points to "hello" stored in heap
    let s2 = s1;                        // ownership transferred from s1 to s2
    // println!("{}", s1);              // error

    let s3 = String::from ("hello"); 
    let s4 = s1.clone();                // deep copy for heap data; no error 
}
