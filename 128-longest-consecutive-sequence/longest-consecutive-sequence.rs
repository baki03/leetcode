use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut longest = 0;

        for num in nums {
            if !map.contains_key(&num) {
                let before = *map.get(&(num - 1)).unwrap_or(&0);
                let after = *map.get(&(num + 1)).unwrap_or(&0);
                let value = before + after + 1;

                map.insert(num, value);
                map.insert(num - before, value);
                map.insert(num + after, value);

                longest = longest.max(value);
            }
        }

        longest
    }
}