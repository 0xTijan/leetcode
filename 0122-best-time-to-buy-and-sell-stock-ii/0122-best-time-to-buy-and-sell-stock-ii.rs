impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;
        let mut entry_price = -1;    // 0 -> no active trade

        for i in 0..prices.len() {
            if i+1 < prices.len() {
                if prices[i+1] > prices[i] && entry_price < 0 {
                    // if next prices is higher buy
                    entry_price = prices[i];
                } else if entry_price >= 0 && prices[i+1] < prices[i] {
                    // else if in trade sell if next price is lower
                    total_profit += prices[i] - entry_price;
                    entry_price = -1;
                }
            } else {
                if entry_price >= 0 {
                    // there is still an active trade - close it
                    total_profit += prices[i] - entry_price;
                    entry_price = -1;
                }
            }
        }

        return total_profit;
    }
}