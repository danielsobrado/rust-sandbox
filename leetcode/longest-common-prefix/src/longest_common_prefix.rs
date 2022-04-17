struct Solution;

// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        String::from("")
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")]), String::from("fl"));
}

#[test]
fn test2() {
    assert_eq!(Solution::longest_common_prefix(vec![String::from("dog"),String::from("racecar"),String::from("car")]), String::from(""));
}
