use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(day01::solve));
}

pub fn day02_benchmark(c: &mut Criterion) {
    c.bench_function("day02", |b| b.iter(day02::solve));
}

pub fn day03_benchmark(c: &mut Criterion) {
    c.bench_function("day03", |b| b.iter(day03::solve));
}

pub fn day04_benchmark(c: &mut Criterion) {
    c.bench_function("day04", |b| b.iter(day04::solve));
}

pub fn day05_benchmark(c: &mut Criterion) {
    c.bench_function("day05", |b| b.iter(day05::solve));
}

pub fn day06_benchmark(c: &mut Criterion) {
    c.bench_function("day06", |b| b.iter(day06::solve));
}

pub fn day07_benchmark(c: &mut Criterion) {
    c.bench_function("day07", |b| b.iter(day07::solve));
}

pub fn day08_benchmark(c: &mut Criterion) {
    c.bench_function("day08", |b| b.iter(day08::solve));
}

pub fn day09_benchmark(c: &mut Criterion) {
    c.bench_function("day09", |b| b.iter(day09::solve));
}

pub fn day10_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day10");
    group.sample_size(10);
    group.bench_function("solve", |b| b.iter(day10::solve));
    group.finish()
}

pub fn day11_benchmark(c: &mut Criterion) {
    c.bench_function("day11", |b| b.iter(day11::solve));
}

pub fn day12_benchmark(c: &mut Criterion) {
    c.bench_function("day12", |b| b.iter(day12::solve));
}

pub fn day15_benchmark(c: &mut Criterion) {
    c.bench_function("day15", |b| b.iter(day15::solve));
}

pub fn day16_benchmark(c: &mut Criterion) {
    c.bench_function("day16", |b| b.iter(day16::solve));
}

pub fn day17_benchmark(c: &mut Criterion) {
    c.bench_function("day17", |b| b.iter(day17::solve));
}

pub fn day18_benchmark(c: &mut Criterion) {
    c.bench_function("day18", |b| b.iter(day18::solve));
}

pub fn day19_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day19");
    group.sample_size(10);
    group.sampling_mode(criterion::SamplingMode::Flat);

    group.bench_function("parsing", |b| b.iter(day19::load_input));

    let input = day19::load_input();

    group.bench_function("part1", |b| {
        b.iter(|| day19::solve_part1(input.0, &input.1))
    });

    group.bench_function("part2", |b| {
        b.iter(|| day19::solve_part2(input.0, &input.1))
    });

    group.bench_function("solve", |b| b.iter(day19::solve));

    group.finish();
}

pub fn day25_benchmark(c: &mut Criterion) {
    c.bench_function("day25", |b| b.iter(day25::solve));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            (
                day01::solve(),
                day02::solve(),
                day03::solve(),
                day04::solve(),
                day05::solve(),
                day06::solve(),
                day07::solve(),
                day08::solve(),
                day09::solve(),
                day10::solve(),
                day11::solve(),
                day12::solve(),
                day15::solve(),
                day16::solve(),
                day17::solve(),
                day18::solve(),
                day19::solve(),
            )
        })
    });
}

criterion_group! {
    name = benches;

    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(350)
        .measurement_time(Duration::from_secs(30 / 3))
        .warm_up_time(Duration::from_secs(15 / 3))
        .noise_threshold(0.05);

    targets =
        day01_benchmark,
        day02_benchmark,
        day03_benchmark,
        day04_benchmark,
        day05_benchmark,
        day06_benchmark,
        day07_benchmark,
        day08_benchmark,
        day09_benchmark,
        day10_benchmark,
        day11_benchmark,
        day12_benchmark,
        day15_benchmark,
        day16_benchmark,
        day17_benchmark,
        day18_benchmark,
        day19_benchmark,
        day25_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
