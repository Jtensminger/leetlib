// find subarray of arr[_]...arr[_] == target
// both are unsorted
// sort both & check equality Time(2(nlogn)) OR
// linear scan & hashmap Time(2n) Space(2n)
use std::collections::HashMap;
fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut m = HashMap::with_capacity(target.len());

        for &item in &arr { 
                *m.entry(item).or_insert(0) += 1; 
        }

        for &item in &target {
                if let Some(counter) = m.get_mut(&item) {
                        *counter -= 1;
                        if *counter < 0 {
                                return false;
                        } else if *counter == 0 {
                                m.remove(&item);
                        } else {
                                return false;
                        }
                }
        }
        m.len() == 0
}
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        assert_eq!(true, can_be_equal(vec![1,2,3,4],vec![2,4,1,3]));
    }
}
