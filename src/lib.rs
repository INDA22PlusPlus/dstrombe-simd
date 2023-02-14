#![feature(portable_simd)]
pub mod mandelbrot;
use std::simd::f64x4;
use std::simd::u32x4;
use std::simd::{StdFloat, SimdPartialOrd, Mask};
use std::ops::{Mul, Add, Sub};





