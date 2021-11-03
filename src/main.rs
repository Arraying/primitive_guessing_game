use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read STDIN.");

        let guess = match guess.trim().parse::<u32>() {
            Ok(guess) => guess,
            Err(_) => {
                println!("You need to input a number!");
                continue
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
