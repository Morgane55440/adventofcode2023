use std::cmp::Ordering;
use std::{fs::read_to_string, usize};

use enum_map::{enum_map, EnumMap, Enum};
use strum::{EnumIter, IntoEnumIterator};
use sorted_vec::SortedSet;

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
enum Dir {
    North,
    South,
    East,
    West
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East
        }
    }

    fn rotated(&self, other : &Self) -> Self {
        match (self, other) {
            (Dir::North, dir) | (dir, Dir::North) => *dir,
            (Dir::South, dir) | (dir, Dir::South) => dir.opposite(),
            (dir_1, dir_2) => (dir_1 != dir_2).then_some(Dir::North).unwrap_or(Dir::South)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    cost : u8,
    distances : EnumMap<Dir, u32>
}

impl Tile {
    fn parse(c : char) -> Option<Self> {
        c.to_digit(10).and_then(|n| 
            n.try_into().ok()).map(|n| 
                Self { cost: n, distances: (enum_map! {
                    _ => u32::MAX,
                })})
    }
}



#[derive(Debug, Clone)]
struct Labyrinth {
    tiles : Vec<Vec<Tile>>,
}

impl Labyrinth {

    fn new(tiles : Vec<Vec<Tile>>) -> Option<Self> {
        tiles.iter().all(|v| v.len() == tiles[0].len()).then(|| {Self { tiles }})
    }

    fn at(&self, (x,y) : (usize, usize)) -> Option<&Tile> {
        self.tiles.get(x)?.get(y)
    }

    fn next_pos(&self, (x,y) : (usize, usize), dir : Dir) -> Option<(usize, usize)> {
        let next_pos = match dir {
            Dir::North => (x.checked_sub(1)?, y),
            Dir::East => (x, y + 1),
            Dir::South => (x + 1, y),
            Dir::West => (x, y.checked_sub(1)?)
        };
        self.at(next_pos).map(|_| next_pos)
    }

    fn at_unchecked(&self, (x,y) : (usize, usize)) -> &Tile {
        &self.tiles[x][y]
    }

    fn at_unchecked_mut(&mut self, (x,y) : (usize, usize)) -> &mut Tile {
        &mut self.tiles[x][y]
    }

    fn find_updates(&self, 
        pos : (usize, usize),
        in_dir : Dir, 
        steps : &EnumMap<Dir, Vec<Vec<Dir>>>) -> Vec<(Dir, (usize, usize), u32)> {
        let src = self.at_unchecked(pos).to_owned();
        let start_dist = src.distances[in_dir];
        Dir::iter().map(|relative_dir| {
            let movement_dir = relative_dir.rotated(&in_dir.opposite());
            steps[relative_dir].iter().filter_map(move |v|
                v.iter().try_fold((pos, in_dir, start_dist), 
                |(prev_pos, _, distance), new_relative_dir| {
                    let dir = new_relative_dir.rotated(&movement_dir.opposite());
                    self.next_pos(prev_pos, dir).map(|p| (p, dir.opposite(), 
                        distance.checked_add(self.at_unchecked(p).cost as u32).unwrap_or(u32::MAX)))
                }).and_then(|(pos, dir, dist)| {
                    (self.at_unchecked(pos).distances[dir] > dist).then(||{ 
                        (dir,pos, dist)
                    })
                })
            )
        }).flatten().collect()
    }

    fn adjacent_coords(&self, pos : (usize, usize)) -> EnumMap<Dir, Option<(usize, usize)>> {
        enum_map! {
            dir => self.next_pos(pos, dir)
        }
    }

    fn starting_at(mut self, pos : (usize, usize), steps : &EnumMap<Dir, Vec<Vec<Dir>>>) -> Self {
        self.adjacent_coords(pos).into_iter().for_each(|(dir, p)| {
            p.map(|_| { self.at_unchecked_mut(pos).distances[dir] = 0});
        });

        let mut todo_set = SortedSet::<Todo>::new();
        Dir::iter().for_each(|d| {todo_set.replace(Todo { dist: 0, dir: d, pos });});

        while !todo_set.is_empty(){
            let todo_v= todo_set.iter().next().expect("nonempty").to_owned();
            todo_set.remove_item(&todo_v);
            if self.at_unchecked(todo_v.pos).distances[todo_v.dir] >= todo_v.dist {
                self.find_updates(todo_v.pos, todo_v.dir, steps).into_iter().for_each(|(dir, pos, dist)| {
                    self.at_unchecked_mut(pos).distances[dir] = dist;
                    todo_set.replace(Todo { dist, dir, pos });
                });
            }
        };
        self
    }

    fn ending_at(&self, pos : (usize, usize)) -> u32 {
        self.at(pos).and_then(|tile| tile.distances.into_values().min()).unwrap_or(u32::MAX)
    }

    fn ending_at_bottom_right(&self) -> u32 {
        self.ending_at((self.tiles.len() - 1, self.tiles[0].len() - 1))
    }
     
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Todo {
    dist : u32,
    dir : Dir,
    pos : (usize,usize)
}


impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.dist.cmp(&other.dist) != Ordering::Equal {
            return self.dist.cmp(&other.dist);
        }
        if self.dir.cmp(&other.dir) != Ordering::Equal {
            return self.dir.cmp(&other.dir)
        }
        let ((x1,y1),(x2,y2)) = (self.pos, other.pos);
        if x1.cmp(&x2) != Ordering::Equal {
            return x1.cmp(&x2)
        }
        y1.cmp(&y2)
    }
}


fn main() {
    let lab = Labyrinth::new(read_to_string("input.txt").unwrap().lines().map(|l| l.chars().map(Tile::parse).collect::<Option<Vec<_>>>()).collect::<Option<Vec<_>>>().unwrap()).unwrap();
    println!("part 1 : {:?}", lab.clone().starting_at((0,0), &enum_map! {
        Dir::North | Dir::South  => vec![],
        Dir::East | Dir::West  => (1..=3).map(|n| vec![Dir::North;n]).collect::<Vec<_>>()
    }).ending_at_bottom_right());
    println!("part 2 : {:?}", lab.clone().starting_at((0,0), &enum_map! {
        Dir::North | Dir::South  => vec![],
        Dir::East | Dir::West  => (4..=10).map(|n| vec![Dir::North;n]).collect::<Vec<_>>()
    }).ending_at_bottom_right());
}