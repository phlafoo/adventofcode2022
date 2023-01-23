use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::part2;
/*
cargo bench --bench my_benchmark -- --save-baseline base
*/
fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../../../data/day1/input.txt");
    let mut group = c.benchmark_group("part2");
    group.bench_function("part2 array", |b| b.iter(|| part2::run_array(black_box(input))));
    group.bench_function("part2 binary heap", |b| b.iter(|| part2::run_binary_heap(black_box(input))));
    group.bench_function("part2 full sort", |b| b.iter(|| part2::run_full_sort(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);