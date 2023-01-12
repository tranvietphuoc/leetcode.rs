// https://leetcode.com/problems/container-with-most-water/

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;

        let arr = height.as_slice();

        let mut s = 0;
        let mut i: usize = 0;
        let mut j: usize = arr.len() - 1;

        while i < j {
            let h = cmp::min(arr[i], arr[j]);
            let w = (j - i) as i32;
            s = cmp::max(s, h * w);

            while (arr[i] <= h) && (i < j) {
                i += 1;
            }

            while (arr[j] <= h) && (i < j) {
                j -= 1;
            }
            // println!("{} {} {}", i, j, s);
        }
        s
    }
}
