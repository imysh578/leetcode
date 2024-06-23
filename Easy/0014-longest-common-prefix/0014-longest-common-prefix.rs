impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let first_str = &strs[0];
        let prefix_max_len = first_str.len();

        for i in 0..prefix_max_len {
            let current_char = first_str.as_bytes()[i];
            for j in 1..strs.len() {
                if strs[j].len() <= i || strs[j].as_bytes()[i] != current_char {
                    return first_str[..i].to_string();
                }
            }
        }

        first_str[..prefix_max_len].to_string()
    }
}