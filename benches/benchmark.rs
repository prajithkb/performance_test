use std::fmt::Display;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use performance_test::data_access;
use performance_test::Data;

#[macro_export]
macro_rules! bench_data {
    ($
        // Power of two in bytes
        id:expr,
        // Number items to iterate
        $num_items:expr,
        // the group
        $group: expr
    ) => {{
        const SIZE: usize = 2usize.pow($id - 1);
        println!("ID: {}", $id);
        const NUM_ITEMS: usize = $num_items;
        let data: Data<SIZE> = Data::create_list(NUM_ITEMS);
        $group.bench_with_input(BenchmarkId::from_parameter($id - 1), &data, |b, data| {
            b.iter(|| data_access(data));
        });
    }};
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Parameter {
    value: usize,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", memory(self.value)))
    }
}

pub fn thread_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Size in Bytes (power of 2)");
    // We check the performance for 1B, 2B, 4B, 8B, 16B ... 2^23B
    // bench_data!(1, 10000, group);
    // bench_data!(2, 10000, group);
    // bench_data!(3, 10000, group);
    // bench_data!(4, 10000, group);
    // bench_data!(5, 10000, group);
    // bench_data!(6, 10000, group);
    // bench_data!(7, 10000, group);
    // bench_data!(8, 10000, group);
    // bench_data!(9, 10000, group);
    // bench_data!(10, 10000, group);
    // bench_data!(11, 10000, group);
    // bench_data!(12, 10000, group);
    // bench_data!(13, 10000, group);
    // bench_data!(14, 10000, group);
    // bench_data!(15, 10000, group);
    // bench_data!(16, 10000, group);
    // bench_data!(17, 10000, group);
    // bench_data!(18, 10000, group);
    // bench_data!(19, 10000, group);
    // bench_data!(20, 10000, group);
    // bench_data!(21, 10000, group);
    // bench_data!(22, 10000, group);
    // bench_data!(23, 10000, group);
    bench_data!(24, 10000, group);

    group.finish();
}

criterion_group!(benches, thread_benchmark);
criterion_main!(benches);
