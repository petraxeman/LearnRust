use std::io;
use rand::Rng;


fn main() {
    println!(">>> GUESSING NUMBER <<<");
    println!("          ???          ");

    let answer = rand::thread_rng().gen_range(1..=100);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    println!("Right answer {answer}");
    println!("You choice {choice}");
}
