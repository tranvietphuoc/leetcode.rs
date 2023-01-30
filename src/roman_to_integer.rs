pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::{HashMap, VecDeque};

        let mut roman = HashMap::new(); // HashMap<char, i32>
        roman.insert('I', 1);
        roman.insert('V', 5);
        roman.insert('X', 10);
        roman.insert('L', 50);
        roman.insert('C', 100);
        roman.insert('D', 500);
        roman.insert('M', 1000);

        let mut d = s.chars().into_iter().collect::<VecDeque<_>>();
        let mut result = 0;

        while d.len() > 0 {
            let current = d.pop_front().unwrap(); // get the first element of vecdeque

            // then compare to the next element
            if d.len() > 0 && roman[&d[0]] > roman[&current] {
                let next = d.pop_front().unwrap();
                result += roman[&next] - roman[&current];
                // d.pop_front();
            } else {
                result += roman[&current];
            }
        }

        result
    }
}
