// https://leetcode.com/problems/longest-palindromic-substring/description/
//
//
pub struct Solution;

impl Solution {
    /* pub fn longest_palindrome(s: String) -> String {
        let mut low: usize;
        let mut high: usize;
        let length = s.len();
        let mut start: usize = 0;
        let mut max_length: usize = 1;
        let cloned = s.chars().clone().collect::<Vec<_>>();

        for i in 1..length {
            // even palindrome (length is even)
            low = i - 1;
            high = i;
            // use .get() method because I'm unsure about bounds check of vector cloned
            while low >= 0 && high < length && cloned.get(low) == cloned.get(high) {
                if high - low + 1 > max_length {
                    start = low;
                    max_length = high - low + 1;
                }

                low -= 1;
                high += 1;
            }

            // odd palindrome (length is odd)
            low = i - 1;
            high = i + 1;
            // use .get() method because I'm unsure about bounds check of vector cloned
            while low >= 0 && high < length && cloned.get(low) == cloned.get(high) {
                if high - low + 1 > max_length {
                    start = low;
                    max_length = high - low + 1;
                }

                low -= 1;
                high += 1;
            }
        }

        if start == 0 && max_length == 0 {
            return cloned[0].to_string();
        }
        // println!("start {}, max_length {}", start, max_length);

        cloned[start..start + max_length]
            .into_iter()
            .collect::<String>()
    } */

    pub fn longest_palindrome(s: String) -> String {
        for size in (1..=s.len()).rev() {
            // use .windows() for slice taken.
            match s
                .as_bytes()
                .windows(size)
                .find(|substring| Solution::check_palindrome(substring))
            {
                Some(palindrome) => return String::from_utf8(palindrome.to_vec()).unwrap(),
                None => continue,
            }
        }

        "".to_string()
    }

    fn check_palindrome(s: &[u8]) -> bool {
        s.iter()
            .zip(s.iter().rev())
            .all(|(left, right)| left == right)
    }
}
