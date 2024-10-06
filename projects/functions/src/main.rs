fn another_function (x:i32, y:char) {
    println!("The value of x: {x} {y}");
} // This entire function is an expression since it doesn't have a semicolon

fn five () -> i32 { 5 } // Implicet return

fn five2 () -> i32 { return 5 } // Manual return

fn add1 (x:i32) -> i32 { x + 1 }

fn main() {
    println!("Hello World");
    another_function(five(),'m'); // this is a statment since it ends with a semicolon

    let y = 6; // this is a statement, it does something and doesn't return

    //let z = (let y = 6); // this wouldn't work because the inner let doesn't return, so z can't be set to anything
}
