impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s.to_lowercase();
        s.retain(|c| 
            (c >= 'a' && c <= 'z') ||
            (c >= 'A' && c <= 'Z') ||
            (c >= '0' && c <= '9')
        );

        let bytes = s.as_bytes();
        let len = s.len();
        let mid_len = len / 2;
        for i in 0..mid_len {
            if bytes[i] != bytes[len - i - 1] {
                return false;
            }
        }
        true
    }
}