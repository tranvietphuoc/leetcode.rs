// https://leetcode.com/problems/number-of-1-bits/
//
pub struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> u32 {
        n.count_ones()
    }
}
