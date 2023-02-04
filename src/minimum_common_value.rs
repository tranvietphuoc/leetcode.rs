// https://leetcode.com/contest/biweekly-contest-96/problems/minimum-common-value/

pub struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let s1 = nums1.into_iter().collect::<HashSet<_>>();

        let s2 = nums2.into_iter().collect::<HashSet<_>>();

        let mut res = s1.intersection(&s2).into_iter().map(|e| *e).collect::<Vec<_>>();
        res.sort();

        if res.is_empty() {
            return -1;
        } else {
            return res[0];
        }
    }
}
