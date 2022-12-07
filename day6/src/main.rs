use std::{path::Path, fs, borrow::BorrowMut};


fn main() {
    let p = Path::new("./input.txt");
    let file_data = fs::read_to_string(p);
    
    scan_for_packet(file_data.as_ref().unwrap(), 4);
    scan_for_packet(file_data.as_ref().unwrap(), 14);
}

// Scan for a run of {size} distinct characters, and print the position of the last one
fn scan_for_packet(file_data: &String, size: usize) {
    let chars: Vec<char> = file_data.chars().collect();
    let mut index = 0;

    while !packet_start(&chars[index..index+size]) {
        index+=1;
    }
    println!("{}", index+size);
}

// Take in an array of chars and return whether there are duplicate chars or not
fn packet_start(s: &[char]) -> bool {
    let mut index = 0;
    while index < s.len() {
        if s[index+1..].contains(&s[index]) {
            return false;
        }
        index += 1;
    }
    return true;
}