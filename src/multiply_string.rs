// https://leetcode.com/problems/multiply-strings/description/
// approach 2

pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0".to_string() || num2 == "0".to_string() {
            return "0".to_string();
        }

        // reverse string and convert to vector of u32s
        let mut first = num1
            .chars()
            .rev()
            .map(|e| e.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let mut second = num2
            .chars()
            .rev()
            .map(|e| e.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        // store the multiplication result of each digit of second number with first number
        let mut answer = vec![0; first.len() + second.len()];

        // println!("{:?}", first);
        // mutiply each digit in second number by the first number
        // and add each result to answer
        for (index, digit) in second.into_iter().enumerate() {
            answer = Self::add_strings(Self::multiply_one_digit(&first, digit, index), answer);
        }

        if *answer.last().unwrap() == 0 {
            answer.pop();
        }

        // reverse vec<u32> to get result and convert to string to get the final answer
        answer.into_iter().rev().map(|e| e.to_string()).collect()
    }

    fn add_strings(result: Vec<u32>, answer: Vec<u32>) -> Vec<u32> {
        let mut carry = 0_u32;

        let mut new_answer = Vec::new();

        let mut rs_it = result.iter();
        let mut ans_it = answer.iter();
        let mut zip = Vec::new();

        // create a zip_longest like
        loop {
            match (rs_it.next(), ans_it.next()) {
                (Some(x), Some(y)) => zip.push((*x, *y)),
                (Some(x), None) => zip.push((*x, 0)),
                (None, Some(y)) => zip.push((0, *y)),
                (None, None) => break,
            }
        }

        // iter through zip
        for (digit1, digit2) in zip {
            let curr_sum = digit1 + digit2 + carry;
            carry = curr_sum / 10;
            new_answer.push(curr_sum % 10);
        }

        new_answer
    }

    fn multiply_one_digit(first: &Vec<u32>, digit2: u32, num_zeros: usize) -> Vec<u32> {
        // insert 0s at the begining based on the current digit's place
        let mut current_result = vec![0; num_zeros];

        let mut carry = 0_u32;

        // multiply first number with current digit of second number
        for digit in first {
            let mul = *digit * digit2 + carry;

            // set carry to the tens place digit of multiplication
            carry = mul / 10;

            // append to current result
            current_result.push(mul % 10);
        }

        if carry != 0 {
            current_result.push(carry);
        }

        current_result
    }
}
