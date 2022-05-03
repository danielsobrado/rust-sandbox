struct Solution;

// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".

impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let str = longest_str(&strs);

        let mut result: Vec<char> = Vec::with_capacity(str.len());

        for (i, c) in str.chars().enumerate() {
            let found: bool = strs.clone().into_iter()  
                .map(|it| { 
                    // println!("{} - {}", it, i);
                    it.as_bytes()[i] as char == c })
                .reduce(|acc, mk | acc && mk).unwrap();
            if found {
                result.push(c);
            } else {
                break;
            }
        }
        result.iter().collect()
    }
}

pub fn longest_str(strs: &Vec<String>) -> String {
    if strs.len() == 0 {
        return String::new();
    }
    
    let name = strs.iter().fold(&strs[0], |acc, item| {
        if item.len() < acc.len() {
            item
        } else {
            acc
        }
    });

    name.to_string()
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
