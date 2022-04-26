struct Solution;

// Problem URL: https://leetcode.com/problems/valid-palindrome-ii/
// Given a string s, return true if the s can be palindrome after deleting at most one character from it.

// If two characters are not the same, we'll try removing the one on the right and the one on the left after.
// See: https://www.youtube.com/watch?v=WQTZQ6LHm0M

impl Solution {

    pub fn is_palindrome(s: &[u8]) -> bool {

        let mut j = 0;
        let mut i = s.len() - 1;

        while i > j {
            if s[i] != s[j] {
                return false;
            }
            j += 1;
            i -= 1;
        }
        return true;
    }

    pub fn valid_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }

        let s = s.as_bytes();

        let mut j = 0;
        let mut i = s.len() - 1;

        while i >= j {
            if s[i] != s[j] {
                if Self::is_palindrome(&s[j+1..i+1]) || Self::is_palindrome(&s[j..i]) {
                    return true;
                } else {
                    return false;
                }
            }
            i -= 1;
            j += 1;
        }
        true
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

#[test]
fn test4() {
    assert_eq!(Solution::valid_palindrome("deeee".to_string()), true);
}

#[test]
fn test5() {
    assert_eq!(Solution::valid_palindrome("eccer".to_string()), true);
}
