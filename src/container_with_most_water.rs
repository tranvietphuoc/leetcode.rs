pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;

        let arr = height.as_slice();

        let mut s = 0;
        let mut i: usize = 0_usize;
        let mut j: usize = arr.len() - 1_usize;

        while i < j {
            let h = cmp::min(arr[i], arr[j]);
            let w = (j - i) as i32;
            s = cmp::max(s, h * w);

            while (arr[i] <= h) && (i < j) {
                i += 1_usize;
            }

            while (arr[j] <= h) && (i < j) {
                j -= 1_usize;
            }
            // println!("{} {} {}", i, j, s);
        }
        s
    }
}
