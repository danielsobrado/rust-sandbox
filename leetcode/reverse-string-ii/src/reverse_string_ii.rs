struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-string-ii/
// Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.
// If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, 
// then reverse the first k characters and leave the other as original.

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::reverse_str(String::from("abcdefg"), 2), String::from("bacdfeg"));
}

#[test]
fn test2() {
    assert_eq!(Solution::reverse_str(String::from("abcd"), 2), String::from("bacd"));
}
