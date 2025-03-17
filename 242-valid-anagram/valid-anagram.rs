impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = vec![0; 26];
        let s: Vec<char> = s.chars().map(|c| c).collect();
        let t: Vec<char> = t.chars().map(|c| c).collect();
        for i in 0..s.len() {
            count[(s[i] as u8 - b'a') as usize] += 1;
            count[(t[i] as u8 - b'a') as usize] -= 1;
        }

        count.into_iter().all(|v| if v != 0 { false } else { true })
    }
}