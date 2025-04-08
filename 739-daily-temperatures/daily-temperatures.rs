impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut res = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            let current_temp = temperatures[i];
            while let Some(&(index, temp)) = stack.last() {
                if current_temp > temp {
                    res[index] = (i - index) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((i, current_temp));
        }

        res
    }
}