pub struct Solution;

impl Solution {
    use std::cmp;
    use std::collections::HashMap;

    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp;
        use std::collections::HashMap;

        let mut result = 0;
        let mut counter = HashMap::new();

        // 
        //
        //
    }

    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
        use std::cmp;
        use std::collections::HashMap;

        let mut result = Vec::new();
        let mut counter = HashMap::new();

        let mut tar = target;

        for (i, val) in nums.into_iter().enumerate(){
            let remainder = tar - val;
            if !counter.contains_key(&remainder){
                counter.insert(val, i as i32);
            } else {
                result.push(counter[&remainder]);
                result.push(i as i32);
            }
        }
    }
}
