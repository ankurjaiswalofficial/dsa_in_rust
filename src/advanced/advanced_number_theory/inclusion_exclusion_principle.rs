//! Inclusion-exclusion principle.
//!
//! Inclusion-exclusion counts union sizes by alternating between adding intersections of odd size
//! and subtracting intersections of even size.
//!
//! # Example
//!
//! ```
//! use project1::advanced::advanced_number_theory::inclusion_exclusion_principle::count_divisible_by_any;
//!
//! assert_eq!(count_divisible_by_any(10, &[2, 5]), 6);
//! ```

pub fn count_divisible_by_any(limit: i64, divisors: &[i64]) -> i64 {
    let mut answer = 0;
    for mask in 1usize..(1usize << divisors.len()) {
        let mut lcm = 1_i64;
        let mut bits = 0;
        let mut valid = true;
        for (index, &divisor) in divisors.iter().enumerate() {
            if (mask >> index) & 1 == 1 {
                bits += 1;
                lcm = lcm / gcd(lcm, divisor) * divisor;
                if lcm > limit {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            if bits % 2 == 1 {
                answer += limit / lcm;
            } else {
                answer -= limit / lcm;
            }
        }
    }
    answer
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

#[cfg(test)]
mod tests {
    use super::count_divisible_by_any;

    #[test]
    fn inclusion_exclusion_counts_union_of_divisibility_sets() {
        assert_eq!(count_divisible_by_any(10, &[2, 5]), 6);
    }
}

