use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(">>> GUESSING NUMBER <<<");
    println!("          ???          ");

    let answer = rand::thread_rng().gen_range(1..=100);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number");
    println!("Right answer {answer}");
    println!("You choice {choice}");
    match choice.cmp(&answer) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }
}
