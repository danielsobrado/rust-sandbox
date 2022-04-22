struct Solution;

// Problem URL: https://leetcode.com/problems/rotate-array/
// Given an array, rotate the array to the right by k steps, where k is non-negative.
// Store last k numbers
impl Solution {

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {

        let k = k as usize % nums.len();
        let last_k = nums[(nums.len() - k)..].to_vec();

        for i in (k..nums.len()).rev() {
            nums.swap(i, i - k);
        }
        
        nums.splice(..k, last_k);
    }

}

#[test]
fn test1() {
    let mut nums = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, [5,6,7,1,2,3,4]);
}

#[test]
fn test2() {
    let mut nums = vec![-1,-100,3,99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, [3,99,-1,-100]);
}

#[test]
fn test3() {
    let mut nums = vec![1,2];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, [2,1]);
}