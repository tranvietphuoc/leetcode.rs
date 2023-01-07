#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0_i32;
        }

        use std::cmp;
        use std::collections::HashSet;

        let mut char_set = HashSet::new();

        let mut left = 0;
        let mut result = 0;
        let st = s.chars().into_iter().collect::<Vec<_>>();

        for (right, chr) in st.iter().enumerate() {
            while char_set.contains(chr) {
                char_set.remove(&st[left]);
                left += 1;
            }

            char_set.insert(*chr);
            result = cmp::max(result, right - left + 1);
        }

        result as i32
    }
}
