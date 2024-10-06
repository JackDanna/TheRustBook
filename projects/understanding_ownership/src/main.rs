fn main() {
    let mut _s = "hello"; // stack allocated string
    let mut s2 = String::from("hello"); // heap allocated string, it can be mutated
    s2.push_str(", world!");
    println!("{s2}");
    
    let x = 5;
    let y = x;

    let heap_s1 = String::from("hello");
    //let heap_s2 = heap_s1; This will move the value from heap_s1 to heap_s2, which invalidates heap_s1 from any further use
    let heap_s2 = heap_s1.clone(); // This is a deep clone i.e. the two varialbe now point to different data on the heap

    println!("{heap_s1}, world!");

    // Ownership example
    //takes_ownernship(s2);
    let x = 5;
    makes_copy(x);
    //takes_ownernship(s2); // can't use s2 here since it's ownership was moved to takes_ownership.
    let s3 = gives_ownership();
    let s4 = takes_and_gives_back(s2);

    // References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // & means referencing, so here we are passing a reference to s1, otherwise known as barrowing
    println!("The length of '{s1}' is {len}");

    // //Trying to use two mutalbe references to the same value doesn't work
    // let mut s5 = String::from("hello");
    // let r1 = & s5;
    //let r2 = &mut s5;
    
    // println!("{r1}, {r2}");

    // Dangling References


}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }


fn change (some_string:&mut String) {
    some_string.push_str(", world");
}

// fn change(some_string: &String) {
//     some_string.push_str(", world") // References, like variables, are immutable by default
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_and_gives_back (a_string:String) -> String {
    a_string
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_ownernship(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}
