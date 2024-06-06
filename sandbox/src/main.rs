use std::io;

const WORDS: [&str; 4] = ["Apple", "Orange", "Juice", "Sex"];



fn count_from_to(x: i32, y: i32) {
    for i in x..=y {
        println!("I count {}", i)
    }
}



fn main() {
    println!("Echo expr");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Nothing");
    let user_input = user_input.trim();
    println!("No, it i said: {}", user_input);
    match user_input {
        "hello" => println!("hello bro"),
        "how are you?" => println!("Im fine bro"),
        "count for me" => {
            let mut first_number: String = String::new();
            let mut second_number: String = String::new();
            
            println!("Enter first number:");
            io::stdin().read_line(&mut first_number).expect("Nothing");
            println!("Enter Second number:");
            io::stdin().read_line(&mut second_number).expect("Nothing");
            let first_number: i32 = first_number.trim().parse().expect("Oops it's wrong");
            let second_number: i32 = second_number.trim().parse().expect("Oops it's wrong");

            count_from_to(first_number, second_number)
        },
        "say words you know" => {
            for word in WORDS{
                println!("I know {} word", word);
            }
        },
        _ => println!("Bro what are yo saing?")
    }
}
