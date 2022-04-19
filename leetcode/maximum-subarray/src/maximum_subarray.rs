struct Solution;

// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the 
// largest sum and return its sum.
// A subarray is a contiguous part of an array.

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        if nums.is_empty() {
            return 0;
        }

        let mut max_sum = nums[0];
        let mut current_sum = nums[0];
        
        for i in 1..nums.len() {
            current_sum = i32::max(current_sum + nums[i], nums[i]);
            max_sum = i32::max(max_sum, current_sum);
        }
        max_sum
    }
}


#[test]
fn test1() {
    assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
}

#[test]
fn test2() {
    assert_eq!(Solution::max_sub_array(vec![1]), 1);
}

#[test]
fn test3() {
    assert_eq!(Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
}