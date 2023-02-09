// https://leetcode.com/problems/4sum/description/

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        return Self::k_sum(nums, target, 0_usize, 4);
    }

    fn k_sum(nums: Vec<i32>, target: i32, start: usize, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let numbers = nums.clone();

        // constraints:
        let num_lo_bound = -1 * 10_i32.pow(9);
        let num_up_bound = 10_i32.pow(9);

        // check len constraint
        if nums.len() > 200 || nums.len() < 1 {
            return res;
        }

        if target < num_lo_bound || target > 2 * num_up_bound {
            return res;
        }

        if start == numbers.len() {
            return res;
        }

        let average_value = target / k;
        if numbers[start] > average_value || average_value > *numbers.last().unwrap() {
            return res;
        }

        if k == 2 {
            return Self::two_sum(nums.clone(), target, start);
        }

        for i in start..numbers.len() {
            if numbers[i] > num_up_bound || numbers[i] < num_lo_bound {
                return res;
            }

            if i == start || numbers[i - 1] != numbers[i] {
                for subset in Self::k_sum(nums.clone(), target - numbers[i], i + 1, k - 1) {
                    let mut sub = subset;

                    sub.sort();
                    res.push(vec![numbers[i]]);

                    res.last_mut().unwrap().append(&mut sub);
                }
            }
        }

        res
    }

    fn two_sum(nums: Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        let (mut lo, mut hi) = (start, nums.len() - 1);

        while lo < hi {
            let curr_sum = nums[lo] + nums[hi];
            if curr_sum < target || (lo > start && nums[lo] == nums[lo - 1]) {
                lo += 1;
            } else if curr_sum > target || (hi < nums.len() - 1 && nums[hi] == nums[hi + 1]) {
                hi -= 1;
            } else {
                res.push(vec![nums[lo], nums[hi]]);
                lo += 1;
                hi -= 1;
            }
        }
        res
    }
}
