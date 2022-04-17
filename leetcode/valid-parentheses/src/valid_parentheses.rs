struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
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