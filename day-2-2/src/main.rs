use std::{fs::read_to_string, cmp::max};
use regex::Regex;
use std::convert::identity;

use enum_map::{enum_map, EnumMap, Enum};
use strum::EnumIter;

#[derive(Enum, EnumIter, Clone, Copy)]
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

#[derive(Default)]
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

    fn max(self : &Draw, other : &Draw) -> Draw {
        let mut inner: EnumMap::<Color, u32> = EnumMap::<Color, u32>::default();
        self.inner.iter().for_each(|(color,count)| inner[color] = max(*count, other.inner[color]));
        Draw { inner }
    }

    fn power(self : &Draw) -> u32 {
        self.inner.iter().map(|(color,count)| count).product()
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

fn power_set(s : &str) -> Option<u32> {
    match s.split(": ").nth(1) {
        Some(str) => Some(str.split("; ").filter_map(|str| Draw::new(str)).fold::<Draw,_>(Draw::default(), |d1, d2| d1.max(&d2)).power()),
        None => None
    }
}


fn main() {
    println!("{}", 
    read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines().filter_map(power_set).sum::<u32>());
}