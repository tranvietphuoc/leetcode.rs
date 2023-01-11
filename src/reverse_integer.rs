// https://leetcode.com/problems/reverse-integer/description/
// 0ms runtime ^^

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num: i32;
        let mut result = 0;

        num = x.abs();

        while num > 0 {
            // println!("{result}, {num}");

            // check constraints with upper and lower bound of 32-bit integer
            if result < i32::MIN / 10 || result > i32::MAX / 10 {
                return 0;
            }

            let (m, d) = (num % 10, num / 10);
            result = result * 10 + m;
            num = d;
        }
        if x > 0 {
            return result;
        } else {
            return -1 * result;
        }
    }
}
