// https://leetcode.com/problems/zigzag-conversion/solutions/2868537/zigzag-conversion/

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut result = String::new();
        let n = s.len() as i32;
        let chars_in_section = 2 * num_rows - 2;
        let chars = s.chars().into_iter().collect::<Vec<char>>();

        for curr_row in 0..num_rows {
            let mut index = curr_row;

            while index < n {
                result.push(chars[index as usize]);

                // If currRow is not the first or last row
                // then we have to add one more character of current section.

                if curr_row != 0 && curr_row != num_rows - 1 {
                    let chars_in_between = chars_in_section - 2 * curr_row;
                    let second_index = index + chars_in_between;

                    if second_index < n {
                        result.push(chars[second_index as usize]);
                    }
                }
                // Jump to same row's first character of next section.
                index += chars_in_section;
            }
        }

        result
    }
}
