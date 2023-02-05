// https://leetcode.com/problems/valid-palindrome/
//
//
pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .into_iter()
            .filter(|e| e.is_ascii_alphanumeric())
            .map(|e| e.to_ascii_lowercase())
            .collect::<String>();
        let s_rv = s.chars().clone().into_iter().rev().collect::<String>();
        // println!("{s:?}");
        s == s_rv
    }
}
