use itertools::Itertools;

#[inline]
pub fn run(input: &str) -> u64 {

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
        .max()
        .unwrap()
}