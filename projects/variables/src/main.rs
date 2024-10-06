fn main() {
    //Immutability by default
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6; This won't work if uncommented
    println!("The value of x is: {x}");

    // Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is: {y}");

    // Shadowning vs mut
    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces2 = "    ";
    //spaces2 = spaces2.len(); this won't work since it is trying to change the variables type

}
