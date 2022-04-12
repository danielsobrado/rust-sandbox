
struct Solution;

use std::collections::HashMap;

// Problem:
// Given an array of integers nums and an integer target, 
// return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.
// You can return the answer in any order.
// Solution:
// Use a hashmap to store the values and their positions. 
// Loop through the array, and check if the complement is in the hashmap. 1 pass only.
// Complexity: O(n) time, O(n) space
impl Solution {
    fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (pos, e) in nums.iter().enumerate() {
            let complement = target - e;
            if map.contains_key(&complement) {
                return vec![map[&complement], pos as i32];
            }
            map.insert(*e, pos as i32);
        }
        vec![]
    }
}

#[test]
fn test1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum_hashmap(nums, target), vec![0, 1]);
}

#[test]
fn test2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum_hashmap(nums, target), vec![1, 2]);
}

#[test]
fn test3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum_hashmap(nums, target), vec![0, 1]);
}