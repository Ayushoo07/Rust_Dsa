impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // Initialize index to a large number to indicate no such index found yet
        let mut index = 1000000;
        let n = nums.len();

        // Find the first index from the end where the current number is less than the next number
        for i in (1..n).rev() {
            if nums[i-1] < nums[i] {
                index = i;
                break;
            }
        }

        // If no such index is found, the permutation is the largest, so we sort to get the smallest permutation
        if index == 1000000 {
            nums.sort();
            return;
        }

        // Store the found index in `pre`
        let mut pre = index;
        println!("{}", pre); // Debug print statement

        // Find the smallest number in the range [index, n) that is greater than nums[index - 1]
        for i in (index+1)..n {
            if nums[index - 1] < nums[i] && nums[i] < nums[pre] {
                pre = i;
            }
        }

        // Swap nums[index - 1] with nums[pre]
        let tmp = nums[index-1];
        nums[index-1] = nums[pre];
        nums[pre] = tmp;

        // Sort the part of the array from index to the end to get the next permutation
        nums[index..].sort();
    }
}