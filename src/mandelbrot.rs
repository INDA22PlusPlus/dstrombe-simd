#![feature(portable_simd)]

use std::simd::f32x4;
use std::simd::u32x4;
use std::simd::{StdFloat, SimdPartialOrd, Mask};
use std::ops::{Mul, Add, Sub};

pub struct ComplexNum {
    pub re : f32,
    pub im : f32,
}

struct ComplexNum4 {
    pub re : f32x4,
    pub im : f32x4,
}


pub fn render_mandelbrot(use_simd : bool, from_point : ComplexNum, box_size : f32, pix_count : u32, its_per_pixel : u32) -> Vec<f32> {
    let mut convergences : Vec<f32> = Vec::new();
    if(use_simd) {
        for px in (0u32..pix_count*pix_count).step_by(4) {
            let pix : u32x4 = u32x4::from_array([px, px + 1, px + 2, px + 3]);
            let pix_count = u32x4::splat(pix_count);
            let box_size = f32x4::splat(box_size);
            let re_offset = box_size * (pix % pix_count).cast::<f32>() / pix_count.cast::<f32>();
            let im_offset = box_size * (pix_count - (pix / pix_count)).cast::<f32>() / pix_count.cast::<f32>(); // top -> bottom

            let c = ComplexNum4 {
                re : f32x4::splat(from_point.re) + re_offset,
                im : f32x4::splat(from_point.im) + im_offset,
            };
                    
            let convergence = convergence_simd(its_per_pixel, c);
            convergences.extend_from_slice(&convergence);
        
        }
    }
    else {
        for pix in (0..pix_count*pix_count) {
            let re_offset = box_size * (pix % pix_count) as f32 / pix_count as f32;
            let im_offset = box_size * (pix_count - (pix / pix_count)) as f32 / pix_count as f32; // top -> bottom

            let c = ComplexNum {
                re : from_point.re + re_offset as f32,
                im : from_point.im + im_offset as f32,
            };
                    
            let convergence = convergence_sisd(its_per_pixel, c);
            convergences.push(convergence);
            //println!("{} {} {}", f32::min(255.0 * (convergence / 1.0), 255.0) as u32, f32::min(255.0 * (convergence / 0.66), 255.0) as u32, f32::min(255.0 * (convergence / 0.33), 255.0) as u32);    
        }
    }
    convergences 
}



// test c with sisd/conventional isa
fn convergence_sisd(its_per_pixel : u32, c : ComplexNum) -> f32 {
    let mut last_f_c = ComplexNum {
        re : 0.0,
        im : 0.0,
    };

    let mut convergence : f32 = 0.0;
    // actual loop that tests a point c 
    for it in 0..its_per_pixel {
        // divergence found, record how long it took to diverge (uncomment below for nicer pics)
        //if (f32::sqrt(last_f_c.re * last_f_c.re + last_f_c.im * last_f_c.im) > 2.0) {
        //    convergence = it as f32 / its_per_pixel as f32;
        //}
        last_f_c = f_c(last_f_c, &c);
    }
    if (f32::sqrt(last_f_c.re * last_f_c.re + last_f_c.im * last_f_c.im) < 2.0) {
        convergence = 1.0;
    }
    convergence
}

// f_c(z) = z² + c. 
// if f_c(f_c...f_c(0)...)) converges, c is part of the mandelbrot set
// |f_c(z)| > 2 at at point -> f_c diverges
fn f_c(z : ComplexNum, c : &ComplexNum) -> ComplexNum {
    
    // (x + bi)^2 = x² + 2xbi - b²
    let z_squared = ComplexNum {
        re : z.re * z.re - z.im * z.im,
        im : 2.0 * z.re * z.im 
    };

    ComplexNum {
        re : z_squared.re + c.re,
        im : z_squared.im + c.im,
    }
}

// test c with simd isa
fn convergence_simd(its_per_pixel : u32, c : ComplexNum4) -> [f32; 4] {
    let mut last_f_c = ComplexNum4 {
        re : f32x4::splat(0.0),
        im : f32x4::splat(0.0),
    };

    //let mut simd_divergence_cmp : Mask<f32, 4> = Mask::splat(false);
    let mut convergence = [0.0, 0.0, 0.0, 0.0];
    // actual loop that tests a point c 
    for it in (0..its_per_pixel*4).step_by(4) {
        // divergence found, record how long it took to diverge
        last_f_c = f_c_simd(last_f_c, &c);
    }
    
    let magnitudes_simd = (last_f_c.re * last_f_c.re + last_f_c.im * last_f_c.im).sqrt();

    for (idx, item) in magnitudes_simd.to_array().into_iter().enumerate() {  
        if item < 2.0 {
            convergence[idx] = 1.0;
        }
    }
    convergence
}

// f_c(z) = z² + c. 
// if f_c(f_c...f_c(0)...)) converges, c is part of the mandelbrot set
// |f_c(z)| > 2 at at point -> f_c diverges
fn f_c_simd(z : ComplexNum4, c : &ComplexNum4) -> ComplexNum4 {
    //let x = f32x4::from_array([z.re, 0.5, 0.6, -1.5]);

    // (x + bi)^2 = x² + 2xbi - b²
    let z_squared = ComplexNum4 {
        re : z.re * z.re - z.im * z.im,
        im : f32x4::splat(2.0) * z.re * z.im, 
    };

    ComplexNum4 {
        re : z_squared.re + c.re,
        im : z_squared.im + c.im,
    }
}

