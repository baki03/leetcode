impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result = Vec::new();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let sum = nums[j] + nums[k] + nums[i];
                if sum > 0 {
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
            }
        }
        result
    }
}