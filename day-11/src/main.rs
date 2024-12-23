use std::{collections::HashSet, fs::read_to_string};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Star,
    Void
}

impl Tile {
    fn parse(c : char) -> Self {
        match c {
            '.' => Tile::Void,
            '#' => Tile::Star,
            _ => panic!("char '{}' is not allowed", c)
        }
    }
}

struct Galaxy {
    columns : Vec<usize>,
    lines : Vec<usize>,
    stars : Vec<(usize, usize)>
}

impl Galaxy {

    fn new<I,J>(table : I, time : usize) -> Self
    where
        I : Iterator<Item = J>,
        J : Iterator<Item = Tile>
    {
        let (mut columns, mut lines, mut stars) = 
            (vec![], vec![], vec![]);

        table.enumerate().for_each(|(x,l)|{
            lines.push(time);
            l.enumerate().for_each(|(y,t)|{
                if y >= columns.len() {
                    columns.push(time)
                }
                if t == Tile::Star {
                    lines[x] = 1;
                    columns[y] = 1;
                    stars.push((x,y));
                }
            })
        });
        Galaxy { 
            columns: columns.into_iter().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            }).collect(), 
            lines: lines.into_iter().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            }).collect(), 
            stars 
        }

    }

    fn distance(&self, (xa,ya) : (usize, usize), (xb,yb) : (usize, usize)) -> usize {
        let ((x1,x2), (y1,y2)) = ((xa,xb).order(), (ya,yb).order());
        (self.lines[x2] - self.lines[x1]) + (self.columns[y2] - self.columns[y1])
    }

    fn distances(&self) -> Vec<((usize,usize), (usize, usize), usize)> {
        let mut res = vec![];
        res.reserve_exact((self.stars.len() * (self.stars.len() + 1)) / 2);
        for (i,s1) in self.stars[0..self.stars.len()].iter().enumerate() {
            for s2 in self.stars[(i + 1)..self.stars.len()].iter() {
                res.push((
                    *s1,
                    *s2,
                    self.distance(*s1, *s2)
                ))
            }
        };
        res
    }
    
}

trait Order {
    fn order(self) -> Self;
}

impl<T : Ord> Order for (T,T) {
    fn order(self) -> Self {
        let (a,b) = self;
        match a.cmp(&b) {
            std::cmp::Ordering::Greater => (b,a),
            _ => (a,b)
        }
    }
}


fn main() {
    let input = read_to_string("input.txt").unwrap_or("".to_string());
    let g = Galaxy::new(input.lines().map(|s|s.chars().map(Tile::parse)),2);
    println!("part 1 : {}", Galaxy::new(input.lines().map(|s|s.chars().map(Tile::parse)),2)
                                .distances().iter().map(|(_,_,l)|*l).sum::<usize>());
    println!("part 2 : {}", Galaxy::new(input.lines().map(|s|s.chars().map(Tile::parse)),1000000)
                                .distances().iter().map(|(_,_,l)|*l).sum::<usize>());
}
