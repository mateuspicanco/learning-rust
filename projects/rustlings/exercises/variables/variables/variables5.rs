// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // using shadowing we can use the same variable again 
    // it "overshadows" the previously defined variable
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
