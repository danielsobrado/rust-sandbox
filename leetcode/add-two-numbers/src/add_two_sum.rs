// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
    }
}

#[test]
fn test1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::add_two_numbers(nums, target), vec![0, 1]);
}

#[test]
fn test2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::add_two_numbers(nums, target), vec![1, 2]);
}

#[test]
fn test3() {
    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::add_two_numbers(nums, target), vec![0, 1]);
}