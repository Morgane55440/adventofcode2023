use std::{fs::read_to_string, iter::Peekable, cmp::Ordering};


#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Head {
    Jester,
    Queen,
    King,
    Ace
}


#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Number(u8),
    Head(Head)
}

impl Card {
    fn parse(c : char) -> Option<Self> {
        match c {
            'A' => Some(Self::Head(Head::Ace)),
            'K' => Some(Self::Head(Head::King)),
            'Q' => Some(Self::Head(Head::Queen)),
            'J' => Some(Self::Head(Head::Jester)),
            'T' => Some(Self::Number(10)),
            c => c.to_digit(10).map(|n| Self::Number(n as u8))
        }
    }
}

struct GroupCounter<I : Iterator> 
where
    I::Item : PartialEq
{
    iter : Peekable<I>
}

impl<I : Iterator> Iterator for GroupCounter<I>
where
    I::Item : PartialEq
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {

        match self.iter.next() {
            Some(x) => {
                let mut count : usize = 1;
                while self.iter.peek() == Some(&x) {
                    count += 1;
                    self.iter.next();
                }
                Some((count, x))
            }
            None => None
        }
    }
}

trait GroupCounts<T>
where
    Self : Iterator<Item = T> + Sized,
    T : PartialEq 
{
    fn counts(self) -> GroupCounter<Self>;
}

impl<I, T> GroupCounts<T> for I
where
    I  : Iterator<Item = T> + Sized,
    T : PartialEq 
{
    fn counts(self) -> GroupCounter<Self> {
        GroupCounter { iter: self.peekable() }
    }
}


#[derive(Debug,Clone, Copy, PartialEq, Eq)]
struct Figure {
    mainCard : Card,
    mainNumber : u8,
    additionalPair : Option<Card>
}


impl PartialOrd for Figure {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            match (self.mainNumber.partial_cmp(&other.mainNumber)?, self.additionalPair, other.additionalPair) {
                (Ordering::Equal, Some(_), None) => Ordering::Greater,
                (Ordering::Equal, None, Some(_)) => Ordering::Less,
                (ord, _, _) => ord,
            }
        )
    }
}

impl Ord for Figure {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.mainNumber.cmp(&other.mainNumber), self.additionalPair, other.additionalPair) {
            (Ordering::Equal, Some(_), None) => Ordering::Greater,
            (Ordering::Equal, None, Some(_)) => Ordering::Less,
            (ord, _, _) => ord,
        }
    }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards : [Card;5],
    figure : Option<Figure>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            match (self.figure, other.figure, self.cards.cmp(&other.cards)) {
                (Some(_), None, _) => Ordering::Greater,
                (None, Some(_), _) => Ordering::Less,
                (Some(a), Some(b), _) if  a.cmp(&b) != Ordering::Equal => a.cmp(&b),
                (_,_,x) => x
            }
        )
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.figure, other.figure, self.cards.cmp(&other.cards)) {
            (Some(_), None, _) => Ordering::Greater,
            (None, Some(_), _) => Ordering::Less,
            (Some(a), Some(b), _) if  a.cmp(&b) != Ordering::Equal => a.cmp(&b),
            (_,_,x) => x
        }
    }
}

impl Hand {
    fn parse<I>(mut cards : I) -> Option<Hand>
    where
        I : Iterator<Item=Card>
    {
        let mut arr : [Card;5] = [Card::Number(0);5]; 
        for i in 0..5 as usize {
            arr[i] = cards.next()?
        };
        let mut c = arr.clone();
        c.sort();
        let mut counts = c.into_iter().counts().collect::<Vec<_>>();
        counts.sort_by_key(|(n,_)| *n);
        counts.reverse();
        Some( Hand { cards : arr, 
            figure : match counts.get(0) {
                None | Some((1, _)) => None,
                Some((n, c)) => Some(Figure { mainCard: *c, mainNumber: *n as u8, 
                    additionalPair: counts.get(1).filter(|(n, _)| *n > 1).map(|(_, c)| *c)
                    })
                }
            }
        )
    }
}

fn parse_line(line : &str) -> Option<(Hand, usize)>
{
    let mut split = line.split_ascii_whitespace();
    Some((
        Hand::parse(split.next()?.chars().filter_map(Card::parse))?,
        split.next()?.parse::<usize>().ok()?
    ))
}

fn main() {
    let mut res = read_to_string("input.txt").unwrap_or("".to_string()).lines().filter_map(parse_line).collect::<Vec<_>>();
    res.sort_by_key(|(h, _)| *h);
    println!("part 1 : {}", res.iter().zip(1 as usize..).map(|((_, u), n )| u * n).sum::<usize>());
}
