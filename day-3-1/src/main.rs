use std::{str::Chars, iter::Peekable, fs::read_to_string};

#[derive(Clone)]
struct Triplet<'a> {
    prev : Option<Peekable<Chars<'a>>>,
    cur : Peekable<Chars<'a>>,
    next : Option<Peekable<Chars<'a>>>
}

struct PartNumbers<'a> {
    triplet: Triplet<'a>
}

fn get_next_and_check_if_symbol<'a>(peekable : &mut Option<Peekable<Chars<'a>>>) -> bool {
    check_if_symbol(peekable.as_mut().and_then(|iter| iter.next()))
}

fn check_if_symbol(c : Option<char>) -> bool {
    c.map(|c| c.to_digit(10).is_none() && c != '.').unwrap_or(false)
}

impl<'a> PartNumbers<'a> {

    fn new(triplet : Triplet<'a>) -> Self {
        Self { triplet: triplet }
    }
    
    fn get_part_number(&mut self, mut is_part : bool) -> Option<u64> {
        let mut res : u64 = 0;
        loop {
            let (prev, cur, next) = self.triplet.next();
            if cur.and_then(|c| c.to_digit(10)).is_none() {
                break;
            }
            match cur.and_then(|c| c.to_digit(10)) {
                None => break,
                Some(i) => {
                    res = 10 * res + i as u64;
                    is_part = is_part || check_if_symbol(prev) || check_if_symbol(next);
                    if self.triplet.cur.peek().and_then(|c| c.to_digit(10)).is_none() {
                        break;
                    }
                }
            }
        }
        is_part = is_part 
            || check_if_symbol(self.triplet.cur.peek().copied())
            || check_if_symbol(self.triplet.prev.as_mut().and_then(|peekable|peekable.peek().copied()))
            || check_if_symbol(self.triplet.next.as_mut().and_then(|peekable|peekable.peek().copied()));
        is_part.then_some(res)
    }
}

impl<'a> Iterator for PartNumbers<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.triplet.cur.peek().and_then(|c| c.to_digit(10)).is_some() {
            match self.get_part_number(false) {
                Some(i) => return Some(i),
                None => ()
            }
        }
        while self.triplet.cur.peek().is_some() {
            let (prev, cur, next) = self.triplet.next();
            let is_part = check_if_symbol(prev)
                || check_if_symbol(cur)
                || check_if_symbol(next);
            if self.triplet.cur.peek().and_then(|c| c.to_digit(10)).is_some() {
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
    fn new(cur : &'a str, next : Option<&'a str>) -> Self {
        Self { prev: None, cur: cur.chars().peekable(), next : next.map(|s| s.chars().peekable())}
    }

    fn next_line(&self, next : Option<&'a str>) -> Option<Triplet<'a>> {
        match (next, self.next.clone()) {
            (next, Some(prev_next)) => Some(Self { prev : Some(self.cur.clone()), cur : prev_next, next : next.map(|x| x.chars().peekable())}),
            (Some(next_line), None) => Some(Self { prev : self.prev.clone(), cur : self.cur.clone(), next : Some(next_line.chars().peekable())}),
            (None, None) => None
        }
    }

    fn next(&mut self) -> (Option<char>, Option<char>, Option<char>) {
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
        let next = fst.map(|line| Triplet::<'a>::new(line, inner_iter.next()));
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
    .lines()).for_each(|triplet| println!("{:?}", PartNumbers::new(triplet).collect::<Vec<_>>()));
    println!("{}", LinesByThree::new(read_to_string("input.txt")
    .unwrap_or("".to_string())
    .lines()).map(|triplet| PartNumbers::new(triplet).sum::<u64>()).sum::<u64>());
}
