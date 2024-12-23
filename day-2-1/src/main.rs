use std::fs::read_to_string;
use regex::Regex;
use std::convert::identity;

use enum_map::{enum_map, EnumMap, Enum};
use strum::EnumIter;

#[derive(Enum, EnumIter)]
enum Color {
    Red,
    Blue,
    Green
}

impl Color {
    fn from_string(s : &str) -> Option<Color> {
        match s {
            "blue" => Some(Color::Blue),
            "green" => Some(Color::Green),
            "red" => Some(Color::Red),
            _ => None
        }
    }
}

struct Draw {
    inner : EnumMap<Color, u32>
}

impl Draw {
    fn new(s : &str) -> Option<Draw> {
        let mut inner: EnumMap::<Color, u32> = EnumMap::<Color, u32>::default();
        if s.split(", ").map(|count| -> Option<()> {
            let captures = Regex::new("^([0-9]+) ([a-z]+)$").unwrap().captures(count)?;
            match (captures.get(1).and_then(|n| n.as_str().parse::<u32>().ok()),
                captures.get(2).and_then(|col| Color::from_string(col.as_str()))) {
                (Some(count), Some(color)) => { inner[color] += count; }
                _ => {}
            }
            Some(())
            }
        ).all(|x| x.is_some()) { Some(Draw { inner }) } else { None }
    }

    fn validate(self : &Draw, other : &Draw) -> bool {
        self.inner.iter().all(|(color,count)| other.inner[color] <= *count)
    }
}

fn valid_game(s : &str, d : &Draw) -> Option<u32> {
    let captures = Regex::new("^Game ([0-9]+): (.*)$").unwrap().captures(s)?;
    match (captures.get(1).and_then(|n| n.as_str().parse::<u32>().ok()),
    captures.get(2).and_then(|str| if str.as_str().split("; ").filter_map(|str| Some(d.validate(&Draw::new(str)?))).all(identity) { Some(()) } else { None })) {
        (number, Some(())) => number,
        _ => None
        
    }
}


fn main() {
    println!("{}", 
    read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines().filter_map(|s| valid_game(s, &Draw { inner: enum_map! {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    } })).sum::<u32>());
}