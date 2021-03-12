fn main() {
    println!("Hello, world!");

    // SCALAR: integer, float, boolean, char

        // floats
        let x = 1.0;         // f64 - double precision
        let y: f32 = 1.1;    // f32 - single precision

        // numeric ops
        let sum  = 10 + 5;      let diff = 95.5 - 4.3;
        let prod = 4 * 30;      let quot = 56.7 / 32.2;      let rem  = 43 % 5;

        // boolean
        let t = true;
        let f: bool = false;    // explicit type annotation

        // chars - 4 bytes - unicode
        let x = 'c';
        let heart_eye_cat = '😻';


    // COMPOND

        // tuple
        let tup1 = (i32, f64, u8): (500, 6.4, 1);
        let (a, b, c) = tup1;        // destructuring
        
        let tup2 = (500, 6.4, 1);
        let five_hundret = tup2.0;
        let six_point_four = tup2.1;
        let one = tup2.2;

        // array
        let arr1 = [1, 2, 3, 4, 5];             // data stored on stack, not heap
        let arr2: [i32; 5] = [1, 2, 3, 4, 5];
        let arr3 = [3; 5];                      // arr3 = [3, 3, 3, 3, 3]
        let first = arr1[0];
        let second = arr1[1];
        // let oops = arr[10];                  // will give runtime error
        // pretoection from invalid memory access
}
