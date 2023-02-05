// https://leetcode.com/problems/single-number/
//
pub struct Solution;

impl Solution {
    /* pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        for num in nums {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.1.cmp(&b.1));

        vec[0].0
    } */

    // other rust Solution
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().reduce(|a, b| a ^ b).unwrap()
    }
}
