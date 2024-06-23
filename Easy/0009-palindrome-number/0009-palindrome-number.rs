impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut temp_x = x;
        let mut reserved_x = 0;

        while temp_x > 0 {
            reserved_x = reserved_x * 10 + temp_x % 10;
            temp_x /= 10;
        }

        if reserved_x != x {
            return false;
        }

        return true;
    }
}