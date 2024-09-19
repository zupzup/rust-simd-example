#![feature(portable_simd)]
use num::SimdFloat;
use std::simd::*;

fn main() {
    let mut long_vec = vec![];
    for i in 0..(64 * 100) {
        long_vec.push(i as f32);
    }
    // TODO: why do they have different results??
    simple_calc(long_vec.as_slice());
    simple_calc_simd(long_vec.as_slice());
}

pub fn simple_calc_simd(input: &[f32]) {
    let multiplicator = f32x64::splat(10.0);
    let mut result: f32 = 0.0;
    input.chunks(64).for_each(|chunk| {
        let arr = f32x64::from_slice(chunk);
        let mul = arr * multiplicator;
        let sum = mul.reduce_sum();
        result += sum;
    });
    println!("res simd: {}", result);
}

pub fn simple_calc(input: &[f32]) {
    let multiplicator = 10.0;
    let mut result: f32 = 0.0;
    input.iter().for_each(|n| {
        result += n * multiplicator;
    });
    println!("res: {}", result);
}
