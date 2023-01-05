use std::{fs::File, io::BufReader, io::BufRead};

fn main() {
    let file = File::open("../../data/day3/input.txt").expect("Unable to read file");
    //let file = File::open("../../data/day3/test.txt").expect("Unable to read file");
    let mut file = BufReader::new(file);

    //let sum = part1(file);
    let sum = part2(&mut file);

    println!("{sum}");
}

fn part1(file: BufReader<File>) -> u32 {
    let mut sum = 0;

    for line in file.lines() {
        sum += priority_sum_rucksack(line.unwrap());
    }
    sum
}

fn part2(file: &mut BufReader<File>) -> u32 {
    let mut sum = 0;

    let mut lines = file.lines();

    loop {
        let line1 = match lines.next() {
            Some(line) => line.unwrap(),
            None => break,
        };
        let line2 = lines.next().unwrap().unwrap();
        let line3 = lines.next().unwrap().unwrap();
        sum += priority_sum_group(&line1, &line2, &line3);
    }
    sum
}

/*
a-z, 1-26
A-Z, 27-52

ascii dec
A-Z, 65-90
a-z, 97-122
*/
fn char_to_code(c: char) -> u32 {
    c as u32 - {
        if c.is_uppercase() { 38 }
        else { 96 }
    }
}

fn priority_sum_rucksack(line: String) -> u32 {
    //let mut sum = 0;

    let (first, second) = line.split_at(line.len()/2);
    for c in first.chars() {
        if second.contains(c) {
            return char_to_code(c)
        }
    }
    0
}

fn priority_sum_group(line1: &String, line2: &String, line3: &String) -> u32 {
    for c in line1.chars() {
        if line2.contains(c) && line3.contains(c) {
            return char_to_code(c)
        }
    }
    0
}