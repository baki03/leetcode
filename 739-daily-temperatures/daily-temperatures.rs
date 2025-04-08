impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(usize, i32)> = Vec::new();
        let mut res = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            let current_temp = temperatures[i];
            while !stack.is_empty() && stack.last().unwrap().1 < current_temp {
                match stack.pop() {
                    Some((j, _)) => res[j] = (i - j) as i32,
                    _ => break,
                }
            }
            stack.push((i, current_temp));
        }
        res
    }
}