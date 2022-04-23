struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-words-in-a-string-iii/
// Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

impl Solution {
    pub fn reverse_words(s: String) -> String {

        if s.len() <= 1 {
            return s;
        }

        let mut s = s;
        let mut result = String::new();

        let mut end = s.find(' ').unwrap_or(s.len());

        loop {
            let word = &s[0..end].chars().rev().collect::<String>();
            result = result + word;
            
            if end != s.len() {
                s = s[end+1..].to_string();
                end = s.find(' ').unwrap_or(s.len());
                result.push(' ');
            } else {
                break;
            }
        }

        result
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::reverse_words(String::from("Let's take LeetCode contest")), String::from("s'teL ekat edoCteeL tsetnoc"));
}

#[test]
fn test2() {
    assert_eq!(Solution::reverse_words(String::from("God Ding")), String::from("doG gniD"));
}

#[test]
fn test3() {
    assert_eq!(Solution::reverse_words(String::from("Single")), String::from("elgniS"));
}
