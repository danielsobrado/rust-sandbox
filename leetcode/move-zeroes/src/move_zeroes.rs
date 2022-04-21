struct Solution;

// Problem URL: https://leetcode.com/problems/move-zeroes/
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order 
// of the non-zero elements.
// Note that you must do this in-place without making a copy of the array.


impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        
        
    }
}

#[test]
fn test1() {
    let mut nums = vec![0,1,0,3,12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [1,3,12,0,0]);
}

#[test]
fn test2() {
    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [0]);
}
