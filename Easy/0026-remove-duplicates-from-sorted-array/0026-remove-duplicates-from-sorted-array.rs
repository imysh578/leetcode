impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pointer = 0;

        for i in 0..nums.len() {
            if nums[i] > nums[pointer] {
                pointer += 1;
                nums[pointer] = nums[i];
            }
        }

        (pointer + 1) as i32
    }
}