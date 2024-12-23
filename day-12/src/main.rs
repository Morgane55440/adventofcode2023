use std::{fs::read_to_string, usize, collections::HashMap};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown
}

impl State {
    fn parse(c : char) -> Option<Self> {
        Some(match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => return None
        })
    }
}


struct Arrangement {
    tiles : Vec<State>,
    counts : Vec<usize>
}

impl Arrangement {
    fn parse_line(s :&str) -> Option<Self> {
        let split = s.split_ascii_whitespace().collect::<Vec<&str>>();
        Some(Self {
            tiles : split.get(0)?.chars().map(State::parse).collect::<Option<_>>()? ,
            counts : split.get(1)?.split(',').map(|n| n.parse::<usize>().ok()).collect::<Option<_>>()?
        })
    }

    fn potential_number(&self) -> usize {
        let running_sumed = self.counts.iter().rev().scan(0 as usize, |state, &n| {
            if *state != 0 {
                *state += 1
            };
            *state += n;
            Some(*state)
        }).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();
        solve(
            &self.tiles,
            &running_sumed
        )
    }

    fn fast_potential_number(&self) -> usize {
        let running_sumed = self.counts.iter().rev().scan(0 as usize, |state, &n| {
            if *state != 0 {
                *state += 1
            };
            *state += n;
            Some(*state)
        }).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();
        fast_solve(
            &self.tiles,
            &running_sumed
        )
    }

    fn actually_fast_potential_number(&self) -> usize {
        let running_sumed = self.counts.iter().rev().scan(0 as usize, |state, &n| {
            if *state != 0 {
                *state += 1
            };
            *state += n;
            Some(*state)
        }).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();
        actually_fast_solve(
            &self.tiles,
            &running_sumed
        )
    }

    fn unfolded(&self) -> Self {
        let (mut tiles, mut counts) = (self.tiles.clone(), self.counts.clone());
        tiles.reserve_exact(4 * (self.tiles.len() + 1));
        counts.reserve_exact(4 * self.counts.len());
        for _ in 0..4 {
            tiles.push(State::Unknown);
            tiles.append(&mut self.tiles.clone());
            counts.append(&mut self.counts.clone());
        }
        Self { tiles, counts}
    }
}

//panics if a count is 0
fn solve(states : &[State], counts : &[usize]) -> usize {
    match (counts.get(0), states.len()) {
        (None, _) => states.iter().all(|x| *x != State::Damaged).then_some(1).unwrap_or(0),
        (Some(x), y) if y < *x => 0,
        (Some(x), _) => {
            let s = states.get(0).expect("only possible if a count is 0");
            (
                if *s != State::Damaged { solve(&states[1..], counts) } else { 0 }
            ) + (
                if *s != State::Operational { 
                    if states.iter().take(x - counts.get(1).map(|n| n + 1).unwrap_or(0)).all(|x| *x != State::Operational) && (counts.get(1).map(|n| states[x - n - 1] != State::Damaged).unwrap_or(true)) { 
                        solve(&states[(x - counts.get(1).map(|n| *n).unwrap_or(0))..], &counts[1..])
                    } else { 0 }
                } else { 0 }
            )
        },
    }
}

fn fast_solve(states : &[State], counts : &[usize]) -> usize {
    let left_index = (counts.len() != 0).then(||(counts.len() - 1) / 2).unwrap_or(0);
    if counts.len() <= 4 {
        solve(states, counts)
    } else {
        let (left_len, right_len) = (counts[left_index] - counts[left_index + 1] - 1, counts[left_index + 1] - counts[left_index + 2] - 1);
        let (left_counts, right_counts) = (&counts[..(left_index)].iter().map(|n| *n - counts[left_index] - 1).collect::<Vec<_>>(), &counts[(left_index + 2)..]);
        let mut res = 0;
        let (mut left_map, mut right_map) = (HashMap::<usize,usize>::new(), HashMap::<usize,usize>::new());
        for i in (*left_counts.last().expect("can't be empty"))..(states.len() - counts[left_index].min(states.len())) {
            if (states[i] != State::Damaged) && states[i + 1..].iter().take(left_len).all(|x| *x != State::Operational) &&  states[i + left_len + 1] != State::Damaged
            {
                for j in (i + left_len + 1)..(states.len() - counts[left_index + 1].min(states.len())) {
                    if states[j] != State::Damaged {
                        if states[j + 1..].iter().take(right_len).all(|x| *x != State::Operational) &&  states[j + 1 + right_len] != State::Damaged 
                        {
                            let left_solve = match left_map.get(&i).copied() {
                                Some(x) => x,
                                None => {
                                    let solve = fast_solve(&states[..i], left_counts);
                                    left_map.insert(i, solve);
                                    solve
                                }
                            };
                            let right_solve = match left_map.get(&j).copied() {
                                Some(x) => x,
                                None => {
                                    let solve = fast_solve(&states[j + right_len + 2..], right_counts);
                                    right_map.insert(j, solve);
                                    solve
                                }
                            };

                            res += left_solve * right_solve
                        } 
                    } else {
                        break;
                    }
                }
            }
        }
        res
    }
}

fn actually_fast_solve(states : &[State], counts : &[usize]) -> usize {
    if counts.len() <= 2{
        solve(states, counts)
    } else {
        let index = counts.len() / 2;
        let len = counts[index] - counts[index + 1] - 1;
        let (left_counts, right_counts) = (
            &counts[..(index)].iter().map(|n| *n - counts[index] - 1).collect::<Vec<_>>(), 
            &counts[(index + 1)..]
        );
        let mut res: usize = 0;
        for i in (*left_counts.last().expect("can't be empty"))..(states.len() - counts[index].min(states.len())) {
            if (states[i] != State::Damaged) && states[i + 1..].iter().take(len).all(|x| *x != State::Operational) &&  states[i + len + 1] != State::Damaged {
                res += fast_solve(&states[..i], left_counts) * fast_solve(&states[i + len + 2..], right_counts)
            }
        }
        res
    }
}
fn main() {
    let arrangements = read_to_string("input.txt").unwrap_or("".to_string()).lines().map(Arrangement::parse_line).collect::<Option<Vec<_>>>().expect("input is malformed");
    arrangements.iter().map(Arrangement::fast_potential_number).zip(
        arrangements.iter().map(Arrangement::actually_fast_potential_number)
    ).enumerate().for_each(|(index,(expected, got))|{
        if expected != got { println!("at {index}, expected {expected} but got {got}")}
    });

    println!("part 1 : {}", arrangements.iter().map(Arrangement::potential_number).sum::<usize>());
    println!("part 1 : {}", arrangements.iter().map(Arrangement::fast_potential_number).sum::<usize>());
    println!("part 2 : {}", arrangements.iter().map(Arrangement::unfolded).enumerate().map(|(n, arr)| {
        println!("{}", n);
        /* 
        let fast = arr.fast_potential_number();
        let actually = arr.actually_fast_potential_number();
        if fast != actually {
            println!("at {n}, expected {fast} but got {actually}")
        }*/
        arr.actually_fast_potential_number()
    }).sum::<usize>());
}
