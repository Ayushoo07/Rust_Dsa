use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // Initialize the answer variable to store the count of subarrays with sum equal to k
        let mut ans: i32 = 0;

        // Initialize a variable to store the cumulative sum of elements
        let mut sum: i32 = 0;

        // Initialize a HashMap to store the frequency of cumulative sums
        let mut map: HashMap<i32, i32> = HashMap::new();

        // Insert an initial value into the map to handle the case where a subarray starts from index 0
        map.insert(0, 1);

        // Iterate over the array
        for i in 0..nums.len() {
            // Update the cumulative sum
            sum += nums[i];

            // Check if there is a previous cumulative sum that would make the subarray sum equal to k
            if map.contains_key(&(sum - k)) {
                // If such a cumulative sum exists, add its frequency to the answer
                ans += map.get(&(sum - k)).unwrap();
            }

            // Update the map with the current cumulative sum
            // If the cumulative sum is already in the map, increment its frequency
            // Otherwise, insert it with a frequency of 1
            map.insert(sum, map.get(&sum).unwrap_or(&0) + 1);
        }

        // Return the count of subarrays with sum equal to k
        return ans;
    }
}