#![allow(dead_code)]
#![allow(unused_imports)]

use std::{any::type_name, fmt::Display, io, result, str::FromStr};

const WORDS: [&str; 4] = ["Apple", "Orange", "Juice", "Sex"];



fn count_from_to(x: i32, y: i32) {
    for i in x..=y {
        println!("I count {}", i)
    }
}

fn echo_function() {
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


fn is_negative() {
    println!("Say your name: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Nothing");
    let num: i32 = user_input.trim().parse().expect("msg");
    let is_negative_num: bool;
    if num < 0 { 
        is_negative_num = true;
    } else { 
        is_negative_num = false;
    };
    match is_negative_num {
        true => println!("Num {} is not a negative.", num),
        false => println!("Num {} is negative.", num),
    }
}



fn get_input<T: FromStr>() -> Option<T> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Nothing");
    let value: Option<T> = match user_input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => { return None },
    };
    return value; 
}


fn print_type_of<T: Display>(val: &T) {
    println!("Value is {} and type of this {}", val, type_name::<T>())
}


fn last_char() {
    let oinput: Option<String> = get_input();
    let input: String = match oinput {Some(var) => var, None => String::new()};
    println!("Last char of string <{}> is <{}>", input, input[input.len()])
}


fn main() {
    // echo_function()
    // is_negative();
    last_char();
}
