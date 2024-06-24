impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let first_str = &strs[0];
        let mut prefix_max_len = first_str.len();

        for i in 0..prefix_max_len {
            let current_char = first_str.as_bytes()[i];
            for word in strs.iter().skip(1) {
                if word.len() <= i || word.as_bytes()[i] != current_char {
                    return first_str[..i].to_string();
                }
            }
        }

        first_str[..prefix_max_len].to_string()
    }
}
