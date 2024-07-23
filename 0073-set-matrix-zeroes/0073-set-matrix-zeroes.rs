impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col0 = 1;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    if j == 0 {
                        col0 = 0;
                    } else {
                        matrix[0][j] = 0;    
                    }
                    
                }
            }
        }
        for idx in 1..matrix.len() {
            if matrix[idx][0] == 0 {
               for j in 1..matrix[0].len(){
                    matrix[idx][j] = 0;
               } 
            }
        }
        for idx in 1..matrix[0].len() {
            if matrix[0][idx] == 0 {
                for i in 1..matrix.len(){
                    matrix[i][idx] = 0;
                }
            }
        }
        
        if matrix[0][0] == 0 {
            for j in 0..matrix[0].len() {
                matrix[0][j] = 0;
            }
        }
        if col0 == 0 {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        } 

    }
}