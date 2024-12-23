
struct Race {
    time : usize,
    distance : usize
}

impl Race {
    fn lazy_ways_to_beat(&self) -> usize {
        (0..=self.time).filter_map(|x| {
            (x * (self.time - x) >= self.distance).then_some(0)
        }).count()
    }
    fn fast_ways_to_beat(&self) -> usize {
        let a : f64 = -1 as f64;
        let b : f64 = self.time as f64;
        let c : f64 = -(self.distance as f64);
        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 { return 0;}
        let min = ((b - delta.sqrt()) / 2.0).ceil() as usize;
        let max = ((b + delta.sqrt()) / 2.0).floor() as usize;
        max + 1 - min
    }
}



fn main() {
    let input = vec![
        Race { time : 41, distance : 214},
        Race { time : 96, distance : 1789},
        Race { time : 88, distance : 1127},
        Race { time : 94, distance : 1055}
    ];
    println!("part 1 : {}", input.iter().map(Race::fast_ways_to_beat).fold(1, |a,b| a*b));
    println!("part 2 : {}", Race { time : 41968894, distance : 214178911271055}.fast_ways_to_beat());


}
