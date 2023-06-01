use crate::Solution;

#[allow(dead_code)]
impl Solution {
    fn contains_duplicate_simple(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut nums = nums.clone();

        nums.sort_unstable();
        nums.dedup();

        return size != nums.len();
    }

    fn contains_duplicate_fast(nums: Vec<i32>) -> bool {
        let sorted_nums = {
            let mut arr = nums.clone();
            arr.sort_unstable();
            arr
        };

        return sorted_nums
            .windows(2)
            .filter(|arr| arr.len() == 2)
            .any(|slice| slice.first() == slice.last());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn run_test(nums: Vec<i32>, expected: bool) {
        assert_eq!(
            Solution::contains_duplicate_simple(nums.clone()),
            expected,
            "The vector {:?} results is not expected {} (simple)",
            nums,
            expected
        );

        assert_eq!(
            Solution::contains_duplicate_fast(nums.clone()),
            expected,
            "The vector {:?} results is not expected {} (set)",
            nums,
            expected
        );
    }

    #[test]
    fn cases() {
        run_test(vec![1], false);
        run_test(vec![1, 2, 3, 1], true);
        run_test(vec![1, 2, 3, 4], false);
    }
}
