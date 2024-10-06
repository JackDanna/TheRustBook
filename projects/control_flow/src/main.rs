fn main() {
    let number = 5;

    if number < 5 {
        println!("number was less than 5");
    } else if number > 5 {
        println!("number was greater than 5");
    } else {
        println!("number is 5");
    }

    let newNum = if number == 5 { 6 } else { 7 }; 

    let condition = true;
    //let number = if condition { 5 } else { "six" }; // This won't work because the let has to have a determined type at compilte time

    // Repetition with Loops
    //loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop
    let mut number = 3;
    while number !=0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {element}");
    }

    // Range with for loop
    for number in (1..4).rev() {
        println!{"{number}!"};
    }
    println!("LIFTOFF!");
}
