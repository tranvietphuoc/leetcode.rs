// https://leetcode.com/problems/3sum/

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let mut result = Vec::new();

        let mut arr = nums;
        arr.sort();
        let length = arr.len();

        if length == 0 || (length == 1 && arr[length - 1] == 0) {
            return result;
        }

        for (i, &val) in arr.iter().enumerate() {
            // if current value is equal to the previous value, then move to next loop
            if i > 0 && val == arr[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, length - 1);
            while l < r {
                let three_sums = val + arr[l] + arr[r];
                match three_sums.cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        result.push(vec![val, arr[l], arr[r]]);
                        l += 1;
                        while arr[l] == arr[l - 1] && l < r {
                            l += 1;
                        }
                    }
                }
                /* if three_sums < 0 {
                    l += 1;
                } else if three_sums > 0 {
                    r -= 1;
                } else {
                    result.push(vec![val, arr[l], arr[r]]);
                    l += 1;
                    while arr[l] == arr[l - 1] && l < r {
                        l += 1
                    }
                } */
            }
        }

        result
    }
}
