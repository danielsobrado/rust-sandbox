struct Solution;

// Problem URL: https://leetcode.com/problems/valid-palindrome/
// Given a string s, return true if the s can be palindrome after deleting at most one character from it.

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
}

#[test]
fn test2() {
    assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
}

#[test]
fn test3() {
    assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
}
