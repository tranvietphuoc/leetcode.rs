// https://leetcode.com/problems/add-digits/

pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if (-9..9).contains(&num) {
            return num;
        }

        let mut flag: i32 = 1;

        if num < 0 {
            flag = -1;
        }

        let mut n = num.to_string().chars().into_iter().collect::<Vec<_>>();

        while n.len() > 1_usize {
            let temp_num = n
                .into_iter()
                .map(|e| e.to_digit(10).unwrap())
                .reduce(|a, b| a + b)
                .unwrap();

            println!("{}", &temp_num);
            n = temp_num.to_string().chars().into_iter().collect::<Vec<_>>();
        }
        let result = n.into_iter().collect::<String>().parse::<i32>().unwrap();
        if num < 0 {
            return flag * result;
        }
        result
    }

    /* pub fn add_digits(num: i32) -> i32 {
        match num {
            0 => return 0,
            _ => {
                num = num % 9;
                if num == 0 {
                    return 0;
                } else {
                    return num;
                }
            }
        }
    } */
}
