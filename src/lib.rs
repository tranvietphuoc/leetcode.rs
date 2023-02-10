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
mod add_digits;
mod depth_first_search;
mod integer_to_roman;
mod longest_common_prefix;
mod longest_palindrome;
mod minimum_common_value;
mod multiply_string;
mod number_of_1_bits;
mod power_of_two;
mod remove_duplicates_from_sorted_list;
mod remove_nth_node_from_end_of_list;
mod reverse_bits;
mod roman_to_integer;
mod single_number;
mod two_sum;
mod valid_palindrome;
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
    #[test]
    fn test_multiply_string() {
        assert_eq!(
            multiply_string::Solution::multiply("6913259244".to_string(), "71103343".to_string()),
            "491555843274052692".to_string()
        );
    }

    #[test]
    fn test_roman_to_int() {
        assert_eq!(
            roman_to_integer::Solution::roman_to_int("III".to_string()),
            3
        );
    }

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix::Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
    }

    #[test]
    fn test_integer_to_roman() {
        assert_eq!(
            integer_to_roman::Solution::int_to_roman(1994),
            "MCMXCIV".to_string()
        );

        assert_eq!(
            integer_to_roman::Solution::int_to_roman(58),
            "LVIII".to_string()
        );
    }

    #[test]
    fn test_minimum_common_value() {
        assert_eq!(
            minimum_common_value::Solution::get_common(vec![1, 2, 3], vec![2, 4]),
            2
        );
        assert_eq!(
            minimum_common_value::Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]),
            2
        );
    }

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            longest_palindrome::Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            longest_palindrome::Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn test_reverse_bits() {
        assert_eq!(
            reverse_bits::Solution::reverse_bits(0b00000010100101000001111010011100_u32),
            964176192
        );
    }

    #[test]
    fn test_number_of_1_bits() {
        assert_eq!(
            number_of_1_bits::Solution::hammingWeight(0b00000000000000000000000000001011_u32),
            3
        );
    }

    #[test]
    fn find_1_fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 99;

        let graph = depth_first_search::Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search::depth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn find_1_success() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = depth_first_search::Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search::depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_2_sucess() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 6;

        let correct_path = vec![0, 1, 3, 2, 4, 5, 7, 6];

        let graph = depth_first_search::Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search::depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_3_sucess() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 4;

        let correct_path = vec![0, 1, 3, 2, 4];

        let graph = depth_first_search::Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search::depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }
}
