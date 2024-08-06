impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // O(n^2)
        // for (i, &num1) in nums.iter().enumerate() {
        //     let complement = target - num1;

        //     for (j, &num2) in nums.iter().enumerate() {
        //         if i != j && num2 == complement {
        //             return vec![i as i32, j as i32];
        //         }
        //     }
        // }

        // return vec![];
        

        // O(n)
        use std::collections::HashMap;
        
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let compliment = target - num;

            if let Some(&j) = map.get(&compliment) {
                return vec![j as i32, i as i32];
            }

            map.insert(num, i);
        }

        return vec![];
    }
}