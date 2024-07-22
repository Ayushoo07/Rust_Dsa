impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero = 0;
        let mut one = 0 ;
        let mut two = nums.len() - 1;
        while one <= two {
            if nums[one] == 1 {
                one+=1;
            }else if nums[one] == 0 {
                let tmp = nums[one];
                nums[one] = nums[zero];
                nums[zero] = tmp;
                zero+=1;
                one+=1;
            }else {
                let tmp = nums[one];
                nums[one] = nums[two];
                nums[two] = tmp;
                if two == 0 {
                    break;
                }
                two-=1;
            }

        };   
    }
}