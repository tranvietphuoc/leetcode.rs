mod add_two_numbers;
mod binray_search_tree;
mod container_with_most_water;
mod four_sum;
mod length_of_longest_substring;
mod median_of_two_sorted_arrays;
mod merge_two_sorted_lists;
mod reverse_integer;
mod three_sum;
// mod three_sum_closest;
// mod depth_first_search;
mod multiply_string;
mod two_sum;
mod zigzag_conversion;

// use crate::container_with_most_water;
// use crate::length_of_longest_substring;
// use crate::merge_two_sorted_lists::{self, ListNode};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area() {
        let v = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = container_with_most_water::Solution::max_area(v);
        assert_eq!(49, result);
    }

    /* #[test]
    fn test_merge_two_list() {
        let mut l1 = Box::new(ListNode::new(0));
        let mut ptrl1 = &mut l1.as_ref();
        for _ in 0..3 {
            let tmp = Box::new(ListNode::new(1));
            ptrl1.as_ref().next = tmp;
        }
    } */

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            length_of_longest_substring::Solution::length_of_longest_substring(
                "pwwkwe".to_string()
            ),
            3
        );
    }

    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(
            zigzag_conversion::Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn test_add_two_numbers() {
        // unimplemented!()
    }

    #[test]
    fn test_three_sum_closest() {
        // unimplemented!();
        /* let a = "hello";
        let resonning = a.to_owned();
        println!("{a}"); */
    }

    #[test]
    fn test_reverse_integer() {
        assert_eq!(reverse_integer::Solution::reverse(123), 321);
    }

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            three_sum::Solution::three_sum(nums),
            vec![[-1, -1, 2], [-1, 0, 1]]
        )
    }

    #[test]
    fn test_median_two_sorted_arrays() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        assert_eq!(
            median_of_two_sorted_arrays::Solution::find_median_sorted_arrays(nums1, nums2),
            2.50000
        );
    }

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];

        assert_eq!(two_sum::Solution::two_sum(nums, 9), vec![0, 1])
    }

    #[test]
    fn test_four_sum() {
        let nums1 = vec![1, 0, -1, 0, -2, 2];
        let target1 = 0;
        let nums2 = vec![2, 2, 2, 2, 2];
        let target2 = 8;
        assert_eq!(
            four_sum::Solution::four_sum(nums1, target1),
            vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
        assert_eq!(
            four_sum::Solution::four_sum(nums2, target2),
            vec![[2, 2, 2, 2]]
        );
    }
}
