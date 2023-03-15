/* #TAGS[Dynamic Programming] #DIFFICULTY[Easy] #URL[https://leetcode.com/problems/climbing-stairs/] */

/*
        Task:
                climb n # of steps
                either take 1 or 2 steps at a time
        Constraints: 1 <= n <= 45
        Question: How many distinct ways can you climb to the top?
        
        
My questions:
Can this be rephrased as: how many permutations exist for n, given the constraints?
        I say permutations because the order of the steps you take matters.
                e.x n=3 [1 + 1 + 1] [1 + 2] [2 + 1]
                ..there are 3 unique lists here, where they contain only 1 or 2, and the order & length varies
        
        it's Permutation w/ repetition. Forumula is #_of_valid_values^#_of_placeholders
                #_of_valid_values = 2 since we can either have 2 or 1
                #_of_placeholders = # of stairs to climb

        What am i taking the permutation of? a list of values to sum?
        is the answer always 4? 
                e.x n=4
                [1 + 1 + 1 + 1]
                [1 + 1 + 2] [2 + 1 + 1] [1 + 2 + 1]
                [2 + 2]
        what happens if i start off w/ longest path, reduce it to it's smallest form, & sum up the permutations of each form?
                n=3
                [1, 1, 1]    
                [2, 1] [1, 2] 
                
                n=4
                [1, 1, 1, 1], spaces=4, one=4, two=0, equation: spaces! / (one!two!) = 1
                [2, 1, 1] [1,2,1] [1, 1, 2], spaces=3, one=2, two=1, equation: spaces! / (one!two!) = 3
                [2, 2], spaces=2, one=0, two=2, equation: spaces! / (one!two!) = 1
                
                This method works!
                        1. create longest combination
                        2. Calc & save permutation,
                        3. Add to permutation of last combination,
                        4. repeat until reduced
                        5. return sum of permutations


What is Dynamic Programming?
        Wherever we see a recursive solution that has repeated calls for the same inputs
        rewriting the recurrsion to simply store the results of subproblems
        so that we do not have to re-compute them when needed later
Why is Dynamic Programming relevant?
Seems similar to the .fold() iterator method. lc_0121 since that used the fold() method. Passing values forward to the next iteration.


 */


/*
This method works!
        1. create longest combination
        2. Calc & save permutation => spaces=4, one=4, two=0, => spaces! / (one!two!)
        3. Add to permutation of last combination,
        4. repeat until reduced
        5. return sum of permutations

        How do I best represent these lists?
        (spaces, one, two, digital_sum_permutations)
                (3, 3, 0, 1)
                (4, 4, 0, 1)    (5, 5, 0, 1) [1,1,1,1,1]
                (3, 2, 1, 4)    (4, 3, 1, 4) [1,1,1,2]
                (2, 0, 2, 5)    (3, 1, 2, X) [1,2,2]
                
                (6, 6, 0, X)
                (5, 4, 1, X)
                (3, 2, 2, X)
                (2, 0, 3, X)

                (7, 7, 0, 1)    (8, 8, 0, 1)
                (6, 5, 1, X)    (7, 6, 1, X)
                (5, 3, 2, X)    (6, 4, 2, X)
                (4, 1, 3, X)    (5, 2, 3, X)
                                (4, 0, 4, X)

                


        what's the transformation rule?
                
        
                
 */

// The version I submitted to LeetCode since it doesn't have the num crate available
// pub fn climb_stairs(n: i32) -> i32 {
//         (0..n)
//             .fold((1, 0), |(res, prev), _| (res + prev, res))
//             .0
// }


extern crate num;
use num ::{BigUint, One};

pub fn climb_stairs(n: i32) -> i32 {
        match n {
                0 | 1 | 2 => n,
                _ => { 
                        let mut spaces: u64 = n as u64;
                        let mut ones: u64 = spaces;
                        let mut twos: u64 = 0;
                        let mut acc: u64 = 1;
                        let max_twos: u64 = spaces / 2;

                        
                        while twos != max_twos {  
                                spaces -= 1;
                                ones -= 2;
                                twos += 1;
                                acc += permutations(spaces, ones, twos);
                        }
                        acc as i32
                }
        }
}

pub fn permutations(spaces: u64, ones: u64, twos: u64) -> u64 {
        (factorial(spaces) / (factorial(ones) * factorial(twos)))
        .to_u64_digits()
        .pop()
        .unwrap()
}

pub fn factorial(num: u64) -> BigUint {
        match num {
                0 | 1 => BigUint::one(),
                _ => {
                     let mut f = BigUint::one();
                     let mut i = 2;
                     while i <= num {
                        f *= i;
                        i += 1;
                     }
                     f
                }
        }
}


#[cfg(test)]
mod test {
        use super::*;


        #[test]
        fn ext1() {
                assert_eq!(2, climb_stairs(2));
        }

        #[test]
        fn ext2() {
                assert_eq!(3, climb_stairs(3));
        }

        #[test]
        fn ext3() {
                assert_eq!(5, climb_stairs(4));
        }

        #[test]
        fn ext4() {
                assert_eq!(63245986, climb_stairs(38));
        }
}