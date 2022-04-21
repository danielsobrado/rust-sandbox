struct Solution;

// Problem URL: https://leetcode.com/problems/merge-sorted-array/
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, 
// but instead be stored inside the array nums1. 
// To accommodate this, nums1 has a length of m + n, 
// where the first m elements denote the elements that should be merged, 
// and the last n elements are set to 0 and should be ignored. 
// nums2 has a length of n.
// Solution: O(n) time, O(n) space

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

        if n == 0 {
            return;
        }

        let mut i: i32 = 0;
        let mut j: i32 = 0;

        let mut result = vec![0; m as usize + n as usize];

        // Populate the result vector with the lowest of nums1 or nums2
        while i + j < n+m {
            if i < m && j < n && nums1[i as usize] < nums2[j as usize] {
                result[i as usize + j as usize] = nums1[i as usize];
                i += 1;
            } else if j < n {
                result[i as usize + j as usize] = nums2[j as usize];
                j += 1;
            } else if i < m {
                result[i as usize + j as usize] = nums1[i as usize];
                i += 1;
            }
        }

        // Populate the rest of result with nums2
        while i < m-1 {
            result[i as usize + j as usize] = nums2[i as usize];
            i += 1;
        }
        
        *nums1 = result.to_vec();

    }
}

#[test]
fn test1() {
    let mut nums1 = vec![1,2,3,0,0,0];
    let mut nums2 = vec![2,5,6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, [1,2,2,3,5,6]);
}

#[test]
fn test2() {
    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    Solution::merge(&mut nums1, 1, &mut nums2, 0);
    assert_eq!(nums1, [1]);
}

#[test]
fn test3() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    Solution::merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, [1]);
}

#[test]
fn test4() {
    let mut nums1 = vec![2,0];
    let mut nums2 = vec![1];
    Solution::merge(&mut nums1, 1, &mut nums2, 1);
    assert_eq!(nums1, [1, 2]);
}

#[test]
fn test5() {
    let mut nums1 = vec![4,5,6,0,0,0];
    let mut nums2 = vec![1,2,3];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, [1,2,3,4,5,6]);
}