impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left = height[left];
        let mut max_right = height[right];
        let mut count = 0;

        while left < right {
            if max_left < max_right {
                left += 1;
                max_left = max_left.max(height[left]);
                count += max_left - height[left];
            } else {
                right -= 1;
                max_right = max_right.max(height[right]);
                count += max_right - height[right];
            }
        }
        count
    }
}