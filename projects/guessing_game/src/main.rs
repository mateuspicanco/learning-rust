// let's try to edit this function
// to randomly select a number between 0 and 10
// and handle true inputs
use std::io;
// to pick the random number we can import the rand crate 
// need to add it to the cargo file 
// from the docs, the latest version is 0.8.5
use rand::Rng;

// for comparing stuff we use the idea in the book:
use std::cmp::Ordering;

fn main() {

    // defining a multi-line string
    let prompt_text = "Guess the number! \
        Please input your guess \
        -- Should be a number between 0 and 10";

    // printing the string directly instead of writing the text
    println!("{prompt_text}");

    // make the pick
    // get a random seed that will be used to generate the numbers
    let mut random_seed = rand::thread_rng();
    // generates number within range
    // this syntax is similar to Kusto's !
    // the last element of the range is not inclusive, so we need to add
    // +1 to it to pick up until 10
    let pick: i32 = random_seed.gen_range(0..11);

    // retrieving the input, which must be a string
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input!");

    let guess: i32 = guess.trim().parse().expect("Please type a number!"); 
    
    match guess.cmp(&pick) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You got it right!"),
    }
}
