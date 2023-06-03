use super::Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn char_to_int(roman: &char) -> i32 {
            match roman {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            }
        }

        let slice = s.chars().collect::<Vec<_>>();

        // 1 - parse the initial value
        let result = slice.iter().fold(0, |acc, roman| acc + char_to_int(&roman));

        let delta = slice
            .windows(2)
            .filter(|group| group.len() == 2)
            .map(|group| (group[0], group[1]))
            .fold(0, |acc, (first, second)| {
                let delta = match first {
                    'I' => match second {
                        'V' | 'X' => -2,
                        _ => 0,
                    },
                    'X' => match second {
                        'L' | 'C' => -20,
                        _ => 0,
                    },
                    'C' => match second {
                        'D' | 'M' => -200,
                        _ => 0,
                    },
                    _ => 0,
                };
                acc + delta
            });

        return result + delta;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(s: &'static str, expected: i32) {
        assert_eq!(
            Solution::roman_to_int(s.to_string()),
            expected,
            "Str '{s}' cannot be converted to expected value {expected}"
        );
    }

    #[test]
    fn cases() {
        run_test("III", 3);
        run_test("IV", 4);
        run_test("V", 5);
        run_test("VI", 6);
        run_test("LVIII", 58);
        run_test("MCMXCIV", 1994);
    }
}
