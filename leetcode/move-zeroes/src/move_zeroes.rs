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

        let mut i = nums.len() - 1;
        while i >= 0 as usize {
            if nums[i] == 0 && i != nums.len()-1 {
                let mut j = i;
                while j < nums.len()-1 {
                    if nums[j+1] != 0  {
                        nums.swap(j, j+1);
                    } 
                    j += 1;
                }
            }
            if i != 0 as usize {
                i -= 1;
            } else {
                return;
            }
        }
        return;        

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
