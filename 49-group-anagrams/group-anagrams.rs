use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for value in strs {
            let mut freqs = value
                .bytes()
                .map(|b| (b - b'a') as usize).fold([0; 26], |mut freqs, bin| {
                    freqs[bin] += 1;
                    freqs
                }
            );

            result.entry(freqs).or_default().push(value)
        }

        result.into_values().collect()
    }
}