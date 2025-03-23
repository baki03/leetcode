use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut square: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }

                if rows.entry(r).or_default().contains(&board[r][c]) ||
                    cols.entry(c).or_default().contains(&board[r][c]) ||
                    square.entry((r / 3, c / 3)).or_default().contains(&board[r][c]) {
                        return false;
                }

                rows.entry(r).or_default().insert(board[r][c]);
                cols.entry(c).or_default().insert(board[r][c]);
                square.entry((r / 3, c / 3)).or_default().insert(board[r][c]);
            }
        }

        true
    }
}