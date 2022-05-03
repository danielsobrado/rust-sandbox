struct Solution;

// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".
// Solution: https://leetcode.com/problems/longest-common-prefix/discuss/1960002/Rust-Functional-Solution
impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        if strs.is_empty() {
            return "".to_string();
        }

        let str = strs[0].clone();

        let mut count: i32 = 0;

        for (i, c) in str.chars().enumerate() {
            let found: bool = strs.clone().into_iter() 
                .inspect(|it| { print!("Pre: {} Len: {} i: {} Count: {} ", it, it.len(), i, count); })
                .filter_map(|it| -> Option<bool> { 
                    if it.len() > i as usize {
                        Some(it.as_bytes()[i] as char == c) 
                    } else {
                        Some(false)
                    }
                })
                // .map(|it| { it == c })
                .inspect(|it| { println!("Post Map: {} ", it.to_string()); })
                // .reduce(|acc, mk | acc && mk).unwrap_or(false);
                .fold(true, |acc, mk | {
                    println!("Fold -> Acc: {} Mk: {} Folded: {}", acc, mk, acc && mk);
                    acc && mk
                });
            if found {
                count += 1;
                println!("Result ++1: {}", count);
            } else {
                break;
            }
        }
        
        str.chars().take(count as usize).collect()
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

#[test]
fn test3() {
    assert_eq!(Solution::longest_common_prefix(vec![String::from("cir"),String::from("car")]), String::from("c"));
}

#[test]
fn test4() {
    assert_eq!(Solution::longest_common_prefix(vec![String::from("ab"),String::from("a")]), String::from("a"));
}
