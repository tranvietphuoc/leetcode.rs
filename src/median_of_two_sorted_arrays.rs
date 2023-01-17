// https://leetcode.com/problems/median-of-two-sorted-arrays/
//
//

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // merge two arrays
        let mut nums = Vec::new();
        let mut i = 0_usize;
        let mut j = 0_usize;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                nums.push(nums1[i]);
                i += 1;
            } else {
                nums.push(nums2[j]);
                j += 1;
            }
        }

        if i < nums1.len() {
            nums.extend_from_slice(&nums1[i..]);
        } else if j < nums2.len() {
            nums.extend_from_slice(&nums2[j..]);
        }

        println!("{:?}", nums);

        let size = nums.len();
        let mid = size / 2;

        if size % 2 == 0 {
            return ((nums[mid] as f64) + (nums[mid - 1] as f64)) / 2.0;
        } else {
            return nums[mid] as f64;
        }
    }
}
