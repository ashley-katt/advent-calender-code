use std::{path::Path, fs, borrow::BorrowMut};


fn main() {
    let p = Path::new("./input.txt");
    let file_data = fs::read_to_string(p);
    
    puzzle1(file_data.as_ref().unwrap());
    puzzle2(file_data.as_ref().unwrap());
}


fn puzzle1(file_data: &String) {
    let mut lines = file_data.lines().into_iter().peekable();
    let stack_count = lines.peek().unwrap().len()/4usize;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    stacks.resize_with(stack_count, || vec![]);

    init_stacks(&mut stacks, &mut lines);
    for l in lines {
        let sp: Vec<&str> = l.split(" ").collect();
        let amnt = sp[1].parse::<usize>().unwrap();
        let from = sp[3].parse::<usize>().unwrap() - 1;
        let to = sp[5].parse::<usize>().unwrap() - 1;
        for _ in 0..amnt {
            if let Some(e) = stacks.get_mut(from).unwrap().pop() {
                stacks.get_mut(to).unwrap().push(e);
            }
        }
    }
    let mut answer = String::with_capacity(stacks.len());
    for l in stacks {
        answer.push(l[l.len()-1]);
    }
    println!("{:?}", answer);
}


fn puzzle2(file_data: &String) {
    let mut lines = file_data.lines().into_iter().peekable();
    let stack_count = lines.peek().unwrap().len()/4usize;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
    stacks.resize_with(stack_count, || vec![]);

    init_stacks(&mut stacks, &mut lines);
    for l in lines {
        let sp: Vec<&str> = l.split(" ").collect();
        let amnt = sp[1].parse::<usize>().unwrap();
        let from = sp[3].parse::<usize>().unwrap() - 1;
        let to = sp[5].parse::<usize>().unwrap() - 1;
        let mut temp = Vec::with_capacity(amnt);
        for _ in 0..amnt {
            if let Some(e) = stacks.get_mut(from).unwrap().pop() {
                temp.push(e);
            }
        }
        temp.reverse();
        stacks.get_mut(to).unwrap().append(&mut temp);
    }
    let mut answer = String::with_capacity(stacks.len());
    for l in stacks {
        answer.push(l[l.len()-1]);
    }
    println!("{:?}", answer);
}

fn init_stacks(v: &mut Vec<Vec<char>>, it: &mut dyn Iterator<Item = &str>) {
    for line in it {
        if line.starts_with('E') {
            break;
        }
        let mut l = line.chars();
        let mut index = 0;
        while let Some(n) = l.next() {
            if n == '[' {
                v[index].push(l.next().unwrap()); // Next character is relevant
                l.next(); l.next(); // Skip the "[" and " "
            } else {
                l.next(); l.next(); l.next(); // Skip the next 3 spaces
            }
            index+=1;
        }
    }
    for stack in v.iter_mut() {
        stack.reverse();
    }
}