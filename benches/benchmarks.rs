use rust_speed_tester::{fibonacci, merge_sort};

fn main() {
    divan::main();
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn bench_fibonacci(n: u64) -> u64 {
    fibonacci(n)
}

#[divan::bench(args = [100, 1000, 10000])]
fn bench_merge_sort(n: usize) -> Vec<i64> {
    let data: Vec<i64> = (0..n as i64).rev().collect();
    merge_sort(data)
}
