struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-vowels-of-a-string/
// Given a string s, reverse only all the vowels in the string and return it.
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both cases.

impl Solution {

    pub fn reverse_vowels(s: String) -> String {

        let is_vowel = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U');
        
        let mut result: Vec<u8> = s.chars().map(|c| c as u8).collect();

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if is_vowel(result[left]) {
                if is_vowel(result[right]) {
                    result.swap(left, right);
                    left += 1;
                    right -= 1;
                } else {
                    right -= 1;
                }
            } else {
                left += 1;
            }
        }

        result.iter().map(|c| *c as char).collect()
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
