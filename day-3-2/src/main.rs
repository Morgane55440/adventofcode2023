use std::{str::Chars, iter::{Peekable, Enumerate}, fs::read_to_string, convert::identity};
use std::collections::BTreeMap;

#[derive(Clone)]
struct Triplet<'a> {
    index : usize,
    prev : Option<Peekable<Enumerate<Chars<'a>>>>,
    cur : Peekable<Enumerate<Chars<'a>>>,
    next : Option<Peekable<Enumerate<Chars<'a>>>>
}

struct PartNumbers<'a> {
    triplet: Triplet<'a>
}

fn check_if_symbol(c : Option<(usize, char)>, line : usize) -> Option<(usize, usize)> {
    c.and_then(|(column,c)| (c == '*').then_some((line, column)))
}

fn get_char(x : (usize, char)) -> char {let (_, c) = x;c}

impl<'a> PartNumbers<'a> {

    fn new(triplet : Triplet<'a>) -> Self {
        Self { triplet: triplet }
    }
    
    fn get_part_number(&mut self, mut is_part : Vec<(usize, usize)>) -> Option<Vec<((usize, usize), u64)>> {
        let mut res : u64 = 0;
        loop {
            let (prev, cur, next) = self.triplet.next();
            if cur.and_then(|(_,c)| c.to_digit(10)).is_none() {
                break;
            }
            match cur.and_then(|(_,c)| c.to_digit(10)) {
                None => break,
                Some(i) => {
                    res = 10 * res + i as u64;
                    is_part.append(&mut vec![check_if_symbol(prev, self.triplet.index - if prev.is_some() {1} else {0}), check_if_symbol(next, self.triplet.index + 1)].into_iter().filter_map(identity).collect::<Vec<_>>());
                    if self.triplet.cur.peek().and_then(|(_,c)| c.to_digit(10)).is_none() {
                        break;
                    }
                }
            }
        }
        is_part.append(&mut vec![ check_if_symbol(self.triplet.cur.peek().copied(), self.triplet.index),
            check_if_symbol(self.triplet.prev.as_mut().and_then(|peekable|peekable.peek().copied()), self.triplet.index - if self.triplet.prev.is_some() {1} else {0}),
            check_if_symbol(self.triplet.next.as_mut().and_then(|peekable|peekable.peek().copied()), self.triplet.index + 1)].into_iter().filter_map(identity).collect::<Vec<_>>());
        (!is_part.is_empty()).then_some(is_part.into_iter().map(|x| (x, res)).collect::<Vec<_>>())
    }
}

impl<'a> Iterator for PartNumbers<'a> {
    type Item = Vec<((usize, usize), u64)>;

    fn next(&mut self) -> Option<Vec<((usize, usize), u64)>> {
        if self.triplet.cur.peek().and_then(|(_,c)| c.to_digit(10)).is_some() {
            match self.get_part_number(vec![]) {
                Some(i) => return Some(i),
                None => ()
            }
        }
        while self.triplet.cur.peek().is_some() {
            let (prev, cur, next) = self.triplet.next();
            let is_part = vec![check_if_symbol(prev, self.triplet.index - if prev.is_some() {1} else {0}),
             check_if_symbol(cur, self.triplet.index), check_if_symbol(next, self.triplet.index + 1)].into_iter().filter_map(identity).collect::<Vec<_>>();
            if self.triplet.cur.peek().and_then(|(_,c)| c.to_digit(10)).is_some() {
                match self.get_part_number(is_part) {
                    Some(i) => return Some(i),
                    None => ()
                }
            }  
        }
        None
    }
}

impl<'a> Triplet<'a> {
    fn new(cur : &'a str, next : Option<&'a str>, index : usize) -> Self {
        Self { prev: None, cur: cur.chars().enumerate().peekable(), next : next.map(|s| s.chars().enumerate().peekable()), index}
    }

    fn next_line(&self, next : Option<&'a str>) -> Option<Triplet<'a>> {
        match (next, self.next.clone()) {
            (next, Some(prev_next)) => Some(Self { prev : Some(self.cur.clone()), cur : prev_next, next : next.map(|x| x.chars().enumerate().peekable()), index : self.index + 1 }),
            (Some(next_line), None) => Some(Self { prev : self.prev.clone(), cur : self.cur.clone(), next : Some(next_line.chars().enumerate().peekable()), index : self.index + 1 }),
            (None, None) => None
        }
    }

    fn next(&mut self) -> (Option<(usize, char)>, Option<(usize, char)>, Option<(usize, char)>) {
        (
            self.prev.as_mut().and_then(|iter| iter.next()), 
            self.cur.next(), 
            self.next.as_mut().and_then(|iter| iter.next())
        )
    }
}

struct LinesByThree<'a , T>
where 
 T : Iterator<Item = &'a str>
{
    inner_iter : T,
    next : Option<Triplet<'a>>
}

impl<'a, T> LinesByThree<'a, T>
where 
 T : Iterator<Item = &'a str>
{
    fn new(mut inner_iter : T) -> Self {
        let fst = inner_iter.next();
        let next = fst.map(|line| Triplet::<'a>::new(line, inner_iter.next(), 0));
        Self { inner_iter, next }
    }
}

impl<'a, T> Iterator for LinesByThree<'a, T>
where 
 T : Iterator<Item = &'a str>
{
    type Item = Triplet<'a>; 

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next.clone();
        self.next = self.next.as_mut().and_then(|triplet| triplet.next_line(self.inner_iter.next()));
        res
    }
}

fn main() {
    LinesByThree::new(read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines()).map(|triplet| PartNumbers::new(triplet)).flatten().fold(BTreeMap::<(usize, usize), Vec<u64>>::new(), 
    |mut map, vec| {for (x,y) in vec { if map.get_mut(&x).map(|v| v.append(&mut vec![y])).is_none() {map.insert(x, vec![y]);};}; map}).into_iter().filter_map(|(x, v)| {
        if v.len() != 2 { None } else { Some((x, v.into_iter().sum::<u64>()))}
    }).for_each(|x| println!("{:?}", x));
    
    println!("{}", LinesByThree::new(read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines()).map(|triplet| PartNumbers::new(triplet)).flatten().fold(BTreeMap::<(usize, usize), Vec<u64>>::new(), 
    |mut map, vec| {for (x,y) in vec { if map.get_mut(&x).map(|v| v.append(&mut vec![y])).is_none() {map.insert(x, vec![y]);};}; map}).into_iter().map(|(_, v)| {
        if v.len() != 2 { 0 } else { v.into_iter().product()}
    }).sum::<u64>());
}
