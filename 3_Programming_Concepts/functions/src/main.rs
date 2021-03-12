fn main() {
    // FUNCTION PARAMETERS
    println!("Hello, world! \n");
    another_function1();
    another_function2(5);
    another_function3(5, 6);


    // STATEMENTS and EXPRESSIONS
    let a = 6;               // statement
    // let b = (let a = 6);     // statement, error - expected expression, found statement

    let x = 5;
    let y = {
       let x = 3;
       x + 1               // expression - no ";". if ";", expreession --> statement
    };

    println!("Value of y = {} \n", y); 


    // FUNCTIONS with RETURN VALUES
    let w = five();
    println!("Value of w = {}", w); 
    let z = plus_one(10);
    println!("Value of z = {} \n", z); 
}


fn another_function1() {
    println!("Another function");
}

fn another_function2(x: i32) {
    println!("Parameter x = {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("Parameter 1 = {}", x);
    println!("Parameter 2 = {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}
