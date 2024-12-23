use std::{collections::HashSet, fs::read_to_string, iter::repeat};
use regex::{Regex, Match};


struct Card {
    index : u64,
    input : HashSet<u64>,
    output : HashSet<u64>
}

fn get_all_numbers(m : Option<Match>) -> Option<HashSet<u64>> {
    m.map(|s| s.as_str().split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect::<HashSet<_>>())
}

impl Card {
    fn from_line(str : &str) -> Option<Self> {
        let capt = Regex::new("^Card[ ]+([0-9]+): ([^|]*) \\| (.*)$").unwrap().captures(str)?;
        Some(Self { 
            index: capt.get(1).and_then(|s| s.as_str().parse::<u64>().ok())?, 
            input: get_all_numbers(capt.get(2))?, 
            output: get_all_numbers(capt.get(3))? 
        })
    }

    fn matches(&self) -> u64 {
        self.input.intersection(&self.output).count() as u64
    }

    fn points(&self) -> u64 {
        match self.matches() {
            0 => 0,
            n => (2 as u64).pow(n as u32 - 1)
        }
    }
}

struct InterSum<I,J>
where
    I : Iterator<Item = u64>,
    J : Iterator<Item = u64>
{
    i : I,
    j : J
}

impl<I, J> InterSum<I, J>
where
    I : Iterator<Item = u64>,
    J : Iterator<Item = u64> 
{
    fn new(i : I, j : J) -> Self {
        Self { i, j }
    }
}

impl<I, J> Iterator for InterSum<I, J>
where
    I : Iterator<Item = u64>,
    J : Iterator<Item = u64> 
{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.i.next(), self.j.next()) {
            (None, None) => None,
            (n, m) => Some(n.unwrap_or(0) + m.unwrap_or(0))
        }
    }
}

fn main() {
    println!("part 1 :{}", read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines().filter_map(Card::from_line).map(|c|c.points()).sum::<u64>());
    println!("part 2 :{}", {let (n, _) = read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines().filter_map(Card::from_line).map(|c|c.matches()).fold((0, Vec::<u64>::new()), |acc: (u64, Vec<u64>), n| {
        let (res, vec) = acc;
        println!("{:?}",vec);
        let mut iter = vec.into_iter();
        let cardnb = 1 + iter.next().unwrap_or(0);
        (res + cardnb, InterSum::new(repeat(cardnb).take(n.try_into().unwrap()),iter).collect::<Vec<_>>())

    }); n})
}
