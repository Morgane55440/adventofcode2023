use std::{collections::HashSet, fs::read_to_string, io::Empty};

use enum_map::{enum_map, EnumMap, Enum};
use strum_macros::EnumIter;

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter)]
enum Dir {
    North,
    South,
    East,
    West
}

#[derive(Clone, Debug)]
enum Tile {
    Empty(HashSet<Dir>),
    Deflector(EnumMap<Dir, Dir>),
    Duplicator(EnumMap<Dir, Result<(Dir, Dir), Dir>>)
}


impl Tile {
    fn parse(c : char) -> Option<Self> {
        Some(match c {
            '.' => Self::Empty(HashSet::new()),
            '/' => Self::Deflector(enum_map! {
                Dir::North => Dir::East,
                Dir::South => Dir::West,
                Dir::East => Dir::North,
                Dir::West => Dir::South
            }),
            '\\' => Self::Deflector(enum_map! {
                Dir::North => Dir::West,
                Dir::South => Dir::East,
                Dir::East => Dir::South,
                Dir::West => Dir::North
            }),
            '|' => Self::Duplicator(enum_map! {
                Dir::East | Dir::West => Ok((Dir::North, Dir::South)),
                x => Err(x)
            }),
            '-' => Self::Duplicator(enum_map! {
                Dir::South | Dir::North => Ok((Dir::East, Dir::West)),
                x => Err(x)
            }),
            _ => None?
        })
    }

    fn next(&mut self, dir : Dir) -> Vec<Dir> {
        match self {
            Self::Empty(set) => set.insert(dir).then(|| vec![dir]).unwrap_or(vec![]),
            Self::Deflector(map) => vec![map[dir]],
            Self::Duplicator(map) => match map[dir] {
                Ok((a,b)) => vec![a,b],
                Err(c) => vec![c]
            }
        }
    }
}

#[derive(Clone, Debug)]
struct MirrorField {
    tiles : Vec<Vec<Tile>>
}

impl MirrorField {


    fn new(tiles : Vec<Vec<Tile>>) -> Option<Self> {
        tiles.iter().all(|v| v.len() == tiles [0].len()).then(|| Self { tiles })
    }

    fn at(&self, (x,y) : (usize, usize)) -> Option<&Tile> {
        self.tiles.get(x)?.get(y)
    }

    fn at_unchecked_mut(&mut self, (x,y) : (usize, usize)) -> &mut Tile {
        &mut self.tiles[x][y]
    }

    fn next(&self, (x,y) : (usize, usize), direction : Dir) -> Option<((usize, usize), &Tile)> {
        let next_pos = match direction {
            Dir::North => (x.checked_sub(1)?, y),
            Dir::South => (x + 1, y),
            Dir::East => (x, y + 1),
            Dir::West => (x, y.checked_sub(1)?)
        };
        Some((next_pos, self.at(next_pos)?))
    }


    fn spread_size(&mut self, start : (usize, usize), start_dir : Dir) -> usize {
        let mut res_set = HashSet::new();
        let mut stack: Vec<((usize, usize), Dir)> = self.at(start).map(|_| vec![(start, start_dir)]).unwrap_or(vec![]);
        while let Some((pos, dir)) = stack.pop() {
            res_set.insert(pos);
            stack.append(&mut self.at_unchecked_mut(pos).next(dir).into_iter().filter_map(|d| self.next(pos, d).map(|(next_p,_)| (next_p, d))).collect::<Vec<_>>());
        }
        res_set.len()
    }
}


fn main() {
    let field = MirrorField::new(read_to_string("input.txt").unwrap().lines().map(|l| l.chars().map(Tile::parse).collect::<Option<Vec<_>>>()).collect::<Option<Vec<_>>>().unwrap()).unwrap();
    println!("part 1 : {}", field.clone().spread_size((0,0), Dir::East));
    println!("part 2 : {}", {
        let (n,m) = (field.tiles.len(), field.tiles.get(0).map(|v| v.len()).unwrap_or(0));
        (0..n).map(|x| field.clone().spread_size((x,0), Dir::East).max(field.clone().spread_size((x,m-1), Dir::West))).max().max(
            (0..m).map(|y| field.clone().spread_size((0,y), Dir::South).max(field.clone().spread_size((n-1,y), Dir::North))).max()
        ).unwrap_or(0)
    });
}
