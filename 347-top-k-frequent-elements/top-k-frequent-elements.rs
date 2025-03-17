use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.into_iter()
            .for_each(|num| *map.entry(num).or_insert(0) += 1);
        let mut result: Vec<(i32, i32)> = map.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result.iter().take(k as usize).map(|x| x.0).collect()
    }
}