impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut curr_sum = nums[0];
        let mut max_sum = nums[0];
        let n = nums.len();
        for i in 1..n {
            curr_sum = std::cmp::max(0,curr_sum)+nums[i];
            max_sum = std::cmp::max(max_sum,curr_sum);
        }
        max_sum
    }
}