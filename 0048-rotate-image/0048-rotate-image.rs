impl Solution {
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // Get the number of rows and columns in the matrix
    let row = matrix.len();
    let col = matrix[0].len();
    
    // Transpose the matrix (swap rows with columns)
    for i in 0..(row-1) {
        for j in i+1..col {
            // Swap elements matrix[i][j] and matrix[j][i]
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    // Reverse each row to get the final rotated matrix
    for i in 0..row {
        let mut left = 0;
        let mut right = col - 1;
        // Swap elements from left to right within the row
        while left < right {
            let tmp = matrix[i][left];
            matrix[i][left] = matrix[i][right];
            matrix[i][right] = tmp;
            left += 1;
            right -= 1;
        }
    }
}

}