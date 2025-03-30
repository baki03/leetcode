impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let value = numbers[left] + numbers[right];
            if value > target {
                right -= 1;
            } else if value < target {
                left += 1;
            } else {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }
        Vec::new()
    }
}