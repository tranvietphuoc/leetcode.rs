// https://leetcode.com/problems/3sum/

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new(Vec::new());

        let mut l = nums;
        let length = l.len();

        if length == 0 || (length == 1 && l[length - 1] == 0) {
            return result;
        }

        result
    }
}
