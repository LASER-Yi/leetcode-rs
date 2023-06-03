use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }

        let chars = s
            .chars()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, char)>>();

        let size = s.len();
        let mut longest = 0;

        let mut contain: HashSet<char> = HashSet::new();

        for &(start_idx, value) in chars.iter() {
            if (size - start_idx) <= longest {
                break;
            }

            contain.clear();
            contain.insert(value);

            for &(idx, current_value) in chars.iter().skip(start_idx + 1) {
                if contain.contains(&current_value) {
                    let len = idx - start_idx;
                    longest = longest.max(len);
                    break;
                }

                contain.insert(current_value);

                // EOF
                if idx + 1 >= size {
                    longest = longest.max(contain.len());
                }
            }
        }

        return longest as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(s: &'static str, result: i32) {
        assert_eq!(
            Solution::length_of_longest_substring(s.to_string()),
            result,
            "Str '{s}' should result in {result}"
        );
    }

    #[test]
    fn cases() {
        run_test("abcabcbb", 3);
        run_test("aaabcd", 4);
        run_test("bbbbb", 1);
        run_test("pwwkew", 3);
        run_test("au", 2);
        run_test("aab", 2);
    }
}
