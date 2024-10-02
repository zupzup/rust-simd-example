#![feature(portable_simd)]
use criterion::{criterion_group, criterion_main, Criterion};
use num::SimdFloat;
use std::simd::*;

pub fn simple_calc_simd<const N: usize>(input: &[f32]) -> f32
where
    LaneCount<N>: SupportedLaneCount,
{
    let multiplicator: Simd<f32, N> = Simd::splat(10.0);
    let mut result: f32 = 0.0;
    input.chunks(N).for_each(|chunk| {
        let arr: Simd<f32, N> = Simd::from_slice(chunk);
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

fn crit_bench_simd_n_1(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 1", |b| b.iter(|| simple_calc_simd::<1>(slice)));
}

fn crit_bench_simd_n_2(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 2", |b| b.iter(|| simple_calc_simd::<2>(slice)));
}

fn crit_bench_simd_n_4(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 4", |b| b.iter(|| simple_calc_simd::<4>(slice)));
}

fn crit_bench_simd_n_8(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 8", |b| b.iter(|| simple_calc_simd::<8>(slice)));
}

fn crit_bench_simd_n_16(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 16", |b| b.iter(|| simple_calc_simd::<16>(slice)));
}

fn crit_bench_simd_n_32(c: &mut Criterion) {
    let mut long_vec = vec![];
    for i in 0..(64 * 1000000) {
        long_vec.push(i as f32);
    }
    let slice = long_vec.as_slice();
    c.bench_function("simd 32", |b| b.iter(|| simple_calc_simd::<32>(slice)));
}

criterion_group!(
    benches,
    crit_bench,
    crit_bench_simd_n_1,
    crit_bench_simd_n_2,
    crit_bench_simd_n_4,
    crit_bench_simd_n_8,
    crit_bench_simd_n_16,
    crit_bench_simd_n_32
);
criterion_main!(benches);
