use adventof2024::day_tasks;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("real");
    let tasks = day_tasks();
    for (name, func) in tasks.into_iter() {
        let mut split = name.split("-");
        let day = split.next().unwrap();
        let part = split.next().unwrap();

        let contents = fs::read_to_string(format!("./input/{}/real.txt", day)).unwrap();
        group.bench_function(format!("{}-{}", day, part), |b| {
            b.iter(|| func(black_box(contents.clone())))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
