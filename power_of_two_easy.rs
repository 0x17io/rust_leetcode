// https://leetcode.com/problems/power-of-two

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {

        // log(0) breaks my brain.
        if n == 0 {
            return false;
        }
        
        ((n as f64).log2() / 2_f64.log2()).trunc() == (n as f64).log2() / 2_f64.log2()
    }
}
