use crate::Solution;

const OPEN_SYMBOLS: [char; 3] = ['(', '{', '['];
const CLOSE_SYMBOLS: [char; 3] = [')', '}', ']'];

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::<char>::with_capacity(s.len());

        for value in s.chars() {
            match CLOSE_SYMBOLS.iter().position(|&r| r == value) {
                Some(pos) => {
                    let open = OPEN_SYMBOLS[pos];
                    if stack.last() == Some(&open) {
                        stack.pop();
                        continue;
                    }

                    return false;
                }
                _ => stack.push(value),
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(s: &'static str, result: bool) {
        assert_eq!(
            Solution::is_valid(s.to_string()),
            result,
            "Str '{s}' should result in {result}"
        )
    }

    #[test]
    fn cases() {
        run_test("(", false);
        run_test("()", true);
        run_test(")(", false);
        run_test("(){}[]", true);
        run_test("([{}])", true);
        run_test("({}[])", true);
        run_test("(]", false);
        run_test("([)]", false);
        run_test("({[)", false);
    }
}
