https://leetcode.com/problems/integer-to-roman/description/

pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let ones = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let tens = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let hundreds = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let thousands = vec!["", "M", "MM", "MMM"];


        let mut roman = String::new();
        let mut num = num;

        roman.push_str(thousands[(num / 1000) as usize]);
        num %= 1000;
        roman.push_str(hundreds[(num / 100) as usize]);
        num %= 100;
        roman.push_str(tens[(num / 10) as usize]);
        num %= 10;
        roman.push_str(ones[(num) as usize]);

        println!("{:?}", &roman);

        roman
    }
}
