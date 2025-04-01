impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l + r);
                },
                "-" => if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l - r);
                },
                "*" => if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l * r);
                },
                "/" => if let (Some(r), Some(l)) = (stack.pop(), stack.pop()) {
                    stack.push(l / r);
                },
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }
        match stack.pop() {
            Some(value) => value,
            None => -1
        }
    }
}