impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 0 ;
        let mut ele = 0;
        for i in 0..nums.len() {
            if cnt == 0 {
                ele = nums[i];
                cnt+=1;
            } else if nums[i] == ele {
                cnt+=1;
            } else {
                cnt-=1;
            }
        }
        cnt = 0;
        for i in 0..nums.len() {
            if nums[i] == ele {
                cnt+=1;
            }
        }
        if cnt > (nums.len()/2) {
            return  ele;
        }
        return -1;
    }
}