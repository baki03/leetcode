impl Solution {
    fn backtrack(pair: (i32, i32), n: i32, res: &mut Vec<String>, stack: String) {
        let (open_n, closed_n) = pair;

        if open_n == n && open_n == closed_n {
            res.push(stack);
            return;
        }

        if open_n < n {
            Solution::backtrack(
                (open_n + 1, closed_n),
                n,
                res,
                format!("{}(", stack)
            );
        }

        if closed_n < open_n {
            Solution::backtrack(
                (open_n, closed_n + 1),
                n,
                res,
                format!("{})", stack)
            );
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        Solution::backtrack((0, 0), n, &mut res, "".to_string());
        res       
    }
}