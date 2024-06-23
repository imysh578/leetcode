impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&j) => return vec![j as i32, i as i32],
                None => map.insert(num, i)
            };
        }

        return vec![];
    }
}