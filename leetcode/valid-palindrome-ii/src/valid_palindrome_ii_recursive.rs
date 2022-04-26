struct Solution;

// Problem URL: ht// Problem URL: https://leetcode.com/problems/valid-palindrome-ii/
// Given a string s, return true if the s can be palindrome after deleting at most one character from it.

impl Solution {

    pub fn is_palindrome(s: Vec<char>) -> bool {
        for i in 0..(s.len()/2) {
            // println!("Compare: {} - {}", s[i], s[s.len()-i-1]);
            if s[i] != s[s.len() - i-1] {
                return false;
            }
        }
        true
    }

    pub fn valid_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }

        for i in 0..s.len() {
            let mut s1 = s.chars().collect::<Vec<char>>();
            s1.remove(i);
            
            println!("Iterate over: {:?} ", s1);
            if Solution::is_palindrome(s1) {
                return true;
            }
        }
        false
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
