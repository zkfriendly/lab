use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game!");

    println!("Guess a number, go wild.");

    let secret_guess = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Something went wrong :(");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            },
        };

        match guess.cmp(&secret_guess) {
            Ordering::Greater => println!("wow, hold on your horses now!"),
            Ordering::Less => println!("let loose a bit"),
            Ordering::Equal => {
                println!("There you go, you made it");
                break;
            },
        }
    }

}
