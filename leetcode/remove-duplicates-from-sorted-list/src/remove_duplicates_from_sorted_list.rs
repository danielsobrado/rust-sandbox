struct Solution;

// Problem URL: https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. 
// Return the linked list sorted as well.

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
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      Some(Box::new(ListNode::new(0)))
  }
}

#[test]
fn test1() {
  let n = ListNode::new(1);
  let n = ListNode { val: 1, next: Some(Box::new(n)) };
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let list1 = Some(Box::new(n));

  let n = ListNode::new(1);
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let result = Some(Box::new(n));

  assert_eq!(Solution::delete_duplicates(list1), result);
}

#[test]
fn test2() {

  let n = ListNode::new(1);
  let n = ListNode { val: 1, next: Some(Box::new(n)) };
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let n = ListNode { val: 3, next: Some(Box::new(n)) };
  let n = ListNode { val: 3, next: Some(Box::new(n)) };
  let list1 = Some(Box::new(n));

  let n = ListNode::new(1);
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let n = ListNode { val: 3, next: Some(Box::new(n)) };
  let result = Some(Box::new(n));

  assert_eq!(Solution::delete_duplicates(list1), result);
}
