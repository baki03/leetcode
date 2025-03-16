use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for value in strs {
            let mut count = vec![0; 26];
            for c in value.chars() {
                let key: usize = c.to_ascii_lowercase() as usize - 97;
                count[key] += 1;
            }
            let key: String = count.iter().map(|v| format!("{}#", v.to_string())).collect();

            if let Some(array) = result.get_mut(&key) {
                array.push(value);
            } else {
                result.insert(key, vec![value]);
            }
        }

        result.into_values().collect()
    }
}