impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans: Vec<i32> = vec![0;n];
        let mut pos = 0;
        let mut neg = 1;
        for i in 0..n {
            if nums[i] >=0 {
                ans[pos] = nums[i];
                pos+=2;
            } else {
                ans[neg] = nums[i];
                neg += 2;
            }
        }
        ans
    }
}