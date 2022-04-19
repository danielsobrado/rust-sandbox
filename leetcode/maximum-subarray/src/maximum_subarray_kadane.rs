struct Solution;

// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the 
// largest sum and return its sum.
// A subarray is a contiguous part of an array.

// Kadane’s Algorithm can be viewed both as a greedy and Dynamic Programming problem. 
// As we can see that we are keeping a running sum of integers and when it becomes less than 0, we reset it to 0 (Greedy Part). 
// This is because continuing with a negative sum is way more worse than restarting with a new range. 
// Now it can also be viewed as a Dynamic Programming problem, at each stage we have 2 choices: 
// Either take the current element and continue with previous sum OR restart a new range. 
// These both choices are being taken care of in the implementation. 

// Time Complexity: O(n) Auxiliary Space: O(1)

impl Solution {
    pub fn maximum_subarray_kadane(nums: Vec<i32>) -> i32 {

        if nums.is_empty() {
            return 0;
        }

        let mut max_sum = nums[0];
        let mut current_sum = nums[0];

        

    }
}


#[test]
fn test1() {
    assert_eq!(Solution::maximum_subarray_kadane(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
}

#[test]
fn test2() {
    assert_eq!(Solution::maximum_subarray_kadane(vec![1]), 1);
}

#[test]
fn test3() {
    assert_eq!(Solution::maximum_subarray_kadane(vec![5,4,-1,7,8]), 23);
}