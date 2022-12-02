use std::{path::Path, fs};

fn main() {
    let p = Path::new("./day1input.txt");
    let fileData = fs::read_to_string(p);

    let mut top = Vec::with_capacity(4);
    let mut current_num = 0u64;

    for num in fileData.unwrap().lines().into_iter() {
        match num.parse::<u64>() {
            Ok(n) => {
                current_num += n;
            }
            Err(_) => {
                top.push(current_num);
                top.sort();
                if top.len() >= 4 {
                    top.remove(0);
                }
                current_num = 0;
            }
        }
    }
    println!("{}", top[0] + top[1] + top[2]);
}
















/*

Wimp version

fn main() {
    let p = Path::new("./day1input.txt");
    let fileData = fs::read_to_string(p);
    let mut max_num = 0u64;
    let mut current_num = 0u64;
    for num in fileData.unwrap().lines().into_iter() {
        match num.parse::<u64>() {
            Ok(n) => {
                current_num += n;
            }
            Err(_) => {
                if current_num > max_num {
                    max_num = current_num;
                }
                current_num = 0;
            }
        }
    }
    println!("{}", max_num);
}

*/