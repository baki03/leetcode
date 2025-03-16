use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if !map.contains_key(num) {
                map.insert(num, i);
            }
        }

        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - num)) {
                if (*j != i) {
                    return vec![i as i32, *j as i32];
                }
            }
        }

        Vec::new()
    }
}