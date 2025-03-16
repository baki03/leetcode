use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        !nums.into_iter().all(|n| map.insert(n))
    }
}