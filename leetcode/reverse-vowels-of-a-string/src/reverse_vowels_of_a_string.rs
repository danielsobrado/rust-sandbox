struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-vowels-of-a-string/
// Given a string s, reverse only all the vowels in the string and return it.
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both cases.

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
}

#[test]
fn test2() {
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
}
