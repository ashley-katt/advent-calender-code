use std::{path::Path, fs};

#[derive(Debug)]
struct Cargo {
    first: String,
    second: String
}
impl Cargo {
    fn get_common(&self) -> char {
        for c in self.first.chars() {
            if self.second.contains(c) {
                return c;
            }
        };
        unreachable!();
    }
    fn from_str(s: &str) -> Self {
        let mut first = String::new();
        let mut second = String::new();
        for (i, c) in s.chars().enumerate() {
            if i < s.len()/2 {
                first.push(c);
            } else {
                second.push(c);
            }
        };
        Self { first, second }
    }
}

fn get_score(c: char) -> u8 {
    match c as u8 {
        t @ b'a'..=b'z' => t-b'a'+1,
        t @ b'A'..=b'Z' => t-b'A'+27,
        _ => 0
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
        let cargo = Cargo::from_str(line);
        //println!("{:?}, {:?}, {:?}, {:?}", line, cargo, cargo.get_common(), get_score(cargo.get_common()));
        score += get_score(cargo.get_common()) as u64;
    }
    println!("score: {}", score);
}


fn puzzle2(file_data: &String) {
    let mut lines = file_data.lines();
    let mut score = 0u64;
    while let Some(m) = lines.next() {
        score += get_score(common_char(m, lines.next().unwrap(), lines.next().unwrap())) as u64;
    }
    println!("group score: {}", score);
}

fn common_char(a: &str, b: &str, c: &str) -> char {
    for d in a.chars() {
        if b.contains(d) && c.contains(d) {
            return d;
        }
    }
    unreachable!();
}