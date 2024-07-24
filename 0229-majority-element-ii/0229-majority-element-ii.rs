use std::collections::HashMap;
impl Solution {
   pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32,usize> = HashMap::new();
        let length = nums.len();
        for i in 0..length {
            if map.contains_key(&nums[i]) {
                let cnt = map.get_mut(&nums[i]).unwrap();
                *cnt += 1;
            } else {
                if map.len() == 2 {
                    let mut rem: Vec<i32> = Vec::new();
                    for (key, value) in map.iter_mut() {
                        *value-=1;
                        if *value == 0 {
                            rem.push(*key);
                        }
                    }
                    for idx in 0..rem.len() {
                        map.remove(&rem[idx]);
                    }
                } else {
                    map.insert(nums[i], 1);
                }
            }
        }

        let mut ans: Vec<i32> = Vec::new();
        for (key, value) in map.iter() {
            let mut cnt = 0;
            for i in 0..length {
                if *key == nums[i] {
                    cnt+=1;
                }
            }
            if cnt > (length/3) {
                ans.push(*key);
            }
        }
        return ans;
    }
}