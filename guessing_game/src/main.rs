use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn game(secret_number: u32) {
    loop {println!("Guess:");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed.");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    
    println!("Guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small."),
        Ordering::Greater => println!("Too big."),
        Ordering::Equal => {
            println!("Correct.");
            break;
        },
    }}

}

fn main() {

    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);
    game(secret_number);
}
