impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        // 첫 번째 문자열을 기준으로 초기 접두사를 설정합니다.
        let first_str = &strs[0];
        let mut prefix_max_len = first_str.len();

        // 첫 번째 문자열의 문자들을 슬라이스로 접근하여 처리합니다.
        for word in strs.iter() {
            prefix_max_len = std::cmp::min(prefix_max_len, word.len());
        }

        for i in 0..prefix_max_len {
            let current_char = first_str.as_bytes()[i];
            for j in 1..strs.len() {
                if strs[j].as_bytes()[i] != current_char {
                    return first_str[..i].to_string();
                }
            }
        }

        first_str[..prefix_max_len].to_string()
    }
}
