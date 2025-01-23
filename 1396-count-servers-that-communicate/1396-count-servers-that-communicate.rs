impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_counts = vec![0; grid.len()];
        let mut col_counts = vec![0; grid[0].len()];
        let mut total_servers = 0;

        // count servers in each row and column
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    row_counts[r] += 1;
                    col_counts[c] += 1;
                    total_servers += 1;
                }
            }
        }

        // count isolated servers - no server in row and column
        let mut isolated_servers = 0;
        for r in 0..grid.len() {    
            for c in 0..grid[0].len() {
                // if server is found and sum of servers in row and columns is 1 (only this server)
                if grid[r][c] == 1 && row_counts[r] == 1 && col_counts[c] == 1 {
                    isolated_servers += 1;
                }
            }
        }

        total_servers - isolated_servers
    }
}