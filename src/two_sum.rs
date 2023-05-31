use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();

        for &(idx, value) in nums.iter() {
            for &(inn_idx, inn_value) in nums.iter().skip(idx + 1) {
                if (value + inn_value) == target {
                    return vec![idx as i32, inn_idx as i32];
                }
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cases() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
