use std::{fs::read_to_string, collections::HashMap};

use enum_map::{Enum, EnumMap, enum_map};
use num::integer::lcm;
use regex::Regex;



#[derive(Debug,Clone, Copy, PartialEq, Eq, Enum)]
enum Direction {
    Left,
    Right
}

impl Direction {
    fn parse(c : char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None
        }
    }
}

fn parse_line(line : &str) -> Option<(String, EnumMap<Direction, String>)> {
    let capt = Regex::new("([A-Z]+) = \\(([A-Z]+), ([A-Z]+)\\)").unwrap().captures(line)?;
    Some((capt.get(1)?.as_str().to_owned(), enum_map! {
        Direction::Left => capt.get(2)?.as_str().to_owned(),
        Direction::Right => capt.get(3)?.as_str().to_owned()
    }))
}

struct Loop<T> {
    vec : Vec<T>,
    index : usize
}

impl<T> Loop<T> {
    fn new(vec : Vec<T>) -> Self {
        Self { vec, index : 0 }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<T> Iterator for Loop<T>
where 
    T : Copy
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.vec.len() != 0).then(||{    
            let res = self.vec[self.index % self.vec.len()];
            self.index += 1;
            res
        })
    }
}

impl<T> Clone for Loop<T>
where
    T : Clone 
{
    fn clone(&self) -> Self {
        Self { vec: self.vec.clone(), index: 0 }
    }
}


fn main() {
    let input = read_to_string("input.txt").unwrap_or("".to_string());
    let mut lines =input.lines();
    let mut directions = Loop::new(lines.next().unwrap().chars().filter_map(Direction::parse).collect::<Vec<_>>());
    let direction_nb = directions.len();
    println!("loop_length : {direction_nb}");
    let map = lines.filter_map(parse_line).collect::<HashMap<String, EnumMap<Direction, String>>>();
    println!("part 1 : {}", {
        let (mut node, mut steps) = ("AAA".to_owned(), 0);
        while node != "ZZZ".to_owned() {
            (node, steps) = (map[&node][directions.next().unwrap()].clone(), steps + 1)
        };
        steps
    });
    let loops = map.keys().filter(|s| s.chars().nth(2).unwrap() == 'A').map(String::clone).
        map(|mut node|{
            let (mut start, mut loop_len, mut direction) : (usize, usize, Loop<Direction>) = (0, 0, directions.clone());
            while node.chars().nth(2) != Some('Z') {
                (node, start) = (map[&node][direction.next().unwrap()].clone(), start + 1)
            };
            (node, loop_len) = (map[&node][direction.next().unwrap()].clone(), 1);
            while node.chars().nth(2) != Some('Z') || loop_len % direction_nb != 0 {
                (node, loop_len) = (map[&node][direction.next().unwrap()].clone(), loop_len + 1)
            };
            loop_len
    }).collect::<Vec<_>>();
    println!("part 2 : {}", loops.iter().fold(1, |n, m| lcm(n,*m)));
}
