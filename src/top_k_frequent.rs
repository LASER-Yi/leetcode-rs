use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: Vec<(i32, i32)> = Vec::<(i32, i32)>::with_capacity(nums.len());

        nums.iter().for_each(|value| {
            let mut create = true;
            for (key, count) in map.iter_mut() {
                if *key == *value {
                    std::ops::AddAssign::add_assign(count, 1);
                    create = false;
                    break;
                }
            }

            if create {
                map.push((*value, 1));
            }
        });

        map.sort_unstable_by_key(|&(_, count)| count);
        return map
            .iter()
            .map(|&(key, _)| key)
            .rev()
            .take(k as usize)
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn run_test(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        let expected: HashSet<i32> = HashSet::<i32>::from_iter(expected.into_iter());

        let result = Solution::top_k_frequent(nums.clone(), k);
        let result: HashSet<i32> = HashSet::<i32>::from_iter(result.into_iter());

        assert_eq!(
            result, expected,
            "The result of arr {:?} is not an expected value",
            nums
        );
    }

    #[test]
    fn cases() {
        run_test(vec![10, 10, 10, 22, 22, 33], 2, vec![10, 22]);
        run_test(vec![1], 1, vec![1]);
        run_test(vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]);
    }
}
