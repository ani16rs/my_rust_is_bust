fn main() {
    // INTRO
        let s1 = String::from("hello oh shite");
        let word1 = first_word_v1(&s1);
        println!("Index of first word (passing String, returning index): {}", word1);


    // STRING SLICES           // reference to a portion of string
        let s2 = String::from("hello world");
        let hello = &s2[0..5];                    // 0 to 4
        let world = &s2[6..11];                   // 6 to 10

        let len = s2.len();

        let slice1 = &s2[0..2];
        let slice1 = &s2[..2];          // these are equal

        let slice2 = &s2[3..len];       // if slice includes last byte of String, you can drop trailing number
        let slice2 = &s2[3..];          // these two are equal

        let slice3 = &s2[0..len];
        let slice3 = &s2[..];            // again, valid


    // first_word rewrite with slices
    let word2 = first_word_v2(&s1); 
    println!("First word (passing String, returning slice)         : {}", word2);

    /* even if s1, s2 are mut, writing s1.clear() or s2.clear() will give error.
     * why?
     * because for word1 and word2, immutable borrowing is done.
     * borrowing rules: if we have an immutable reference to something, we cannot also take a mutable reference.
     * Clear needs to truncate the String, it needs to get a mutable reference.
     * Rust disallows this, and compilation fails.
     * Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
     *
     * word1, 2 have immutable references. cant clear coz it'll end up in shit and flames
     */ 


    // STRING LITERALS ARE SLICES
    let s3 = "hello world";         
    // typeof(s3) = &str  -->  slice pointing to that specific point of the binary
    // &str is immutable. this is why string literals are immutable.


    // STRING SLICES AS PARAMETERS
    let word3 = first_word_v3(&s1[..]);
    println!("First word (passing slice, returning slice)          : {}", word3);


    // OTHER SLICES
    let a = [1, 2, 3, 4, 5];
    let array_slice = &a[1..3];
    // typeof(array_slice) = &[i32]
}


fn first_word_v1(s: &String) -> usize {             // takes String, (without slices) returning index
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_v2(s: &String) -> &str {              // takes String, returns a slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_v3(s: &str) -> &str {                 // takes slice, returns slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
