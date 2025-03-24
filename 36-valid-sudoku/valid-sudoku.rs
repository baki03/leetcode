impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: [i32; 9] = [0; 9];
        let mut cols: [i32; 9] = [0; 9];
        let mut square: [i32; 9] = [0; 9];

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    continue;
                }

                let value = board[r][c] as u8 - b'1';
                if rows[r] & (1 << value) > 0 ||
                    cols[c] & (1 << value) > 0 ||
                    square[(r / 3) * 3 + (c / 3)] & (1 << value) > 0 {
                    return false;
                }

                rows[r] |= 1 << value;
                cols[c] |= 1 << value;
                square[(r / 3) * 3 + (c / 3)] |= 1 << value;
            }
        }

        true
    }
}