// https://leetcode.com/problems/two-sum/

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut tar = target;
        let mut ht = HashMap::new();
        for (i, &e) in nums.iter().enumerate() {
            let remainder = tar - e;
            if !ht.contains_key(&remainder) {
                ht.insert(e, i as i32);
            } else {
                return vec![ht[&remainder], i as i32];
            }
        }
        vec![]
    }
}
