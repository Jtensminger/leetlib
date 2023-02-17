/*
        You are given an array prices where prices[i] is the price of a given stock on the ith day.

        You want to maximize your profit by choosing a single day to buy one stock 
        and choosing a different day in the future to sell that stock.

        Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
 */

pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut highest_profit_potential = 0;
        let num_of_days = prices.len();

        for i in 0..num_of_days - 1 {
                for n in i+1..num_of_days {
                        let profit = prices[n] - prices[i];

                        if profit >= highest_profit_potential {
                                highest_profit_potential = profit;        
                        }
                }
        }
        highest_profit_potential
}

#[cfg(test)]
mod test {
        use super::*;

        #[test]
        fn ex1() {
                // input prices
                let prices = vec![7,1,5,3,6,4];
                
                let max_profit = max_profit(prices);
                assert_eq!(5, max_profit);
        }

        #[test]
        fn ex2() {
                // input prices
                let prices = vec![7,6,4,3,1];


                // check solution
                let max_profit = max_profit(prices);
                assert_eq!(0, max_profit);
        }
}