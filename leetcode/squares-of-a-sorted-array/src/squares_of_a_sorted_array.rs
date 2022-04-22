struct Solution;

// Problem URL: https://leetcode.com/problems/squares-of-a-sorted-array/
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {

        if nums.len() == 0 {
            return nums;
        }

        let mut result = nums.iter().map(|x| x * x).collect::<Vec<i32>>();

        result.sort_unstable();

        result
    }

}

#[test]
fn test1() {
    let nums = vec![-4,-1,0,3,10];
    assert_eq!(Solution::sorted_squares(nums), [0,1,9,16,100]);
}

#[test]
fn test2() {
    let nums = vec![-7,-3,2,3,11];
    assert_eq!(Solution::sorted_squares(nums), [4,9,9,49,121]);
}

#[test]
fn test3() {
    let nums = vec![-1];
    assert_eq!(Solution::sorted_squares(nums), [1]);
}
