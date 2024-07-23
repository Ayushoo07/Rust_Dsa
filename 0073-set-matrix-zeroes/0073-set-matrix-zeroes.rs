impl Solution {
   pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    // Initialize a variable to track if the first column should be set to zero
    let mut col0 = 1;
    
    // Traverse the matrix to mark the rows and columns that need to be set to zero
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            // If an element is zero, mark the corresponding row and column
            if matrix[i][j] == 0 {
                // Mark the first element of the row
                matrix[i][0] = 0;
                // If it's the first column, update col0
                if j == 0 {
                    col0 = 0;
                } else {
                    // Otherwise, mark the first element of the column
                    matrix[0][j] = 0;
                }
            }
        }
    }
    
    // Traverse the matrix again to set the rows to zero based on the markers
    for idx in 1..matrix.len() {
        if matrix[idx][0] == 0 {
            for j in 1..matrix[0].len() {
                matrix[idx][j] = 0;
            }
        }
    }
    
    // Traverse the matrix again to set the columns to zero based on the markers
    for idx in 1..matrix[0].len() {
        if matrix[0][idx] == 0 {
            for i in 1..matrix.len() {
                matrix[i][idx] = 0;
            }
        }
    }
    
    // If the first element of the matrix is zero, set the entire first row to zero
    if matrix[0][0] == 0 {
        for j in 0..matrix[0].len() {
            matrix[0][j] = 0;
        }
    }
    
    // If col0 is zero, set the entire first column to zero
    if col0 == 0 {
        for i in 0..matrix.len() {
            matrix[i][0] = 0;
        }
    }
}

}