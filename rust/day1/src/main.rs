use day1::{part1, part2};

fn main(){
    //let input = std::fs::read_to_string("../../data/day1/input.txt")?;
    let input = include_str!("../../../data/day1/input.txt");

    let p1_answer = part1::run(input);
    println!("part 1 answer = {p1_answer:#?}");

    let p2_answer = part2::run_array(input);
    println!("part 2 answer = {p2_answer:#?}");
}