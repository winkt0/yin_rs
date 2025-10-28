# 🎵 YIN Pitch Detection in Rust


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rustc-1.93.0%2B-orange.svg)](https://www.rust-lang.org)

Implementation of the YIN algorithm for estimating the fundamental frequency of an audio signal in Rust. Optimized for speed, and with parabolic interpretation for refining the result.

## Background

The **YIN algorithm** (Cheveigné & Kawahara, 2002) estimates the fundamental frequency (*f₀*) of a signal by minimizing the Cumulative Mean Normalized Difference Function (CMNDF).

This implementation follows the standard YIN steps:

1. Difference function (DF)
2. Cumulative mean normalization (CMNDF)
3. Absolute thresholding
4. (Parabolic interpolation for refined lag estimate)

The resulting best lag gives the estimated period, and therefore the frequency:
  
$$
f = \frac{f_s}{\text{lag}}
$$


De Cheveigné, A., & Kawahara, H. (2002). YIN, a fundamental frequency estimator for speech and music. The Journal of the Acoustical Society of America, 111(4), 1917–1930. https://doi.org/10.1121/1.1458024

```latex
@article{de-cheveigne-2002,
	author = {De Cheveigné, Alain and Kawahara, Hideki},
	journal = {The Journal of the Acoustical Society of America},
	month = {4},
	number = {4},
	pages = {1917--1930},
	title = {{YIN, a fundamental frequency estimator for speech and music}},
	volume = {111},
	year = {2002},
	doi = {10.1121/1.1458024},
	url = {https://doi.org/10.1121/1.1458024}
}
```

## Example Usage

```rust
use yin_rs::Yin;

fn main() {
    let sample_rate = 44100.0;
    let frequency = 440.0;
    let duration = 2.0;

    let signal: Vec<f64> = (0..(duration as usize * sample_rate as usize))
        .map(|n| (2.0 * std::f64::consts::PI * frequency * n as f64 / sample_rate).sin())
        .collect();

    let yin = Yin::init(0.1, 50.0, 2000.0, sample_rate);

    let result = yin.yin(&signal);
    println!(
        "Detected frequency: {:.2} Hz",
        result.get_frequency() // 441
    );
    println!(
        "Using parabolic interpolation, detected frequency: {:.2} Hz",
        result.get_frequency_with_interpolation() // 440.02
    );
}
```
