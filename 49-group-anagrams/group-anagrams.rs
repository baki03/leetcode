use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut freqs = s.bytes()
                .map(|c| (c - b'a') as usize)
                .fold([0; 26], |mut freqs, bin| {
                    freqs[bin] += 1;
                    freqs
                });

            map.entry(freqs)
                .or_default()
                .push(s);
        }

        map.into_values().collect()
    }
}