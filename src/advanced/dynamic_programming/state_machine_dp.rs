//! State machine DP.
//!
//! Some DP problems become simpler when we describe them as a finite set of states and allowed
//! transitions. Stock trading is a common example because each day we are either holding a stock,
//! resting, or in cooldown after selling.
//!
//! # Example
//!
//! ```
//! use project1::advanced::dynamic_programming::state_machine_dp::max_profit_with_cooldown;
//!
//! assert_eq!(max_profit_with_cooldown(&[1, 2, 3, 0, 2]), 3);
//! ```

pub fn max_profit_with_cooldown(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let mut hold = -prices[0];
    let mut sold = 0;
    let mut rest = 0;
    for &price in &prices[1..] {
        let previous_sold = sold;
        sold = hold + price;
        hold = hold.max(rest - price);
        rest = rest.max(previous_sold);
    }
    sold.max(rest)
}

#[cfg(test)]
mod tests {
    use super::max_profit_with_cooldown;

    #[test]
    fn state_machine_dp_solves_stock_with_cooldown() {
        assert_eq!(max_profit_with_cooldown(&[1, 2, 3, 0, 2]), 3);
    }
}
