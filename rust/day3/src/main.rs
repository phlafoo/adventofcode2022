use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    let file = File::open("../../data/day3/input.txt").expect("Unable to read file");
    //let file = File::open("../../data/day3/test.txt").expect("Unable to read file");
    let file = BufReader::new(file);

    let mut sum = 0;

    for line in file.lines() {
        sum += priority_sum_rucksack(line.unwrap());
    }
    println!("{sum}")
}

/*
a-z, 1-26
A-Z, 27-52

ascii dec
A-Z, 65-90
a-z, 97-122
*/

fn priority_sum_rucksack(line: String) -> u32 {
    let mut sum = 0;

    let (first, second) = line.split_at(line.len()/2);
    for c in first.chars() {
        if second.contains(c) {
            sum += c as u32 - {
                if c.is_uppercase() { 38 }
                else { 96 }
            };
            break;
        }
    }
    sum
}