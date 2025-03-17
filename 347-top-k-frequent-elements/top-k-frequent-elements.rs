use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut freqs: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        let mut result = vec![0; k as usize];

        for num in nums {
            map
                .entry(num)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        for (k, v) in map.into_iter() {
            freqs[v as usize].push(k);
        }

        let mut i = 0;
        let mut j = freqs.len() - 1;
        while (i < k && j > 0) {
            for n in &freqs[j] {
                result[i as usize] = *n;
                i += 1;
                if i == k {
                    return result;
                }
            }

            j -= 1;
        }

        result
    }
}