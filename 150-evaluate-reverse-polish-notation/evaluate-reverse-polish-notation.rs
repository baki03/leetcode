use std::ops::{Add, Sub, Mul, Div};

impl Solution {
    fn get_operator(token: &str) -> Option<fn(i32, i32) -> i32> {
        match token {
            "+" => Some(i32::add),
            "-" => Some(i32::sub),
            "*" => Some(i32::mul),
            "/" => Some(i32::div),
            _ => None
        }
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            if let Some(op) = Solution::get_operator(token.as_str()) {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(op(left, right));
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack.pop().unwrap()
    }
}