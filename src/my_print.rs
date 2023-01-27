use std::io::{self}; // I need to look more into this.

pub fn get_input() -> i32{
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)
        .expect("Failed to read line");
    return input_line.trim().parse().expect("Input not an integer");
}

