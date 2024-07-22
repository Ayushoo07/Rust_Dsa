impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        for i in 0..nums.len() {
            ans = ans ^ nums[i];
        }
        return ans;
    }
}