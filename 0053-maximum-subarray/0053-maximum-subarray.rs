impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_sum = nums[0];
        let mut max_sum = nums[0];
        for i in 1..nums.len() {
            curr_sum = std::cmp::max(curr_sum + nums[i], nums[i]);
            max_sum = std::cmp::max(curr_sum, max_sum);
        }
        max_sum
    }
}