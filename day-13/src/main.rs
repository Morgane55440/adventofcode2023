use std::{fs::read_to_string, collections::HashMap, ops::BitXor};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Line {
    hash : u128,
    len : u8,
    rock_nb : u8,
}


impl Line {
    fn parse<I>(iter : I) -> Option<Self> 
    where
        I : Iterator<Item=char>
    {
        let (mut hash, mut len, mut rock_nb) : (u128, u8, u8) = (1,0,0);
        for c in iter {
            let i : u8 = match c {
                '.' => 0,
                '#' => 1,
                _ => false.then_some(0)?
            };
            hash = hash.checked_mul(2)? + (i as u128);
            len += 1;
            rock_nb += i;
        }
        Some(Self { hash, len, rock_nb })
    }

    fn distance(&self, other: &Self) -> u32 {
        self.hash.bitxor(other.hash).count_ones()
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.distance(other) == 0
    }
}

#[derive(Debug, Clone)]
struct Terrain {
    lines : Vec<Line>,
    columns: Vec<Line>,
}

impl Terrain {
    fn parse<'a, I>(iter : I) -> Option<Self>
    where
        I : Iterator<Item = &'a str>
    {
        let matrix = iter.map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        matrix.iter().all(|line| line.len() == matrix[0].len()).then_some(())?;
        let lines = matrix.iter().map(|line| Line::parse(line.iter().map(|c|*c))).collect::<Option<Vec<_>>>()?;
        let columns = (0..matrix[0].len()).map(|i|Line::parse(matrix.iter().map(|l| l[i]))).collect::<Option<Vec<_>>>()?;
        Some(Self {
            lines,
            columns,
        })
    }

    fn summarize(&self) -> usize {
        100 * (1..self.lines.len()).filter(|i| symmetric_distance(*i, &self.lines) == Some(0)).sum::<usize>()
        + (1..self.columns.len()).filter(|i| symmetric_distance(*i, &self.columns) == Some(0)).sum::<usize>()
    }

    fn almost_summarize(&self) -> usize {
        100 * (1..self.lines.len()).filter(|i| symmetric_distance(*i, &self.lines) == Some(1)).sum::<usize>()
        + (1..self.columns.len()).filter(|i| symmetric_distance(*i, &self.columns) == Some(1)).sum::<usize>()
    }
}

fn symmetry(index : usize, vec : &Vec<Line>) -> bool {
    index < vec.len() && {
        let half_size = index.min(vec.len() - index);
        half_size != 0 && (0..half_size).all(|i| vec[index - 1 - i] == vec[index + i])
    }
}

fn symmetric_distance(index : usize, vec : &Vec<Line>) -> Option<u64> {
    let half_size = index.min(vec.len() - index);
    (index < vec.len() && half_size != 0).then(||
        (0..half_size).map(|i| vec[index - 1 - i].distance(&vec[index + i]) as u64).sum()
    )
}



fn main() {

    let u = read_to_string("input.txt").unwrap_or("".to_string()).lines().group_by(|l| *l != "").into_iter().filter_map(|(b, v)| b.then_some(v)).map(Terrain::parse).collect::<Option<Vec<_>>>().unwrap();
    println!("part 1 : {}", u.iter().map(Terrain::summarize).sum::<usize>());
    println!("part 2 : {}", u.iter().map(Terrain::almost_summarize).sum::<usize>());
}
