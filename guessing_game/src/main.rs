use std::io;

fn main() {
    println!("Guess the number!");

    println!("Prease type in your guess.");

    // Dafuk is mut? Mutable?
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
