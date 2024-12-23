use std::fs::read_to_string;

fn discrete_differentiation(v : &Vec<i64>) -> Vec<i64> {
    (1..v.len()).map(|index| v[index] - v[index - 1]).collect()
}

fn next_value(v : Vec<i64>) -> Option<i64> {
    Some(if v.iter().all(|x| x == &0) {
        0
    } else {
        let n = next_value(discrete_differentiation(&v))?;
        v.last()? + n
    })
}

fn previous_value(v : Vec<i64>) -> Option<i64> {
    Some(if v.iter().all(|x| x == &0) {
        0
    } else {
        let n = previous_value(discrete_differentiation(&v))?;
        v.get(0)? - n
    })
}

fn parse_line(line : &str) -> Vec<i64> {
    line.split_ascii_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect()
}

fn main() {
    println!("part 1 : {}", read_to_string("input.txt").unwrap_or("".to_string()).lines().map(parse_line).filter_map(next_value).sum::<i64>());
    println!("part 2 : {}", read_to_string("input.txt").unwrap_or("".to_string()).lines().map(parse_line).filter_map(previous_value).sum::<i64>());
}
