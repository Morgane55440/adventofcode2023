use std::{fs::read_to_string, collections::HashMap};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Movable,
    UnMovable
}

impl Rock {
    fn parse(c : char) -> Option<Self> {
        Some(match c {
            'O' => Self::Movable,
            '#' => Self::UnMovable,
            _ => None?
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    tiles : Vec<Vec<Option<Rock>>>
}

impl Platform {
    
    fn parse<'a, I>(iter : I) -> Option<Self>
    where
        I : Iterator<Item = &'a str>
    {
        let tiles = iter.map(|line| line.chars().map(Rock::parse).collect::<Vec<_>>()).collect::<Vec<_>>();
        tiles.iter().all(|line| line.len() == tiles[0].len()).then_some(())?;
        Some(Self{ tiles })
    }

    fn tilt_north(&mut self) -> &Self {
        let row_nb = self.tiles.len();
        let col_nb = self.tiles.get(0).map(|v| v.len()).unwrap_or(0);
        for i in 0..col_nb {
            let mut fall_spot : Option<usize> = None;
            for j in 0..row_nb {
                match self.tiles[j][i] {
                    Some(Rock::UnMovable) => { fall_spot = None }
                    None => { fall_spot.get_or_insert(j); }
                    Some(Rock::Movable) => { 
                        if let Some(spot) = fall_spot.as_mut() {
                            self.tiles[*spot][i] = Some(Rock::Movable);
                            self.tiles[j][i] = None;
                            *spot += 1;
                        }
                    }
                }
            }
        }
        self
    }

    fn tilt_south(&mut self) -> &Self {
        let row_nb = self.tiles.len();
        let col_nb = self.tiles.get(0).map(|v| v.len()).unwrap_or(0);
        for i in 0..col_nb {
            let mut fall_spot : Option<usize> = None;
            for j in (0..row_nb).rev() {
                match self.tiles[j][i] {
                    Some(Rock::UnMovable) => { fall_spot = None }
                    None => { fall_spot.get_or_insert(j); }
                    Some(Rock::Movable) => { 
                        if let Some(spot) = fall_spot.as_mut() {
                            self.tiles[*spot][i] = Some(Rock::Movable);
                            self.tiles[j][i] = None;
                            *spot -= 1;
                        }
                    }
                }
            }
        }
        self
    }

    fn tilt_east(&mut self) -> &Self {
        let row_nb = self.tiles.len();
        let col_nb = self.tiles.get(0).map(|v| v.len()).unwrap_or(0);
        for i in 0..row_nb {
            let mut fall_spot : Option<usize> = None;
            for j in (0..col_nb).rev() {
                match self.tiles[i][j] {
                    Some(Rock::UnMovable) => { fall_spot = None }
                    None => { fall_spot.get_or_insert(j); }
                    Some(Rock::Movable) => { 
                        if let Some(spot) = fall_spot.as_mut() {
                            self.tiles[i][*spot] = Some(Rock::Movable);
                            self.tiles[i][j] = None;
                            *spot -= 1;
                        }
                    }
                }
            }
        }
        self
    }

    fn tilt_west(&mut self) -> &Self {
        let row_nb = self.tiles.len();
        let col_nb = self.tiles.get(0).map(|v| v.len()).unwrap_or(0);
        for i in 0..row_nb {
            let mut fall_spot : Option<usize> = None;
            for j in 0..col_nb {
                match self.tiles[i][j] {
                    Some(Rock::UnMovable) => { fall_spot = None }
                    None => { fall_spot.get_or_insert(j); }
                    Some(Rock::Movable) => { 
                        if let Some(spot) = fall_spot.as_mut() {
                            self.tiles[i][*spot] = Some(Rock::Movable);
                            self.tiles[i][j] = None;
                            *spot += 1;
                        }
                    }
                }
            }
        }
        self
    }

    fn cycle(&mut self) -> &Self {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
        self
    }


    fn load_south(&self) -> usize {
        self.tiles.iter().enumerate().map(|(i, v)| {
            (self.tiles.len() - i) * v.iter().filter(|x| *x == &Some(Rock::Movable)).count()
        }).sum()
    }

}


fn do_the_billion(p : &Platform) -> usize {
    let mut platform = p.clone();
    let mut loop_count = 0;
    let mut encountered_maps = HashMap::<Platform, usize>::new();
    encountered_maps.insert(p.clone(), 0);
    while loop_count < 1_000_000_000 {
        platform.cycle();
        loop_count += 1;
        if let Some(prev_index) = encountered_maps.get(&platform) {
            let reset_len = loop_count - prev_index;
            loop_count += ((1_000_000_000 - loop_count) / reset_len) * reset_len;
        }
        encountered_maps.insert(platform.clone(), loop_count);
        println!("{} loops", loop_count)
    }
    platform.load_south()
}


fn main() {
    println!("{:?}", Platform::parse(read_to_string("input.txt").unwrap().lines()).unwrap());
    println!("part 1 : {}", Platform::parse(read_to_string("input.txt").unwrap().lines()).unwrap().tilt_north().load_south());
    println!("part 2 : {}", do_the_billion(&Platform::parse(read_to_string("input.txt").unwrap().lines()).unwrap()))
}
