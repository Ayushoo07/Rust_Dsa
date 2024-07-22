impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_sum = nums[0];
        let mut max_sum = nums[0];
        let n = nums.len();
        for i in 1..n {
            if curr_sum > 0 {
                curr_sum = curr_sum + nums[i];
            } else {
                curr_sum = nums[i];
            }
            if max_sum < curr_sum {
                max_sum = curr_sum;
            }
        }
        max_sum
    }
}