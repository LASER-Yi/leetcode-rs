use super::Solution;

const BARRIER_MAX: i32 = i32::MAX / 10;
const BARRIER_MIN: i32 = i32::MIN / 10;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut original = x;
        let mut result = 0;

        while original != 0 {
            if result > BARRIER_MAX || result < BARRIER_MIN {
                return 0;
            }

            result = result * 10 + (original % 10);
            original /= 10;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(input: i32, expected: i32) {
        assert_eq!(Solution::reverse(input), expected);
    }

    #[test]
    fn cases() {
        run_test(123, 321);
        run_test(-123, -321);
        run_test(0, 0);
        run_test(i32::MAX - 1, 0);
        run_test(i32::MIN + 1, 0);
    }
}
