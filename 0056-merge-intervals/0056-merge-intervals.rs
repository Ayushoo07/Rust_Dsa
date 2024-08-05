impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals.clone();
        let len = intervals.len();
        intervals.sort();
        let mut i = 0;
        let mut ans: Vec<Vec<i32>> = Vec::new();
        while i < len {
            let left = intervals[i][0];
            let mut right = intervals[i][1];
            while i+1 < len && right >= intervals[i+1][0] {
                right = std::cmp::max(right, intervals[i+1][1]);
                i += 1;
            }
            ans.push(vec![left,right]);
            i+=1;
        }
        return ans;
    }   
}