//! Fast Fourier Transform.
//!
//! FFT evaluates a polynomial at roots of unity and reconstructs it efficiently, enabling near
//! linear-time polynomial multiplication.
//!
//! This implementation uses an iterative Cooley-Tukey FFT over complex numbers represented by a
//! small custom struct so the code stays self-contained.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algebra_polynomial::fast_fourier_transform_fft::fft_multiply;
//!
//! assert_eq!(fft_multiply(&[1, 2], &[3, 4]), vec![3, 10, 8]);
//! ```

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, Default)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

fn fft(values: &mut [Complex], invert: bool) {
    let n = values.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            values.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let angle = 2.0 * PI / len as f64 * if invert { -1.0 } else { 1.0 };
        let wlen = Complex::new(angle.cos(), angle.sin());
        for start in (0..n).step_by(len) {
            let mut w = Complex::new(1.0, 0.0);
            for i in 0..len / 2 {
                let u = values[start + i];
                let v = values[start + i + len / 2] * w;
                values[start + i] = u + v;
                values[start + i + len / 2] = u - v;
                w = w * wlen;
            }
        }
        len <<= 1;
    }

    if invert {
        for value in values {
            value.re /= n as f64;
            value.im /= n as f64;
        }
    }
}

pub fn fft_multiply(left: &[i64], right: &[i64]) -> Vec<i64> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut n = 1usize;
    while n < left.len() + right.len() - 1 {
        n <<= 1;
    }
    let mut fa = vec![Complex::default(); n];
    let mut fb = vec![Complex::default(); n];
    for (i, &value) in left.iter().enumerate() {
        fa[i] = Complex::new(value as f64, 0.0);
    }
    for (i, &value) in right.iter().enumerate() {
        fb[i] = Complex::new(value as f64, 0.0);
    }
    fft(&mut fa, false);
    fft(&mut fb, false);
    for i in 0..n {
        fa[i] = fa[i] * fb[i];
    }
    fft(&mut fa, true);
    fa.into_iter()
        .take(left.len() + right.len() - 1)
        .map(|value| value.re.round() as i64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::fft_multiply;

    #[test]
    fn fft_interface_multiplies_polynomials() {
        assert_eq!(fft_multiply(&[1, 2], &[3, 4]), vec![3, 10, 8]);
        assert_eq!(fft_multiply(&[2, 0, 1], &[1, 3]), vec![2, 6, 1, 3]);
    }
}
