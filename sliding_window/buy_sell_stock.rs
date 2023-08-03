// Solution for: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp;

#[test]
fn test_max_profit() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);

    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(max_profit(prices), 0);

    let prices = vec![2, 1, 2, 1, 0, 1, 2];
    assert_eq!(max_profit(prices), 2);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_seen = prices[0];
    let mut max_profit = 0;

    for price in prices {
        min_seen = cmp::min(min_seen, price);
        max_profit = cmp::max(max_profit, price - min_seen);
    }

    max_profit
}
