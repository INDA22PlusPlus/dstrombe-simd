# dstrombe-simd
mandelbrot simd

you need rust nightly to build this (portable simd is nightly). Rust nightly is only available with rustup. 

test_image.ppm contains a mandelbrot render, the benchmark doesn't render anything bc of io ops and how criterion only can benchmark library crates.

**run:**
rustup run nightly cargo run

**bench:** 
rustup run nightly cargo bench
