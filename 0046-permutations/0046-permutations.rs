// backtraciking and recursion - swapping
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        Self::generate_permutations(&mut nums, 0, &mut res);
        return res;
    }

    pub fn generate_permutations(nums: &mut [i32], start: usize, out: &mut Vec<Vec<i32>>) {
        // end of tree was reached - save this permutation
        if start == nums.len() {
            out.push(nums.to_vec());
            return;
        }

        // create all possible permutations by swapping all possivble combinations
        for i in start..nums.len() {
            // swap and create next permutes
            nums.swap(start, i);    
            Self::generate_permutations(nums, start + 1, out);  // explore tree
            // swap back - backtrack
            nums.swap(start, i);
        }
    }
}