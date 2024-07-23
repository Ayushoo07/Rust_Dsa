impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let row = matrix.len();
        let col = matrix[0].len();
        for i in 0..(row-1) {
            for j in i+1..col{
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for i in 0..row {
            let mut left = 0;
            let mut right = col-1;
            while left < right {
                let tmp = matrix[i][left];
                matrix[i][left] = matrix[i][right];
                matrix[i][right] = tmp;
                left+=1;
                right-=1;
            }
        }
    }
}