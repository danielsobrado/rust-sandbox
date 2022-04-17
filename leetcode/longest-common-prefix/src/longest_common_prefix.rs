struct Solution;

// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".

impl Solution {

    pub fn longest_str(strs: Vec<String>) -> String {
        let str = strs.iter().fold(strs[0], |acc, &item| {
            if str.len() > acc.len() {
                str
            } else {
                acc
            }
        });
    
        return Some(str);
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let str = longest_str(strs).unwrap_or("");

        let result: Vec<char> = Vec::with_capacity(str.len());

        for (i, c) in str.chars().enumerate() {
            let val: bool = strs.iter().chars.collect()
                .inspect(|x| println!("about to filter: {}", x))
                .filter(|&x| x[i] == c)
                .inspect(|x| println!("made it through filter: {}", x))
                .fold(false, |false, i| sum + i);
        }
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
