pub mod praxis_1 {
    use std::collections::HashMap;

    pub fn median(v: &[i32]) -> i32 {
        let mut vv = v.to_vec();
        vv.sort();
        vv[vv.len() / 2]
    }

    pub fn mode(v: &[i32]) -> i32 {
        let mut counts = HashMap::new();
        for &i in v {
            counts.entry(i).and_modify(|c| *c += 1).or_insert(1);
        }
        *counts.iter().max_by_key(|&(_, v)| v).unwrap().0
    }
}

pub mod praxis_2 {
    pub fn pig_latin(s: &str) -> String {
        let mut result = String::new();
        for word in s.split_whitespace() {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                match first {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                        result.push_str(&format!("{}hay ", word));
                    }
                    _ => {
                        result.push_str(&format!("{}{}ay ", chars.as_str(), first));
                    }
                }
            }
        }
        result.trim_end().to_string()
    }
}
