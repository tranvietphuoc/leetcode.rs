mod container_with_most_water;
mod length_of_longest_substring;
mod merge_two_sorted_lists;
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
}
