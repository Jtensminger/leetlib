pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut position_speed_sorted = position
                                        .into_iter()
                                        .zip(speed)
                                        .into_iter()
                                        .collect::<Vec<(i32, i32)>>();
        position_speed_sorted.sort_by_key(|k| -k.0);
    
        let mut fleets = 1;
        let mut leader_eta = (target - position_speed_sorted[0].0) as f32 / position_speed_sorted[0].1 as f32;
        for i in 1..position_speed_sorted.len() {
            let eta = (target - position_speed_sorted[i].0) as f32 / position_speed_sorted[i].1 as f32;
            if eta > leader_eta {
                fleets += 1;
                leader_eta = eta;
            }
        }
    
        fleets
}

pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        // Add your test code here
    }
}
