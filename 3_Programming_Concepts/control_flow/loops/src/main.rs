fn main() {
    
    /* infinite loop
     * loop {
     *     println!("AGAIN!");
     * }
     */

    // RETURNING VALUES FROM LOOPS
    let mut counter  = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result = {} \n", result);


    // CONDITIONAL LOOPS WITH WHILE
    let mut num = 3;
    while num!=0 {
        println!("{}", num);
        num -= 1;
    }

    println!("LIFTOFF! \n");


    // FOR LOOPS
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Value = {}", a[index]);
        index += 1;
    }
    println!("\n");

    for element in a.iter() {
        println!("Value = {}", element);
    }
    println!("\n");

    for number in (1..4).rev() {
        println!("{}", number);
    }
}
