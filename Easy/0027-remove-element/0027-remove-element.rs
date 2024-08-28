impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;

        for index in 0..nums.len() {
            let num = nums[index];
            if num != val {
                nums[count] = num;
                count += 1;
            }
        }

        for _index in count..nums.len() {
            nums.remove(count);
        }

        nums.len() as i32
    }
}