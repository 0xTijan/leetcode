impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_col: i8 = (grid[0].len() - 1) as i8; // max index for columns
        let all_rows = grid.len() - 1;
        let mut counter: usize = 0;

        // loop through rows and cols
        for (row_index, row) in grid.iter().enumerate() {
            for (col_index, &val) in row.iter().take((max_col + 1) as usize).enumerate() { // only loop until max_col index is reached
                if (max_col < 0) {
                    return counter as i32
                }
                if (val < 0) {
                    // formula - look at notes (adding all elements below and to the right to the max col (unchecked col))
                    let cols_to_right = ((max_col as usize) - col_index) + 1;
                    counter += (all_rows - row_index + 1) * cols_to_right;
                    max_col -= cols_to_right as i8;
                    break; // all cols from this point onward are already counted (no need to loop more)
                }
                println!("{}, {}, {}, {}", row_index, col_index, max_col, counter);
            }
        }

        counter as i32
    }
}