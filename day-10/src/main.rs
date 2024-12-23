use std::{fs::{read_to_string, self}, collections::HashSet};



#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East   
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North
        }
    }
}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Animal
}

impl Pipe {
    fn parse(c : char) -> Option<Self> {
        Some(match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Animal,
            _ => None?
        })
    }

    fn in_and_out(&self) -> Option<(Direction, Direction)> {
        match self {
            Pipe::Vertical => Some((Direction::North, Direction::South)),
            Pipe::Horizontal => Some((Direction::East, Direction::West)),
            Pipe::NorthEast => Some((Direction::North, Direction::East)),
            Pipe::NorthWest => Some((Direction::North, Direction::West)),
            Pipe::SouthEast => Some((Direction::South, Direction::East)),
            Pipe::SouthWest => Some((Direction::South, Direction::West)),
            _ => None
        }
    }

    fn next(&self, dir : Direction) -> Option<Direction> {
        let (in_dir, out_dir) = self.in_and_out()?;
        (dir.opposite() == in_dir).then_some(out_dir).or((dir.opposite() == out_dir).then_some(in_dir))
    }

    fn char(&self) -> char {
        match self {
            Pipe::Vertical => '|',
            Pipe::Horizontal => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthEast => 'F',
            Pipe::SouthWest => '7',
            Pipe::Ground => '.',
            Pipe::Animal => 'S'
        }
    }
}

struct UnLooped {
    animal : (usize, usize)
}

struct Looped {
    loop_set : HashSet<(usize, usize)>
}

#[derive(Debug, Clone)]
struct HotSpring<T> {
    table : Vec<Vec<Pipe>>,
    maybe_loop : T
}
impl<T> HotSpring<T> {

    fn at(&self, (x,y) : (usize, usize)) -> Option<Pipe> {
        self.table.get(x)?.get(y).copied()
    }

    fn next(&self, (x,y) : (usize, usize), direction : Direction) -> Option<(Pipe,  (usize, usize))> {
        let next_pos = match direction {
            Direction::North => (x.checked_sub(1)?, y),
            Direction::South => (x + 1, y),
            Direction::East => (x, y + 1),
            Direction::West => (x, y.checked_sub(1)?)
        };
        Some((self.at(next_pos)?,next_pos))
    }

}

impl HotSpring<UnLooped> {
    fn new<'a, I>(lines : I) -> Option<Self>
    where
        I : Iterator<Item = &'a str>
    {
        let table = lines.map(|s| s.chars().map(Pipe::parse).collect::<Option<Vec<_>>>()).collect::<Option<Vec<_>>>()?;
        let animal = table.iter().enumerate().filter_map(|(x, l)| l.iter().enumerate().filter_map(|(y, p)| (*p == Pipe::Animal).then_some((x, y))).next()).next()?;
        Some( Self { 
            table,
            maybe_loop : UnLooped { animal }
        })
    }

