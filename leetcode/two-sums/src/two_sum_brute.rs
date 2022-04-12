
struct Solution;

// Problem:
// Given an array of integers nums and an integer target, 
// return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.
// You can return the answer in any order.
// Solution:
// 2 nested loops, using brute force
// Complexity: O(n^2) time, O(n) space
impl Solution {
    fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (pos, e) in nums.iter().enumerate() {
            for (mut pos2, e2) in nums.iter().skip(pos+1).enumerate() {
                pos2 = pos2 + pos + 1;
                println!("pos: {} e: {} - pos2: {} e2: {}", pos, e, pos2, e2);
                if (e + e2) == target {
                    println!("Return: {}, {}", pos as i32, pos2 as i32);
                    return vec![pos as i32, pos2 as i32];
                }
            }
        }
        vec![]
    }
}

#[test]
fn test1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum_brute(nums, target), vec![0, 1]);
}

#[test]
fn test2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum_brute(nums, target), vec![1, 2]);
}

#[test]
fn test3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum_brute(nums, target), vec![0, 1]);
}