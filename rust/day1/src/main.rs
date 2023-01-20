use std::env;

mod part1;
mod part2;
/*
file = open("../../data/day1/input.txt", encoding="utf-8")
#file = open("test.txt", encoding="utf-8")
*/
fn main() {
    let file = env::args();

    part1::run();
}

fn parse(args: impl Iterator<item = String>) -> Option<String> {
    
}