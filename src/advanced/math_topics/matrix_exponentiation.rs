//! Matrix exponentiation.
//!
//! Matrix exponentiation reduces linear recurrences from `O(n)` transitions to `O(log n)` matrix
//! squaring steps. The classic example is Fibonacci numbers.
//!
//! # Example
//!
//! ```
//! use project1::advanced::math_topics::matrix_exponentiation::fibonacci_matrix;
//!
//! assert_eq!(fibonacci_matrix(10), 55);
//! ```

type Matrix = [[u64; 2]; 2];

fn multiply(a: Matrix, b: Matrix) -> Matrix {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        ],
    ]
}

pub fn fibonacci_matrix(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = [[1, 0], [0, 1]];
    let mut base = [[1, 1], [1, 0]];
    let mut power = n - 1;
    while power > 0 {
        if power & 1 == 1 {
            result = multiply(result, base);
        }
        base = multiply(base, base);
        power >>= 1;
    }
    result[0][0]
}

#[cfg(test)]
mod tests {
    use super::fibonacci_matrix;

    #[test]
    fn matrix_exponentiation_computes_fibonacci_numbers() {
        assert_eq!(fibonacci_matrix(0), 0);
        assert_eq!(fibonacci_matrix(1), 1);
        assert_eq!(fibonacci_matrix(10), 55);
    }
}
