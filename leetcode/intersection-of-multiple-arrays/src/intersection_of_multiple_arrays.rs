struct Solution;

// Problem URL: https://leetcode.com/problems/intersection-of-multiple-arrays/
// Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, 
// return the list of integers that are present in each array of nums sorted in ascending order.

use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {

        let l = nums.len() as i32;

        if l == 0 {
            return vec![];
        } else if l == 1 {
            let result = nums[0].clone().sort();
            result
        }

        let mut nums_map:HashMap<i32, i32> = HashMap::new();

        for nums_slice in nums {
            for num in nums_slice {
                // Count no. of occurences of each number in hashmap
                *nums_map.entry(num).or_insert(0) += 1;
            }
        }

        let mut result = nums_map.into_iter()
                                    .filter(|&(_, v)|v == l)
                                    .map(|(i, _v)|{i})
                                    .collect::<Vec<i32>>();
        
        result.sort();
        result

    }
}

#[test]
fn test1() {
    assert_eq!(Solution::intersection(vec![vec![3,1,2,4,5], vec![1,2,3,4], vec![3,4,5,6]]), vec![3, 4]);
}

#[test]
fn test2() {
    assert_eq!(Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]), vec![]);
}
