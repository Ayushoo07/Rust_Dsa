impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut cnt = 0;
    for i in 1..nums.len() {
        
        if nums[i-1] > nums[i] {
            cnt+=1;
        }
    }
    if nums[0] < nums[nums.len()-1] {
        cnt+=1;
        }
    return cnt <= 1;
    }
}