use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(">>> GUESSING NUMBER <<<");
    println!("          ???          ");

    let answer = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You choice {choice}");

        match choice.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win"); break}
        }
    }
}