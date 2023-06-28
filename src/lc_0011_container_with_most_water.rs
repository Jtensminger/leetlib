/*     
        You are given an integer array height of length n.
        There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]). 
        Find two lines that together with the x-axis form a container, such that the container contains the most water.
        Return the maximum amount of water a container can store.
*/

/* Solution:Tallest bars, Farthest Away */
/* Maximize Area of container (e.g square)
        H[0] = 8, H[3] = 5
        Smallest = MIN(H[0], H[3])
        Distance = 3 - 0 = 3
        Area = Smallest * 2 * Distance
*/
/* Naive: Track max of previous areas calculated & return max
        curr, prev
*/


/* two ptr strategy */
use std::cmp::{min, max};
pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut left, mut right) = (0, 0, height.len() - 1);
        while left < right {
                let area = min(height[left], height[right]) * ((right - left) as i32);
                max_area = max(area, max_area);
                if height[left] < height[right] {
                        left +=1;
                } else {
                        right -=1;
                }
        }
        max_area
}

pub fn max_area_slices(height: Vec<i32>) -> i32 {
        max_area_recurse(&height[..], 0)
}

pub fn max_area_recurse(height: &[i32], mut max_area: i32) -> i32 {
        if height.len() == 1 || height.len() == 0  { return max_area }
        let (left, right) = (0, height.len() - 1);
        let area = min(height[left], height[right]) * ((right - left) as i32);
        max_area = max(area, max_area);
        if height[left] < height[right] {
                max_area_recurse(&height[left+1..], max_area)
        } else {
                max_area_recurse(&height[..=right-1], max_area)
        }
}

#[cfg(test)]
pub mod tests {
        use super::*;
        #[test]
        fn ext1() {
                assert_eq!(49, max_area(vec![1,8,6,2,5,4,8,3,7]));
                assert_eq!(4, max_area(vec![2,1,2]));
                assert_eq!(1, max_area(vec![1,1]));
        }
        #[test]
        fn ext2() {
                assert_eq!(49, max_area_slices(vec![1,8,6,2,5,4,8,3,7]));
                assert_eq!(4, max_area_slices(vec![2,1,2]));
                assert_eq!(1, max_area_slices(vec![1,1]));
        }
}
