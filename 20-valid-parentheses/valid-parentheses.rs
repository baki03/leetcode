impl Solution {
    fn get_pair(c: char) -> Option<char> {
        match c {
            '(' => Some(')'),
            '{' => Some('}'),
            '[' => Some(']'),
            _  => None
        }
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() { 
            if let Some(bracket) = Solution::get_pair(c) {
                stack.push(bracket);
            } else if Some(c) != stack.pop() {
                return false;
            }
        }
        stack.len() == 0
    }
}