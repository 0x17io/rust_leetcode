// https://leetcode.com/problems/unique-number-of-occurrences/

use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut heap: HashMap<i32, i32> = HashMap::new();
        
        for num in arr {
            if !heap.contains_key(&num) {
                heap.insert(num, 1);
            }
            else {
                heap.insert(num, heap[&num] + 1);
            }
        }


        let mut stack: Vec<i32> = Vec::new();

        for item in heap.values() {
            if !stack.contains(item) {
                stack.push(*item);  
            }
            else {
                return false;
            } 
        }

        true
    }
}
