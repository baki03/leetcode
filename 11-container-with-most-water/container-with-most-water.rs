impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut area = 0;

        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            area = area.max(w * h);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}