    fn classic_length(&self) -> Option<usize> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
        ].into_iter().filter_map(|start_dir| {
            let (mut steps, mut pos, mut dir, mut tile) : (usize, _, _, _) = 
            (0, self.maybe_loop.animal, start_dir, Pipe::Animal);
            loop {
                (tile, pos) = self.next(pos, dir)?;
                steps += 1;
                if tile == Pipe::Animal {
                    return Some(steps / 2)
                }
                dir = tile.next(dir)?
            }

        }).next()
    }

    fn looped(mut self) -> Result<HotSpring<Looped>, HotSpring<UnLooped>> {
        match self.complete_loop() {
            Some(loop_set) => Ok(HotSpring { table: self.table , maybe_loop: Looped { loop_set } }),
            None => Err(self)
        }
    }

    fn complete_loop(&mut self) -> Option<HashSet<(usize, usize)>> {
        let dirs = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ].into_iter().filter_map(|start_dir| {
            let mut res = HashSet::<(usize, usize)>::new();
            res.insert(self.maybe_loop.animal);
            let mut dir = start_dir;
            let (mut tile, mut pos) = self.next(self.maybe_loop.animal, dir)?;
            loop {
                if !res.insert(pos) {
                    return Some(start_dir)
                }
                dir = tile.next(dir)?;
                (tile, pos) = self.next(pos, dir)?;
            }
        }).collect::<Vec<_>>();
        let real_tile = match (dirs.get(0)?, dirs.get(1)?) {
            (Direction::North, Direction::South) => Pipe::Vertical,
            (Direction::North, Direction::East) => Pipe::NorthEast,
            (Direction::North, Direction::West) => Pipe::NorthWest,
            (Direction::South, Direction::East) => Pipe::SouthEast,
            (Direction::South, Direction::West) => Pipe::SouthWest,
            (Direction::East, Direction::West) => Pipe::Horizontal,
            _ => unreachable!("no other pair should be reachable from the start")
        };
        let (x,y) = self.maybe_loop.animal;
        self.table[x][y] = real_tile;
        println!("new animal : {:?}", self.at(self.maybe_loop.animal));
        let mut res = HashSet::<(usize, usize)>::new();
        res.insert(self.maybe_loop.animal);
        let mut dir = *dirs.get(0).expect("this was already red before");
        let (mut tile, mut pos) = self.next(self.maybe_loop.animal, dir)?;
        loop {
            if !res.insert(pos) {
                return Some(res)
            }
            dir = tile.next(dir)?;
            (tile, pos) = self.next(pos, dir)?;
        }
    }

//    fn insides_set_alt
}

impl HotSpring<Looped> {
    fn classic_length(&self) -> usize {
        self.maybe_loop.loop_set.len() / 2
    }

    fn cleaned(&self) -> Vec<Vec<Pipe>> {
        self.table.iter().enumerate().map(|(x, v)|
            v.iter().enumerate().map(|(y, p)|
                self.maybe_loop.loop_set.contains(&(x,y)).then_some(*p).unwrap_or(Pipe::Ground)
            ).collect()
        ).collect()
    }

    fn insides_set(&self) -> HashSet<(usize, usize)> {
        let loop_set = &self.maybe_loop.loop_set;
        let left_top_most = loop_set.iter().min_by_key(|(x,y)| x + y).expect("loop set should never be empty");
        let mut insides_set = HashSet::<(usize, usize)>::new();
        let (mut pos, mut dir, mut tile)= 
            (*left_top_most, Direction::East, self.at(*left_top_most).expect("left_top_most is from the loop"));
        loop {
            if let Some((_, right_pos)) = self.next(pos, dir.right()) {
                if !loop_set.contains(&right_pos) {
                    let mut todo_set = vec![right_pos];
                    while let Some(next_pos) = todo_set.pop() {
                        if insides_set.insert(next_pos) {
                            vec![
                                Direction::North,
                                Direction::South,
                                Direction::East,
                                Direction::West,
                            ].iter().filter_map(|d| {
                                self.next(next_pos, *d).filter(|(_,p)| !loop_set.contains(p))
                            }).for_each(|(_,p)| 
                            todo_set.push(p)
                            );
                        }
                    }
                }
            }
            (tile, pos) = self.next(pos, dir).expect(" pos is in the loop it should be fine to find next pos");
            if pos == *left_top_most {
                println!("intersection : {:?}", insides_set.intersection(&loop_set));
                return insides_set
            }
            dir = tile.next(dir).expect(&format!("next pos ({:?}, {:?}) is in the loop it should be fine to find next dir ({:?})", tile, pos, dir))
        }
    }

