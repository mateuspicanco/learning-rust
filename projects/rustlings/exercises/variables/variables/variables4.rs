// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    // we need to define variable x as a mutable variable
    // otherwise assigning the value of 5 to it will fail 
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
