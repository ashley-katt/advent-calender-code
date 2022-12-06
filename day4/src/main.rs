use std::{path::Path, fs};

struct Range {
    min: u64,
    max: u64
}
impl Range {
    fn is_contained(&self, o: &Self) -> bool {
        self.min >= o.min && self.max <= o.max
    }
    fn overlaps(&self, o: &Self) -> bool {
        (self.max >= o.min && self.max <= o.max) || (self.min >= o.min && self.min <= o.max)
    }
    fn from_str(o: &str) -> Self {
        let mut s = o.split("-").into_iter();
        Self {
            min: s.next().unwrap().parse().unwrap(),
            max: s.next().unwrap().parse().unwrap()
        }
    }
}

fn main() {
    let p = Path::new("./input.txt");
    let file_data = fs::read_to_string(p);
    
    puzzle1(file_data.as_ref().unwrap());
    puzzle2(file_data.as_ref().unwrap());
}


fn puzzle1(file_data: &String) {
    let mut score = 0u64;
    for line in file_data.lines().into_iter() {
        let mut sp = line.split(",").into_iter();
        let a = Range::from_str(sp.next().unwrap());
        let b = Range::from_str(sp.next().unwrap());
        if a.is_contained(&b) || b.is_contained(&a) {
            score+=1;
        }
    }
    println!("Number of fully contained: {}", score);
}


fn puzzle2(file_data: &String) {
    let mut score = 0u64;
    for line in file_data.lines().into_iter() {
        let mut sp = line.split(",").into_iter();
        let a = Range::from_str(sp.next().unwrap());
        let b = Range::from_str(sp.next().unwrap());
        if a.overlaps(&b) || b.overlaps(&a) {
            score+=1;
        }
    }
    println!("Number of overlaps: {}", score);
}

