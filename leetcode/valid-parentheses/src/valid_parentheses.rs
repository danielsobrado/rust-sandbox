struct Solution;

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

impl Solution {
    pub fn is_valid(s: String) -> bool {

        if (s.len() % 2) != 0 {
            return false;
        }

        if s.is_empty() {
            return true;
        }

        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                _ => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
}

#[test]
fn test2() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
}

#[test]
fn test3() {
    assert_eq!(Solution::is_valid(String::from("(]")), false);
}

#[test]
fn test4() {
    assert_eq!(Solution::is_valid(String::from("((")), false);
}

#[test]
fn test5() {
    assert_eq!(Solution::is_valid(String::from("((")), false);
}