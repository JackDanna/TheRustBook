fn main() {
    //let guess = "42".parse().expect("Not a number"); This won't work without specifying the type of guess

    // Scalar types: Single value types

    // Integer Types
    let uNum: u32 = 4;
    let iNum: i32 = -4;
    let architexture_based_num: isize = -4; // dependant on the architexture of the system

    let decimal = 182_000;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // Floating-Point Types

    let myF64 = 2.0; //Single precision
    let myF32 = 3.0; // Double Precision

    // Numeric Operations

    //addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    //division
    let quotient = 56.7;
    let truncated  = -5/3; // results in -1
    //remainder
    let remainder = 43 % 5;

    // The Boolean Type
    let t = true;
    let f:bool = false;

    // The Character Type
    let c = 'z';
    let z: char = 'i';

    // Compound Types

    // Tuple Type
    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x,y,z) = tup; // How to destructure a tuple
    println!("This is the valueo of {y}");

    let five_hundred = tup.0;
    let unit: () = (); //Unit type

    let array = [1,2,3,4,5]; // on the stack, fixed length, use vectors for unfixed length
    let array2:[i32;3] = [1,2,3];
    let initArray = [0;3]; // creates [0,0,0]
    
    let first = array[0];
    let second = array[1];


}

