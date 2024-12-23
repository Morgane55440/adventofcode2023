use std::fs::read_to_string;

static DIGITS_AS_STRINGS : [(&str,u32);10] = 
    [("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), 
    ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

fn get_digit_if_possible(s : &str) -> Option<u32> {
    match s.chars().next().and_then(|c| c.to_digit(10)) {
        Some(d) => Some(d),
        None => DIGITS_AS_STRINGS.iter().filter_map(|(name,n)| if s.starts_with(name) {Some(*n)} else {None}).next()
    }
}


fn conatenate_first_and_last_digits(s : &str) -> Option<u32> {
    let mut iter = (0..s.len()).filter_map(|i| get_digit_if_possible(&s[i..]));
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
