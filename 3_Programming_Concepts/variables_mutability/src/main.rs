fn main() {

    // MUTABLES
    let mut x = 5;
    println!("Value of x = {}", x);
    
    x = 6;
    println!("Value of x = {}", x);


    // CONSTANTS
    const MAX_Y: u32 = 100_000;
    println!("Value of MAX_Y = {}", MAX_Y);


    // SHADOWING
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("Value of z = {}", z);

    let spaces = "     ";
    println!("Value of spaces = {}", spaces);
    let spaces = spaces.len();
    println!("Value of spaces = {}", spaces);

    // error: can't change mut variable data type
    // let mut s = "     ";
    // s = s.len();
}
