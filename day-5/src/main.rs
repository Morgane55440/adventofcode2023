use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Range {
    start : u64,
    end : u64
}

impl Range {
    fn new(start : u64, end : u64) -> Self {
        Self { start, end }
    }
}

fn ranges_from_nbs<T>(iter :&mut T) -> Vec<Range>
where
    T : Iterator<Item=u64>
{
    let mut vec = Vec::<Range>::new();
    let mut maybe_start = iter.next();
    let mut maybe_len = iter.next();
    while let (Some(start), Some(len)) = (maybe_start, maybe_len) {
        vec.push(Range::new(start, start + len));
        maybe_start = iter.next();
        maybe_len = iter.next();
    };
    vec
}

struct MapRange {
    start : u64,
    end : u64,
    mapped_start : u64
}

impl MapRange {
    fn new(line : &str) -> Option<Self> {
        let v = line.split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<_>>();
        (v.len() == 3 && v[2] != 0).then_some(
            Self { start: v[1], end: v[1] + v[2] , mapped_start: v[0] }
        )
    }

    fn apply(&self, n : u64) -> u64 {
        (n - self.start) + self.mapped_start
    }

    fn apply_range(&self, r : &Range) -> Option<Range> {
        (r.start < self.end && r.end > self.start).then(||
            Range::new(
                self.mapped_start + (r.start - self.start.min(r.start)),
                self.mapped_start + (self.end.min(r.end) - self.start)
            )
        )
    }
}

struct Mapping {
    maps : Vec<MapRange>
}

fn collect_ranges<I>(iter :&mut I) -> Vec<Range>
where
    I : Iterator<Item=Range>,
{
    iter.fold(Vec::<Range>::new(), |mut vec, r| {
        let modified_index = match vec.binary_search_by_key(&r.start, |vec_r|vec_r.start) {
            Ok(i) => {if vec[i].end < r.end {vec[i] = r;};i},
            Err(0) => {vec.insert(0, r);0},
            Err(i) => if vec[i-1].end >= r.start {
                vec[i-1] = Range::new(vec[i-1].start, vec[i-1].end.max(r.end));
                i-1
            } else {vec.insert(i, r);i}
        };
        if modified_index + 1 < vec.len() && vec[modified_index].end >= vec[modified_index+1].start {
            let r = vec.remove(modified_index + 1);
            vec[modified_index] = Range::new(vec[modified_index].start, vec[modified_index].end.max(r.end))
        };
        vec
    })
}

impl Mapping {

    fn new<'a, I>(iter :&mut I) -> Self
    where
        I : Iterator<Item=&'a str>,
    {
        let mut maps = iter.take_while(|s| *s != "").filter_map(|s| MapRange::new(s)).collect::<Vec<_>>();
        maps.sort_by_key(|mr| mr.start);
        Self { maps }
    }

    fn apply(&self, vec : &Vec<u64>) -> Vec<u64> {
        vec.iter().map(
            |n| {
                match self.maps.binary_search_by_key(n, |mr| mr.start) {
                    Ok(i) => Some(i),
                    Err(0) => None,
                    Err(i) =>  (*n < self.maps[i - 1].end).then_some(i - 1)
                }.map(|i| self.maps[i].apply(*n)).unwrap_or(*n)
            }
        ).collect()
    }

    fn apply_range(&self, vec : &Vec<Range>) -> Vec<Range> {
        collect_ranges(&mut vec.iter().map(
            |r| self.maps.iter().filter_map(|mr| mr.apply_range(r))
        ).flatten())
    }
}

struct Instructions<'a> {
    values : Vec<(&'a str, Vec<u64>)>
}

impl<'a> Instructions<'a> {
    fn new<T>(iter :&mut T) -> Self
    where
        T : Iterator<Item=&'a str>,
    {
        let mut values = vec![("seeds", iter.next().map(|s| s.split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<_>>()).unwrap_or(vec![]))];
        iter.next();
        while let Some(next_name) = iter.next().and_then(|s| Regex::new("([a-z]+)-to-([a-z]+) map:").unwrap().captures(s)).and_then(|c| c.get(2)).map(|m| m.as_str()) {
            let next_values = Mapping::new(iter).apply({let (_, v) = values.last().unwrap(); v});
            values.push((next_name, next_values));
        };
        Instructions { values }
    }
}

struct InstructionsRanges<'a> {
    values : Vec<(&'a str, Vec<Range>)>
}

impl<'a> InstructionsRanges<'a> {
    fn new<T>(iter :&mut T) -> Self
    where
        T : Iterator<Item=&'a str>,
    {
        let mut values = vec![("seeds", iter.next().map(|s| ranges_from_nbs(&mut s.split_ascii_whitespace().filter_map(|s| s.parse::<u64>().ok()))).unwrap_or(vec![]))];
        iter.next();
        while let Some(next_name) = iter.next().and_then(|s| Regex::new("([a-z]+)-to-([a-z]+) map:").unwrap().captures(s)).and_then(|c| c.get(2)).map(|m| m.as_str()) {
            let next_values = Mapping::new(iter).apply_range({let (_, v) = values.last().unwrap(); v});
            values.push((next_name, next_values));
        };
        InstructionsRanges { values }
    }
}

fn main() {
    println!("part 1 : {}", Instructions::new(&mut read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines()).values.last().unwrap().1.iter().min().unwrap_or(&0));
    println!("part 2 : {}", InstructionsRanges::new(&mut read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines()).values.last().unwrap().1.iter().map(|r| r.start).min().unwrap_or(0));
}
