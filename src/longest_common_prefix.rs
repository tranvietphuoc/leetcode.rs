pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = String::new();
        let first_word = &strs[0];
        let words_from_1 = &strs.clone()[1..];

        if strs.is_empty() {
            return common;
        }

        for (i, char) in first_word.chars().into_iter().enumerate() {
            for (_, nstr) in words_from_1.iter().enumerate() {
                let v_nstr = nstr.chars().into_iter().collect::<Vec<_>>();
                if v_nstr.is_empty() || v_nstr.len() - 1 < i || v_nstr[i] != char {
                    return common;
                }
            }
            common.push(char)
        }

        common
    }
}
