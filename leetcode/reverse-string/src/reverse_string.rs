struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-string/
// Write a function that reverses a string. The input string is given as an array of characters s.
// You must do this by modifying the input array in-place with O(1) extra memory.
// You may assume all the characters consist of printable ascii characters.

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        
    }
}
#[test]
fn test1() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}

#[test]
fn test2() {
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}
