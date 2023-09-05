
fn find_min(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        /* base case */
        if len == 1 {
            return nums[0];
        }
        let (left, right) = (0, len - 1);
        /* it has not been rotated or all elements are the same */
        if nums[left] < nums[right] {
            return nums[left];
        }
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[right] {
                /* The array's minimum value (the pivot point of the rotation) must be in the right half of the array. */
                /* The reason we don't include mid (i.e., why we use mid + 1 instead of mid) is because we've already checked nums[mid], and we know that it's not less than nums[right] */
                find_min(nums[mid + 1..].to_vec())
        } else if nums[mid] < nums[right] {
                /* the minimum value must be in the left half of the array, because this condition signifies that the array is sorted from mid to right */
                /* When we decide to search the left half of the array, we need to include mid, setting the right boundary as mid and not mid - 1.
                 This is because nums[mid] is potentially the minimum value in the array (we know it's less than nums[right], but we don't know if there's anything smaller to its left). */
                find_min(nums[..=mid].to_vec())
        } else {
                /* middle elememnt == last element case (i.e duplicates) 
                In this case, we can't be sure which half of the array to search.
                So we shrink the search space by one (from the right side) and perform the search again.*/
                find_min(nums[..right].to_vec())
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn ext1() {
                assert_eq!(1, find_min(vec![1,3,5]));
                assert_eq!(0, find_min(vec![2,2,2,0,1]));
                assert_eq!(1, find_min(vec![3,1,3]));
                assert_eq!(1, find_min(vec![1,3,3,3]));
        }
}
