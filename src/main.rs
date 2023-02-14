#![feature(portable_simd)]
use dstrombe_simd::mandelbrot::{render_mandelbrot, ComplexNum};
use std::simd::f64x4;
use std::simd::u32x4;
use std::simd::{StdFloat, SimdPartialOrd, Mask};
use std::ops::{Mul, Add, Sub};

fn main() {
    let pix_count = 2048;
    println!("P3");
    println!("{} {}", pix_count, pix_count);
    println!("255");
    let convergences = render_mandelbrot(true, ComplexNum {
        re : -1.0,
        im : -1.0,
    }, 2.0, pix_count, 200);
        for i in convergences {
        let a = (i * 255.0) as i32;
        println!("{} {} {}", a, a, a);    
    }
}
