struct Solution;

// Problem URL: https://leetcode.com/contest/biweekly-contest-78/problems/find-the-k-beauty-of-a-number/
// The k-beauty of an integer num is defined as the number of substrings of num when 
// it is read as a string that meet the following conditions:
// It has a length of k.
// It is a divisor of num.
// Given integers num and k, return the k-beauty of num.
// Note: Leading zeros are allowed. 0 is not a divisor of any value. 
// A substring is a contiguous sequence of characters in a string.

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let num_str: String = num.to_string();
        let num_len = num_str.len();
        let mut beauty: i32 = 0;

        
        for i in 0..num_len {
            if i+k as usize > num_len {
                break;
            }
            let substr = num_str.get(i..i+k as usize).unwrap();
            let number = substr.parse::<i32>().or_else(|e| Err(e)).unwrap();
            if number == 0 {
                continue;
            }
            if num % number == 0 {
                beauty += 1;
            }
        };
        beauty
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::divisor_substrings(240, 2), 2);
}

#[test]
fn test2() {
    assert_eq!(Solution::divisor_substrings(430043, 2), 2);
}
