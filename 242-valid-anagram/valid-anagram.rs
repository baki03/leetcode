impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut map = vec![0; 26];
        let s: Vec<char> = s.chars().map(|c| c).collect();
        let t: Vec<char> = t.chars().map(|c| c).collect();
        for i in 0..s.len() {
            let key_s: usize = s[i].to_ascii_lowercase() as usize - 97;
            let key_t: usize = t[i].to_ascii_lowercase() as usize - 97;
            map[key_s] += 1;
            map[key_t] -= 1;
        }

        map.into_iter().all(|v| if v != 0 { false } else { true })
    }
}