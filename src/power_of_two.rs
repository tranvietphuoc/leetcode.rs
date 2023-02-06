// https://leetcode.com/problems/power-of-two/
pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        // let n = n as u32;
        let sqrt = f32::sqrt(n as f32).ceil() as u32;
        for i in 1..=sqrt {
            if 2i32.pow(i) == n {
                return true;
            }
        }

        false
    }
}
