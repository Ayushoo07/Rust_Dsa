impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut arr = vec![false;nums.len()+1];
        for i in 0..nums.len() {
            arr[nums[i] as usize] = true;
        }
        for i in 0..arr.len() {
            if !arr[i] {
                return i as i32;
            }
        }
        return -1;
    }
}