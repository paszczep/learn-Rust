use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn fun(x: u32, y: u32) {
    let difference = (x - y) as i32;
    println!("Missed by {}", difference.abs());
}

fn check(guess: u32, secret_number: u32) {
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
        }
    }
}

fn game(secret_number: u32) {
    println!("Guess the number");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
    check(guess, secret_number);
    fun(secret_number, guess);
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    
    loop {
        game(secret_number)
    };
    
}
