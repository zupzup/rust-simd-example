#![feature(portable_simd)]
use num::SimdFloat;
use std::simd::*;

fn main() {
    let mut long_vec = vec![];
    for i in 0..(4096) {
        long_vec.push(i as f32);
    }
    // TODO: why do they have different results??
    // floating point arithmetic difference
    simple_calc(long_vec.as_slice());
    simple_calc_simd(long_vec.as_slice());
}

pub fn simple_calc_simd(input: &[f32]) {
    let multiplicator = f32x8::splat(1.3);
    let mut result: f32 = 0.0;
    let mut i = 0;
    let mut last = 0.0;
    input.chunks(8).for_each(|chunk| {
        i += 1;
        let arr = f32x8::from_slice(chunk);
        let mul = arr * multiplicator;
        // println!("simd: {}", result);
        result += mul.reduce_sum();
        last = *chunk.last().unwrap();
    });
    println!("last: {last} iters: {i} res simd: {result}");
}

pub fn simple_calc(input: &[f32]) {
    let multiplicator = 1.3;
    let mut result: f32 = 0.0;
    let mut i = 0;
    let mut last = 0.0;
    input.iter().for_each(|n| {
        i += 1;
        result += n * multiplicator;
        last = *n;
        // println!("simd: {}", result);
    });
    println!("last: {last} iters: {i} res: {result}");
}
