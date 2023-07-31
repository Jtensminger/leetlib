/* 
Given a non-negative integer x, return the square root of x rounded down to the nearest integer. 
The returned integer should be non-negative as well.
You must not use any built-in exponent function or operator.
 */
use std::cmp::Ordering;
pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 { // non-negative => 0 is allowed
                return 0
        }
        let mut lower = 2;
        let mut upper = 46340.min(x / 2); // max possible root sqrt(i32::MAX)
        // boundary conditions for optimization
        if x <= 3 {
                return 1;
        }
        if x >= upper * upper {
                return upper;
        }
        // Binary search the correct perfect square
        while upper - lower > 1 {
                let mid = (upper + lower) / 2;
                let pow = mid * mid;
                match pow.cmp(&x) {
                        Ordering::Less => lower = mid,
                        Ordering::Greater => upper = mid,
                        Ordering::Equal => return mid
                }
        }
        // In the edge case that the number is between 2 values,
        // we take the lower which is equivalent to taking the floored mean
        lower
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(2, my_sqrt(4));
        assert_eq!(2, my_sqrt(8));
    }
}
