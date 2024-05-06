impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        let iteration = nums.len();
        let mut count = 0;
        
        while count < iteration {
            let mut sum = 0;
            for (position, num) in nums.iter().enumerate() {
                sum += *num;
                if count == position {
                    break;
                }
            }
            count+=1;
            result.push(sum);
        }
        result
    }
}