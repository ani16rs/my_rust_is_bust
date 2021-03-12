fn main() {
    



    // IF CONDITIONS
        let num1 = 3;
 
        if num1 < 5 {
            println!("condition was true!");
        } else {
            println!("condition was false!");
        }

        // condition must be bool, else error (unlike C/C++)
        // if number {
        //     println!("number was 3!");
        // }
       
        if num1 != 0 {
            println!("number was not 0");
        }
       
        // ELSE IF CONDITION
        let num2 = 6;
    
        if num2 % 4 == 0 {
            println!("number was divisible by 4");
        } else if num2 % 3 == 0 {
            println!("number was divisible by 3");
        } else if num2 % 2 == 0 {
            println!("number was divisible by 2");
        } else {
            println!("number was not divisiblke by 2, 3, or 4");
        }
    
        // IF CONDITION IN A LET STATEMENT
        let condition = true;
        let num3 = if condition { 5 } else { 6 };
       println!("value of num3 = {}", num3);

        // let num4 = if condition { 5 } else { "six" };    // error - if else must have same value types
        println!("value of num3 = {}", num3);
}
