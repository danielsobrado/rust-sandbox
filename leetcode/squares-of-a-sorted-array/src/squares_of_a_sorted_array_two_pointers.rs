struct Solution;

// Problem URL: https://leetcode.com/problems/squares-of-a-sorted-array/
// Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.
// Solution: Use two pointers.
// Time Complexity: O(n) where n is the length of nums.
// Space Complexity: O(1) since we are not using any extra space.
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {

        if nums.len() == 0 {
            return nums;
        } else if nums.len() == 1 {
            return vec![nums[0] * nums[0]];
        }

        let mut i = 0 as usize;
        let mut j = nums.len() - 1;
        let mut counter = nums.len();

        let mut result = vec![0; nums.len()];

        while i <= j && counter > 0 {
            counter -= 1;
            if nums[i].abs() > nums[j].abs() {
                result[counter] = nums[i].abs() * nums[i].abs();
                i += 1;
            } else {
                result[counter] = nums[j].abs() * nums[j].abs();
                j -= 1;
            }
        }

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
