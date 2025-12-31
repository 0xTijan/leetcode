impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec!(vec!(1));

        if num_rows >= 2 {
            res.push(vec!(1, 1));
        }

        for row in 3..(num_rows+1) {
            let row = row as usize;
            let mut row_res: Vec<i32> = vec![1; row];
            let floor_half = (row / 2) - 1;             // floor
            let ceiling_half = (row / 2 + row % 2) - 1; // ceiling
            for i in 1..(ceiling_half+1) {
                // sum upper parts
                let val = res[row-2][i-1] + res[row-2][i];
                // save to appropriate places in array
                row_res[i] = val;
                if i != ceiling_half || row % 2 == 0{
                    row_res[row-1-i] = val;
                }
            }
            res.push(row_res);
        }

        res
    }
}