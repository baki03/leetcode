use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - num)) {
                return vec![i as i32, *j as i32];
            } else {
                map.insert(num, i);
            }
        }

        Vec::new()
    }
}