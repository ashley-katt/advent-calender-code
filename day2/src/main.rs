use std::{path::Path, fs};

#[derive(Debug, Copy, Clone)]
enum Play {
    Rock,
    Scissors,
    Paper
}
impl Play {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn wins(&self) -> Play {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    fn loses(&self) -> Play {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn score_against(&self, o: &Play) -> u64 {
        match self {
            Self::Rock => match o {
                Self::Rock => 3,
                Self::Scissors => 6,
                Self::Paper => 0,
            },
            Self::Scissors => match o {
                Self::Rock => 0,
                Self::Scissors => 3,
                Self::Paper => 6,
            },
            Self::Paper => match o {
                Self::Rock => 6,
                Self::Scissors => 0,
                Self::Paper => 3,
            },
        }
    }
    fn from_char(c: char) -> Option<Self> {
        if c=='A' || c=='X' {
            Some(Self::Rock)
        } else if c=='B' || c=='Y' {
            Some(Self::Paper)
        } else if c=='C' || c=='Z' {
            Some(Self::Scissors)
        } else {
            None
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
    let mut score = 0;
    for data in file_data.lines() {
        let mut sp = data.chars();
        let other_play = Play::from_char(sp.next().unwrap()).unwrap();
        sp.next();
        let my_play = Play::from_char(sp.next().unwrap()).unwrap();
        score += my_play.score() + my_play.score_against(&other_play);
    }
    println!("Puzzle 1: {}", score);
}


fn puzzle2(file_data: &String) {
    let mut score = 0;
    for data in file_data.lines() {
        let mut sp = data.chars();
        let other_play = Play::from_char(sp.next().unwrap()).unwrap();
        sp.next();
        match sp.next() {
            Some('X') => { 
                score += other_play.wins().score()
            }
            Some('Y') => { 
                score += 3 + other_play.score();
                
            }
            Some('Z') => { 
                score += 6 + other_play.loses().score();
            }
            _ => unreachable!(),
        };
    }
    println!("Puzzle 2: {}", score);
}