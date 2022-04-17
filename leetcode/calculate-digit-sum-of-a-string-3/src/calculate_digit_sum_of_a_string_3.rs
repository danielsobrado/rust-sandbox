struct Solution;


// You are given a string s consisting of digits and an integer k.
// A round can be completed if the length of s is greater than k. In one round, do the following:
// Divide s into consecutive groups of size k such that the first k characters are in the first group, 
// the next k characters are in the second group, and so on. Note that the size of the last group can be smaller than k.
// Replace each group of s with a string representing the sum of all its digits. 
// For example, "346" is replaced with "13" because 3 + 4 + 6 = 13.
// Merge consecutive groups together to form a new string. If the length of the string is greater than k, repeat from step 1.
// Return s after all rounds have been completed.
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {

        if s.len() <= k as usize {
            return s;	
        }

        
        // Divide in groups of k
        let mut groups = vec![];
        let mut group = String::new();
        for c in s.chars() {
            group.push(c);
            if group.len() == k as usize {
                groups.push(group);
                group = String::new();
            }
        }
        if !group.is_empty() {
            groups.push(group);
        }
        
        // Replace each group with the sum of its digits
        let mut groups2 = vec![String::from(""); groups.len()];   // Create a new vector with the same length as groups to avoid borrow checker issues
        for (i, group) in groups.iter_mut().enumerate() {
            let mut sum = 0;
            for c in group.chars() {
                sum += c.to_digit(10).unwrap();
            }
            println!("Group {} - {}: {} ", i, group, sum);
            groups2[i] = sum.to_string();
        }
    
        // Merge groups
        let mut result = String::new();
        for group in groups2 {
            println!("Merging {}", group);
            result.push_str(&group);
        }
        
        // Repeat until no more groups
        if result.len() > k as usize {
            println!("Iterate over {}", result);
            Solution::digit_sum(result, k)
        } else {
            result
        }

    }
}

#[test]
fn test1() {
    assert_eq!(Solution::digit_sum(String::from("11111222223"), 3), String::from("135"));
}

#[test]
fn test2() {
    assert_eq!(Solution::digit_sum(String::from("00000000"), 3), String::from("000"));
}

#[test]
fn test3() {
    assert_eq!(Solution::digit_sum(String::from("233"), 3), String::from("233"));
}

#[test]
fn test4() {
    assert_eq!(Solution::digit_sum(String::from("11111222223"), 3), String::from("135"));
}
