#![feature(portable_simd)]
use criterion::{criterion_group, criterion_main, Criterion};
use num::SimdFloat;
use std::simd::*;

pub fn simple_calc_simd(input: &[f32]) -> f32 {
    let multiplicator = f32x64::splat(10.0);
    let mut result: f32 = 0.0;
    input.chunks(64).for_each(|chunk| {
        let arr = f32x64::from_slice(chunk);
        let mul = arr * multiplicator;
        let sum = mul.reduce_sum();
        result += sum;
    });
    result
    // println!("res simd: {}", result);
}

pub fn simple_calc(input: &[f32]) -> f32 {
    let multiplicator = 10.0;
    let mut result: f32 = 0.0;
    input.iter().for_each(|n| {
        result += n * multiplicator;
    });
    result
    // println!("res: {}", result);
}

fn crit_bench(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("normal", |b| b.iter(|| simple_calc(slice)));
}

fn crit_bench_simd(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd", |b| b.iter(|| simple_calc_simd(slice)));
}

criterion_group!(benches, crit_bench, crit_bench_simd);
criterion_main!(benches);
