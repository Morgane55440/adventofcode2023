use std::{fs::read_to_string, error::Error};

fn conatenate_first_and_last_digits( s : &str) -> Option<u32> {
    let mut iter = s.chars().filter_map(|c| c.to_digit(10));
    let tens = iter.next();
    tens.map(|d| 10 * d + iter.last().unwrap_or(d))
}

fn main() {
    
    println!("{}", 
        read_to_string("input.txt")
        .unwrap_or("".to_string())
        .lines()
        .filter_map(conatenate_first_and_last_digits)
        .sum::<u32>());
}
