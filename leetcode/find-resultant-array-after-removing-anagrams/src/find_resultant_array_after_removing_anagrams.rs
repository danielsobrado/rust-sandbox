struct Solution;

// Problem URL: https://leetcode.com/contest/weekly-contest-293/problems/find-resultant-array-after-removing-anagrams/
// You are given a 0-indexed string array words, where words[i] consists of lowercase English letters.
// In one operation, select any index i such that 0 < i < words.length and words[i - 1] and words[i] 
// are anagrams, and delete words[i] from words. Keep performing this operation as long as you can select 
// an index that satisfies the conditions.
// Return words after performing all operations. It can be shown that selecting the indices for each 
// operation in any arbitrary order will lead to the same result.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase 
// using all the original letters exactly once. For example, "dacb" is an anagram of "abdc".

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        vec![]
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::remove_anagrams(vec!["abba".to_string(), "baba".to_string(), "bbaa".to_string(), "cd".to_string(), "cd".to_string()]), vec!["abba".to_string(), "cd".to_string()]);
}

#[test]
fn test2() {
    assert_eq!(Solution::remove_anagrams(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]), vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]);
}
