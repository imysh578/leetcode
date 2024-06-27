impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if stack.pop() != Some(c) => return false,
                ')' | ']' | '}' => {},
                _ => return false,
            }
        }

        stack.is_empty()
    }
}