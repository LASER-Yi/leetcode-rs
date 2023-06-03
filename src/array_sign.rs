use crate::Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.iter().any(|&value| value == 0) {
            return 0;
        }

        let negative = nums.iter().filter(|value| value.is_negative()).count();

        match negative % 2 {
            0 => return 1,
            _ => return -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(nums: Vec<i32>, expected: i32) {
        assert_eq!(
            Solution::array_sign(nums.clone()),
            expected,
            "The result of input array {:?} is not a expected value {}",
            nums,
            expected
        )
    }

    #[test]
    fn cases() {
        run_test(vec![1; 100], 1);
        run_test(vec![-1, -2, -3, -4, 3, 2, 1], 1);
        run_test(vec![1, 5, 0, 2, -3], 0);
        run_test(vec![], 0);
        run_test(vec![-1, 1, -1, 1, -1], -1);
    }
}
