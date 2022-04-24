struct Solution;

// Problem URL: https://leetcode.com/problems/valid-palindrome/
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing 
// all non-alphanumeric characters, it reads the same forward and backward. 
// Alphanumeric characters include letters and numbers.
// Given a string s, return true if it is a palindrome, or false otherwise.

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        
        let s = s.chars()
                .filter(|&c| c.is_alphanumeric())
                .map(|c| {c.to_ascii_lowercase()})
                .collect::<Vec<char>>();

        if s.len() <= 1 {
            return true;
        }

        let end = s.len() - 1;

        for i in 0..s.len() / 2 {
            if s[i] != s[end - i] {
                return false;
            }
        }
        true
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
}

#[test]
fn test2() {
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
}

#[test]
fn test3() {
    assert_eq!(Solution::is_palindrome(" ".to_string()), true);
}
