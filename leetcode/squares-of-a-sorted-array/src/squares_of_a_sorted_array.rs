struct Solution;

// Problem URL: https://leetcode.com/problems/squares-of-a-sorted-array/
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

impl Solution {
    pub fn squares_of_a_sorted_array(nums: &mut Vec<i32>) {

        if nums.len() <= 1 {
            return;
        }
    }

}

#[test]
fn test1() {
    let mut nums = vec![-4,-1,0,3,10];
    Solution::squares_of_a_sorted_array(&mut nums);
    assert_eq!(nums, [0,1,9,16,100]);
}

#[test]
fn test2() {
    let mut nums = vec![-7,-3,2,3,11];
    Solution::squares_of_a_sorted_array(&mut nums);
    assert_eq!(nums, [4,9,9,49,121]);
}
