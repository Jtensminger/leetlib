pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
    
        while left <= right {
            let mid = left + (right - left) / 2;
            let total_hours: f64 = piles.iter()
                .map(|&pile| (pile as f64 / mid as f64).ceil())
                .sum();
    
            if total_hours as i32 > h {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    
        left
}