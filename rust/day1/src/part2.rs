use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

/*
🏆
Criterion results:
time:   [19.448 µs 19.540 µs 19.623 µs]
*/
#[inline(never)]
pub fn run_array(input: &str) -> u64 {

    let sums = input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        });

    let mut largest = [0; 3];
    for sum in sums {
        if sum > largest[0] {
            largest[0] = sum;
            largest.sort()
        }
    }
    largest.iter().sum()
}

/*
Criterion results:
time:   [26.489 µs 26.577 µs 26.653 µs]
*/
#[inline(never)]
pub fn run_full_sort(input: &str) -> u64 {

    input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .sorted_by_key(|x| Reverse(*x))
        .take(3)
        .sum()
}

/*
Criterion results:
time:   [24.240 µs 24.272 µs 24.307 µs]
*/
#[inline(never)]
pub fn run_binary_heap(input: &str) -> u64 {

    let mut sums = input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .map(Reverse);
        
    let mut largest = BinaryHeap::new();
        
    for sum in (&mut sums).take(3) {
        largest.push(sum);
    }

    for rest in sums {
        largest.push(rest);
        largest.pop();
    }

    largest.iter().map(|Reverse(v)| v).sum()
}
