//! Windowing functions, useful in conjuction with [`StftHelper`][super::StftHelper].

use std::f32;

/// A Hann window function.
///
/// <https://en.wikipedia.org/wiki/Hann_function>
pub fn hann(size: usize) -> Vec<f32> {
    // We want to scale `[0, size - 1]` to `[0, pi]`.
    let scale = (size as f32 - 1.0).recip() * f32::consts::PI;
    (0..size)
        .map(|i| {
            let sin = (i as f32 * scale).sin();
            sin * sin
        })
        .collect()
}

/// Multiply a buffer with a window function.
#[inline]
pub fn multiply_with_window(buffer: &mut [f32], window_function: &[f32]) {
    // TODO: ALso use SIMD here if available
    for (sample, window_sample) in buffer.iter_mut().zip(window_function) {
        *sample *= window_sample;
    }
}