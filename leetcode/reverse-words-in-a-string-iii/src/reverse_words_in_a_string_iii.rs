struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-words-in-a-string-iii/
// Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.


impl Solution {
    pub fn reverse_words(s: String) -> String {
        
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
