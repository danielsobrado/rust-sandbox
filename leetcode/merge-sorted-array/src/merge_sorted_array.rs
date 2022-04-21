struct Solution;

// Problem URL: https://leetcode.com/problems/merge-sorted-array/
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, 
// but instead be stored inside the array nums1. 
// To accommodate this, nums1 has a length of m + n, 
// where the first m elements denote the elements that should be merged, 
// and the last n elements are set to 0 and should be ignored. 
// nums2 has a length of n.
// Solution: O(n) time, O(1) space

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

        if n == 0 {
            return;
        }

        let mut i: i32 =  m - 1;
        let mut j: i32 = n - 1;

        while i >= 0 && j >= 0 {
            // Compare from the end of nums1 before the 0s with the end of nums2
            if nums1[i as usize] > nums2[j as usize] {
                // Start populating from the end on nums1
                nums1[(i+j+1) as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[(i+j+1) as usize] = nums2[j as usize];
                j -= 1;
            }
        }
            
        // Populate the rest of nums1 with nums2
        while j >= 0 {
            nums1[(i+j+1) as usize] = nums2[j as usize];
            j -= 1;
        }

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