    fn alternate_insides_set(&self) -> HashSet<(usize, usize)> {
        let mut biggenedHS = HotSpring {
            table: deep_alternate_with(&self.cleaned(), Pipe::Ground),
            maybe_loop : Looped { loop_set: self.maybe_loop.loop_set.iter().map(|(x,y)| (2*x,2*y)).collect()}
        };
        self.maybe_loop.loop_set.iter().for_each(|(x,y)| {
            let (a,b) = self.at((*x,*y)).expect("the elements of the loop are inside the map").in_and_out().expect("the elements of the loop have ins and outs");
            vec![a,b].into_iter().for_each(|d| {
                let (_, (new_x, new_y)) = biggenedHS.next((2*x,2*y), d).expect(&format!("the elements of the loop are inside the map : {:?}", (x,y)));
                biggenedHS.table[new_x][new_y] = (new_x % 2 != 0).then_some(Pipe::Vertical).unwrap_or(Pipe::Horizontal);
            })
            }
        );
        fs::write("output2.txt", biggenedHS.table.iter().enumerate().map(|(x, l)| l.iter().enumerate().map(|(y, p)| {
            p.char()
        }).collect::<String>() + "\n").collect::<String>()).unwrap();
        let mut outside_set = HashSet::new();
        outside_set.insert((0,0));
        let mut todo_vec = vec![(0,0)];
        while let Some(next_pos) = todo_vec.pop() {
            let next_pos_list = vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ].iter().filter_map(|d| {
                biggenedHS.next(next_pos, *d).filter(|(pipe,p)| *pipe == Pipe::Ground && !outside_set.contains(p))
            }).collect::<Vec<_>>();
            next_pos_list.iter().for_each(|(_,p)| {
                outside_set.insert(*p);
                todo_vec.push(*p)
            }
            );
        }
        let smaller_outside = outside_set.iter().filter_map(|(x,y)| (x % 2 == 0 && y % 2 == 0).then_some((x / 2, y / 2))).collect::<HashSet<_>>();
        let all = self.table.iter().enumerate().map(|(x, line)| {
            (0..line.len()).map(move |y| (x,y))
        }).flatten().collect::<HashSet<_>>();

        all.difference(&smaller_outside).map(|p|*p).collect::<HashSet<_>>().difference(&self.maybe_loop.loop_set).map(|p|*p).collect::<HashSet<_>>()
    }
}

fn alternate_with(vec : &Vec<Pipe>, p : Pipe) -> Vec<Pipe> {
    let mut res = Vec::new();
    let mut iter = vec.iter();
    match iter.next() {
        Some(vec_p) => res.push(*vec_p),
        None => return vec![]
    }
    res.reserve_exact(2 * vec.len() - 1);
    iter.for_each(|vec_p| {
        res.push(p);
        res.push(*vec_p);
    });
    res
}

fn deep_alternate_with(vec : &Vec<Vec<Pipe>>, p : Pipe) -> Vec<Vec<Pipe>> {
    let mut res = Vec::new();
    let mut iter = vec.iter();
    match iter.next() {
        Some(inner_vec) => res.push(alternate_with(inner_vec, p)),
        None => return vec![]
    }
    res.reserve_exact(2 * vec.len() - 1);
    iter.for_each(|inner_vec| {
        res.push(vec![p;2 * inner_vec.len() - 1]);
        res.push(alternate_with(inner_vec, p));
    });
    res
}


fn main() -> Result<(),()>{
    let hot_spring = HotSpring::new(read_to_string("input.txt").unwrap_or("".to_string()).lines()).ok_or(())?.looped().map_err(|_|())?;
    println!("part 1 : {:?}", hot_spring.classic_length());
    let loop_set = &hot_spring.maybe_loop.loop_set;
    let inside_set = hot_spring.insides_set();
    println!("part 2 : {:?}", inside_set.len());
    println!("part 2 bis : {:?}", hot_spring.alternate_insides_set().len());
    hot_spring.alternate_insides_set().symmetric_difference(&inside_set).for_each(|p|
        println!("difference : {:?}", p)
    );
    fs::write("output.txt", hot_spring.table.iter().enumerate().map(|(x, l)| l.iter().enumerate().map(|(y, p)| {
        if loop_set.contains(&(x,y)) {
            p.char()
        } else if inside_set.contains(&(x,y)) {
            'I'
        } else {
            'O'
        }
    }).collect::<String>() + "\n").collect::<String>()).unwrap();
    hot_spring.alternate_insides_set();
    Ok(())
}
