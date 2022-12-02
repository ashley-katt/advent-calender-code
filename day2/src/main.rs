use std::{path::Path, fs};

#[derive(Debug)]
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
    fn score_against(&self, o: &Play) -> u64 {
        match self {
            Self::Rock => match o {
                Self::Rock => 3,
                Self::Scissors => 0,
                Self::Paper => 6,
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

    let mut score = 0;
    for data in file_data.unwrap().lines() {
        let mut sp = data.chars();
        let other_play = Play::from_char(sp.next().unwrap()).unwrap();
        sp.next();
        let my_play = Play::from_char(sp.next().unwrap()).unwrap();
        score += my_play.score() + my_play.score_against(&other_play);
    }
    println!("{}", score);
}