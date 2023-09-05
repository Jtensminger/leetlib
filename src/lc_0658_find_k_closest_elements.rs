/* 
Given a sorted integer array arr, two integers k and x, return the k closest integers to x in the array. 
The result should also be sorted in ascending order. 
An integer a is closer to x than an integer b if:
        |a - x| < |b - x|, or (i.e absolute difference)
        |a - x| == |b - x| and a < b
*/
/* explanation: https://leetcode.com/problems/find-k-closest-elements/solutions/1310981/simple-solutions-w-explanation-all-possible-approaches-explained/ */
fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, arr.len() - k as usize); /* guarentees our window size is correct */
        while left < right {
                let mid = (left + right) / 2;
                /* Now, consider arr[mid] and arr[mid + k].
                Both of these elements surely can't be in our final answer or else the final array size exceeds k.
                So we have to pick one of these. */
                if x - arr[mid] > arr[mid + k as usize] - x { /* left > right, so include the right by shifting left */
                        left = mid + 1;
                } else { /* left <= right, so include the left by shifting right */
                        right = mid;
                }
        }
        arr[left..left + k as usize].to_vec()
}

#[cfg(test)]
mod tests {
        use super::*;
        #[test]
        fn ext1() {
                let k_integers = 4;
                let target = 5;
                assert_eq!(vec![3,4,5,6], find_closest_elements(vec![1,2,3,4,5,6,7,8,9,10], k_integers, target))
        }
}
