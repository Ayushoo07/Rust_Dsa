impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    let len = nums.len();
    let mut ans: Vec<Vec<i32>> = Vec::new();
    nums.sort();
    for i in 0..(len-2) {
        // Skip duplicate elements
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut left = i+1;
        let mut right = len - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                ans.push(vec![nums[i], nums[left], nums[right]]);
                // Skip duplicate elements
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if nums[left] + nums[right] + nums[i] > 0 {
                right -=1
            } else {
                left +=1;
            }
        }
    }
    return ans;
    }
}