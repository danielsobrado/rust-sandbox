struct Solution;

// Problem URL: https://leetcode.com/problems/reverse-vowels-of-a-string/
// Given a string s, reverse only all the vowels in the string and return it.
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both cases.

impl Solution {

    pub fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        }
    }

    pub fn reverse_vowels(s: String) -> String {
        
        let mut result: Vec<u8> = s.chars().map(|c| c as u8).collect();

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if Solution::is_vowel(*result.get(left).unwrap() as char) {
                if Solution::is_vowel(*result.get(right).unwrap() as char) {
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
