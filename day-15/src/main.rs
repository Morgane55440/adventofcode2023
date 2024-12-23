use std::{fs::read_to_string, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum SignType {
    Dash,
    Equal(u32)
}

#[derive(Debug, Clone)]
struct Sign {
    hash : u8,
    label : Vec<u8>,
    sgn_type :  SignType
}

impl PartialEq for Sign {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

impl Eq for Sign {}

impl Sign {
    fn parse<I>(iter : I) -> Option<Self>
    where
        I : Iterator<Item=u8>
    {
        let binding = iter.collect::<Vec<u8>>();
        let mut separator : u8 = 0;
        let parts= binding.split(|c| if *c == b'-' || *c == b'=' {separator = *c;true} else {false}).collect::<Vec<&[u8]>>();
        let hash = parts.get(0)?.iter().fold::<u8,_>(0, |acc, v| acc.wrapping_add(*v).wrapping_mul(17));
        let label = parts.get(0)?.iter().map(|c| *c).collect();
        match separator {
            b'-' => Some(Sign { hash, label, sgn_type: SignType::Dash }),
            b'=' => Some(Sign { hash, label,  sgn_type: SignType::Equal(parts.get(1)?.iter().fold::<Option<u32>,_>(Some(0) , |acc, v| (v.checked_rem(b'0')? as u32).checked_add(acc?.checked_mul(10)?))?) }),
            _ => None
        }

    }
}







fn main() {
    println!("part 1 : {}", read_to_string("input.txt").unwrap().bytes().group_by(|c| *c != b',' && *c != b'\n').into_iter().filter_map(|(b,v)| b.then_some(v).map(|iter|iter.fold::<u8,_>(0, |acc, v| acc.wrapping_add(v).wrapping_mul(17)) as u64)).sum::<u64>());

    let signs = read_to_string("input.txt").unwrap().bytes().group_by(|c| *c != b',' && *c != b'\n').into_iter().filter_map(|(b,v)| b.then_some(v).map(Sign::parse)).collect::<Option<Vec<_>>>().unwrap();
    println!("part 2 : {}", signs.into_iter().fold(HashMap::<u8, Vec<Sign>>::new(), |mut map, s| {
        match s.sgn_type {
            SignType::Dash => {
                let vec = map.entry(s.hash).or_default();
                let find = vec.iter().position(|c| *c == s);
                find.map(|n| vec.remove(n));
            },
            SignType::Equal(_) => {
                let vec = map.entry(s.hash).or_default();
                match vec.iter().position(|c| *c == s) {
                    Some(n) => { vec[n] = s; },
                    None => vec.push(s)
                }
            },
        };
        map
    }).into_iter().map(|(k,v)|
        v.into_iter().enumerate().map(|(index, sign)| (index + 1) * match sign.sgn_type { SignType::Dash => 0, SignType::Equal(n) => n as usize}).sum::<usize>() * (k as usize + 1)
    ).sum::<usize>())
}
