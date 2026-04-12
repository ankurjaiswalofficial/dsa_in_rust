//! Number Theoretic Transform.
//!
//! NTT is the modular-arithmetic analogue of FFT. It avoids floating-point error by working under
//! a prime modulus with suitable roots of unity.
//!
//! This implementation targets the common competitive-programming modulus `998244353` and primitive
//! root `3`.
//!
//! # Example
//!
//! ```
//! use project1::advanced::algebra_polynomial::number_theoretic_transform_ntt::ntt_multiply;
//!
//! assert_eq!(ntt_multiply(&[1, 2], &[3, 4], 998_244_353), vec![3, 10, 8]);
//! ```

const DEFAULT_ROOT: i64 = 3;

fn mod_pow(mut base: i64, mut exp: i64, modu: i64) -> i64 {
    let mut result = 1_i64;
    base = base.rem_euclid(modu);
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base).rem_euclid(modu);
        }
        base = (base * base).rem_euclid(modu);
        exp >>= 1;
    }
    result
}

fn ntt(values: &mut [i64], invert: bool, modulus: i64) {
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

    let mut len = 2usize;
    while len <= n {
        let wlen = mod_pow(DEFAULT_ROOT, (modulus - 1) / len as i64, modulus);
        let wlen = if invert {
            mod_pow(wlen, modulus - 2, modulus)
        } else {
            wlen
        };
        for start in (0..n).step_by(len) {
            let mut w = 1_i64;
            for i in 0..len / 2 {
                let u = values[start + i];
                let v = values[start + i + len / 2] * w % modulus;
                values[start + i] = (u + v) % modulus;
                values[start + i + len / 2] = (u - v).rem_euclid(modulus);
                w = w * wlen % modulus;
            }
        }
        len <<= 1;
    }

    if invert {
        let inv_n = mod_pow(n as i64, modulus - 2, modulus);
        for value in values {
            *value = *value * inv_n % modulus;
        }
    }
}

pub fn ntt_multiply(left: &[i64], right: &[i64], modulus: i64) -> Vec<i64> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut n = 1usize;
    while n < left.len() + right.len() - 1 {
        n <<= 1;
    }
    let mut fa = vec![0_i64; n];
    let mut fb = vec![0_i64; n];
    for (i, &value) in left.iter().enumerate() {
        fa[i] = value.rem_euclid(modulus);
    }
    for (i, &value) in right.iter().enumerate() {
        fb[i] = value.rem_euclid(modulus);
    }
    ntt(&mut fa, false, modulus);
    ntt(&mut fb, false, modulus);
    for i in 0..n {
        fa[i] = fa[i] * fb[i] % modulus;
    }
    ntt(&mut fa, true, modulus);
    fa.truncate(left.len() + right.len() - 1);
    fa
}

#[cfg(test)]
mod tests {
    use super::ntt_multiply;

    #[test]
    fn ntt_interface_multiplies_polynomials_modulo_prime() {
        assert_eq!(ntt_multiply(&[1, 2], &[3, 4], 998_244_353), vec![3, 10, 8]);
        assert_eq!(ntt_multiply(&[2, 0, 1], &[1, 3], 998_244_353), vec![2, 6, 1, 3]);
    }
}
