use std::{str::FromStr, fs::read_to_string, collections::{HashSet, BTreeSet}, ops::Neg};
use strum::{EnumIter, IntoEnumIterator};
use enum_map::{enum_map, EnumMap, Enum};

#[derive(Enum, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
enum Dir {
    North,
    South,
    East,
    West
}

impl Dir {

    fn parse(c : char) -> Option<Self> {
        Some(match c {
            'U' => Dir::North,
            'L' => Dir::West,
            'D' => Dir::South,
            'R' => Dir::East,
            _ => None?
        })
    }

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

    fn sign<T : Neg<Output = T>>(&self, x : T) -> T {
        match self { Dir::North | Dir::West => -x, _ => x}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Color {
    red : u8,
    green : u8,
    blue : u8
}

fn parse_double_hex<I>(chars : &mut I) -> Option<u8> 
where I : Iterator<Item=char>  {
    Some((chars.next()?.to_digit(16)? * 16 + 
    chars.next()?.to_digit(16)?).try_into().ok()?)
}

impl Color {
    fn parse<I>(chars : &mut I) -> Option<Self> 
    where
        I : Iterator<Item=char> {
        Some(Self {
            red: parse_double_hex(chars.skip(2).by_ref())?,
            green: parse_double_hex(chars)?,
            blue: parse_double_hex(chars)?
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct LineInfo {
    dir : Dir,
    len : i128,
    color : Color
}

impl LineInfo {
    fn parse(line : &str) -> Option<Self> {
        let mut split = line.split_whitespace();
        Some(Self {
            dir: Dir::parse(split.next()?.chars().next()?)?,
            len: split.next()?.parse::<i128>().ok()?,
            color: Color::parse(&mut split.next()?.chars())?
        })
    }

    fn uncover(&self) -> Self {
        Self { dir: match self.color.blue % 4 {
            0 => Dir::East,
            1 => Dir::South,
            2 => Dir::West,
            _ => Dir::North,
        }, 
        len: (self.color.blue as i128) / 0x10 +
            (self.color.green as i128) * 0x10 +
            (self.color.red as i128) * 0x1000, 
        color: Color { red: 0, green: 0, blue: 0 }
        }
    }
}

#[derive(Debug, Clone)]
struct Bounds {
    start : (usize, usize),
    size : (Vec<i128>, Vec<i128>)
}

fn compute_bounds<I>(iter : I) -> ((usize, usize), Vec<i128>, Vec<i128>)
where I : Iterator<Item = LineInfo> {
    let ((mut setx,mut sety), end) = iter.fold(((BTreeSet::<i128>::from([0]), BTreeSet::<i128>::from([0])),(0,0)), |((mut setx, mut sety),(mut x, mut y)) : ((BTreeSet<i128>, BTreeSet<i128>),(i128,i128)), line| {
        match line.dir {
            Dir::North => x -= line.len as i128,
            Dir::South => x += line.len as i128,
            Dir::East => y += line.len as i128,
            Dir::West => y -= line.len as i128
        };setx.insert(x);setx.insert(x+1);sety.insert(y);sety.insert(y+1);  
        ((setx, sety),(x,y))
    });
    let (vecx, vecy) : (Vec<i128>, Vec<i128>) = (setx.into_iter().collect(), sety.into_iter().collect());
    ((vecx.binary_search(&0).unwrap(),vecy.binary_search(&0).unwrap()), vecx, vecy)
}

#[derive(Debug, Clone)]
struct Field {
    inner : Vec<Vec<Option<()>>>,
    sizex : Vec<i128>, 
    sizey : Vec<i128>
}

impl Field {
    fn new(sizex : Vec<i128>, sizey : Vec<i128>) -> Self {
        Self { inner: (0..sizex.len()).map(|_| vec![None;sizey.len()]).collect(), sizex, sizey }
    }

    fn at(&self, (x,y) : (usize, usize)) -> Option<&Option<()>> {
        self.inner.get(x)?.get(y)
    }

    fn ax_pos(&self, dir : Dir, (x,y) : (usize, usize)) -> i128 {
        match dir { Dir::North | Dir::South => self.sizex[x], _ => self.sizey[y]}
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

    fn move_unchecked(&self, (x,y) : (&mut usize, &mut usize), dir : Dir) {
        match dir {
            Dir::North => *x -= 1,
            Dir::East => *y += 1,
            Dir::South => *x += 1,
            Dir::West => *y -= 1
        }
    }

    fn at_unchecked(&self, (x,y) : (usize, usize)) -> Option<()> {
        self.inner[x][y]
    }

    fn at_unchecked_mut(&mut self, (x,y) : (usize, usize)) -> &mut Option<()> {
        &mut self.inner[x][y]
    }

    fn draw_lines_unchecked<I>(mut self, (mut x, mut y) : (usize, usize), iter : I) -> Self 
    where I : Iterator<Item = LineInfo> {
        iter.for_each(|info| {
            let start = self.ax_pos(info.dir, (x,y));
            while self.ax_pos(info.dir, (x,y)) != start + info.dir.sign(info.len as i128) {
                self.move_unchecked((&mut x,&mut y), info.dir);
                *self.at_unchecked_mut((x,y)) = Some(());
            }
        });
        self
    }

    fn count_inside(&self) -> i128 {
        let mut res_set = HashSet::<(usize, usize)>::new();
        let (x, y) = (self.inner.len(), self.inner.get(0).map(|v| v.len()).unwrap_or(0));
        (0..x).for_each(|i| {
            self.spread((i, 0), &mut res_set);
            self.spread((i, y - 1), &mut res_set);
        });
        (0..y).for_each(|j| {
            self.spread((0, j), &mut res_set);
            self.spread((x - 1, j), &mut res_set);
        });
        (self.sizex.last().copied().unwrap_or(0) - self.sizex.first().copied().unwrap_or(0) + 1) *
        (self.sizey.last().copied().unwrap_or(0) - self.sizey.first().copied().unwrap_or(0) + 1) - {
            res_set.len() as i128 +
            (0..x).map(|i| {(1..y).filter_map(|j| [(i,j), (i,j-1)].iter().any(|p| res_set.contains(p)
            ).then(|| self.sizey[j] - self.sizey[j-1] - 1)).sum::<i128>()}).sum::<i128>() +
            (0..y).map(|j| {(1..x).filter_map(|i| [(i,j), (i-1,j)].iter().any(|p| res_set.contains(p)
            ).then(|| self.sizex[i] - self.sizex[i-1] - 1)).sum::<i128>()}).sum::<i128>() +
            (1..x).map(|i|(1..y).filter_map(|j| 
                [(i,j), (i-1,j), (i,j-1),(i-1,j-1)].iter().any(|p| res_set.contains(p)).then(|| 
                (self.sizex[i] - self.sizex[i-1] - 1) * (self.sizey[j] - self.sizey[j-1] - 1))).sum::<i128>()).sum::<i128>()
        }
    }

    fn spread(&self, start_pos : (usize, usize), set : &mut HashSet<(usize, usize)>) {
        let mut stack = vec![start_pos];
        while let Some(p) = stack.pop() {
            (self.at_unchecked(p).is_none() && set.insert(p)).then(||{
                Dir::iter().filter_map(|d|self.next_pos(p, d)).for_each(|new_p| {
                    stack.push(new_p)
                })
            });
        }
    }

}
fn main() {
    let lines = read_to_string("input.txt").unwrap().lines().map(
        LineInfo::parse).collect::<Option<Vec<_>>>().unwrap();
    let (start1, vec1x, vec1y) = compute_bounds(lines.iter().map(|&t|t));
    println!("part 1 : {:?}", Field::new(vec1x, vec1y).draw_lines_unchecked(start1, lines.iter().map(|&t|t)).count_inside());
    let (start2, vec2x, vec2y) = compute_bounds(lines.iter().map(LineInfo::uncover));
    println!("part 2 : {:?}", Field::new(vec2x, vec2y).draw_lines_unchecked(start2, lines.iter().map(LineInfo::uncover)).count_inside());
}
