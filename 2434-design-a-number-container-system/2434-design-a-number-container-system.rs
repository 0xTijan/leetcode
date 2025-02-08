use std::collections::{HashMap, BTreeSet};


struct NumberContainers {
    num_indexes: HashMap<i32, BTreeSet<i32>>,   // num -> index[]
    index_values: HashMap<i32, i32>             // index -> num
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {

    fn new() -> Self {
        NumberContainers {
            num_indexes: HashMap::new(),
            index_values: HashMap::new()
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(old_val) = self.index_values.get(&index) {
            // index already had a value, so remove from old num_index[old_val]
            if let Some(set) = self.num_indexes.get_mut(&old_val) {
                // remove index from the the num->index set
                set.remove(&index);
                
                // if set is empty remove from hashmap
                if set.is_empty() {
                    self.num_indexes.remove(&old_val);
                }
            }
        }

        // update index_values (for new value)
        self.index_values.insert(index, number);
        // update num_indexes (for new value)
        self.num_indexes.entry(number).or_insert(BTreeSet::new()).insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        // get the BTreeSet for number from hashmap
        if let Some(set) = self.num_indexes.get(&number) {
            // get the lowest number (= lowest index)
            if let Some(&lowest_index) = set.iter().next() {
                return lowest_index;
            }
        }
        // could not find it - reuturn -1
        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */