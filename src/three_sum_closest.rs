// https://leetcode.com/problems/3sum-closest/

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;

        let mut nums = nums;
        nums.sort();
        let mut three_sum = 0;
        let length = nums.len();
        let mut mn = i32::MAX;

        let mut start: usize;
        let mut end: usize;
        let mut diff: i32;

        for i in 0..length {
            start = i + 1;
            end = length - 1;

            while start < end {
                three_sum = nums[i] + nums[start] + nums[end];
                diff = (three_sum - target).abs();

                if diff < mn {
                    mn = diff;
                    result = three_sum;
                }
                if three_sum > target {
                    end -= 1;
                } else {
                    start += 1;
                }
            }
        }

        result
    }
}
