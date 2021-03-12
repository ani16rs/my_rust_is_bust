fn main() {
    let s1 = String::from("hello");         // s1 immutable   
    let len = calculate_length(&s1);
    println!("{}, {}", s1, len);
    // let len2 = calculate_length2(&s1);   // error - references immut by default. also s1 immutable


    // MUTABLE REFERENCES                       // allows references to modify; BUT: only 1 mut
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s);

        // let r1 = &mut s;                     // error - only 1 mutable refernce to a data in scope
        // let r2 = &mut s;                     // prevents "data races"
        // println!("{}, {}", r1, r2);

        // this is fine
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;


        /* let r3 = &s;
         * let r4 = &s;
         * let r5 = &mut s;                     // error - r5 mut. r5 can change s. r3, r4 immut so s cannot change
         */
    
        // this is fine
        let ref1 = &s;
        let ref2 = &s;
        println!("{}, {}", ref1, ref2);        // ref1 and ref2 no longer used - out of scope - dropped

        let ref3 = &mut s;
        println!("{}", ref3);


    // DANGLING REFERENCES
    // let reference_to_nothing = dangle();     // error
    let reference_to_something = no_dangle();
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

/* fn calculate_length2(s: &String) -> usize {
 * s.push_str(", world");
 * s.len()
 * }
 */

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* fn dangle() -> &String {
 * let s = String::from("hello");
 * &s
 * }                                   // here, s is dropped. we're sending back a ref to s. no s = error
 */

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
