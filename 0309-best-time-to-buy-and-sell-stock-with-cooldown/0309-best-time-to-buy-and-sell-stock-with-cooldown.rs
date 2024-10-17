impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let n = prices.len();
        let mut hold = vec![0; n];       // Max profit when holding stock
        let mut cooldown = vec![0; n];   // Max profit when in cooldown
        let mut idle = vec![0; n];       // Max profit when idle

        // Base cases
        hold[0] = -prices[0];  // We bought the stock on day 0
        cooldown[0] = 0;       // No cooldown on day 0
        idle[0] = 0;           // No profit from being idle on day 0

        for i in 1..n {
            hold[i] = hold[i - 1].max(idle[i - 1] - prices[i]);      // Either we held from yesterday or bought today
            cooldown[i] = hold[i - 1] + prices[i];                   // We sold the stock today and now in cooldown
            idle[i] = idle[i - 1].max(cooldown[i - 1]);              // Either we stayed idle or came from cooldown
        }

        // The maximum profit on the last day is either in idle or cooldown state
        idle[n - 1].max(cooldown[n - 1])
    }
}