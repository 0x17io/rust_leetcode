// https://leetcode.com/problems/valid-palindrome/s

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let stripped_rev: String = s.chars().filter(|c| c.is_alphanumeric()).rev().collect();
        let stripped: String = s.chars().filter(|c| c.is_alphanumeric()).collect();

        stripped_rev.to_lowercase() == stripped.to_lowercase()
    }
}

