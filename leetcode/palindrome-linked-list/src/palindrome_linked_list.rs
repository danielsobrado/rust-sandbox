struct Solution;

// Problem URL: https://leetcode.com/problems/palindrome-linked-list/
// Given the head of a singly linked list, return true if it is a palindrome.

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        
    }
}

#[test]
fn test1() {
    let head = Some(Box::new(ListNode::new(1)));
    let head = Some(Box::new(ListNode::new(2)));
    let head = Some(Box::new(ListNode::new(2)));
    let head = Some(Box::new(ListNode::new(1)));

    assert_eq!(Solution::is_palindrome(head), true);
}

#[test]
fn test2() {

    let head = Some(Box::new(ListNode::new(1)));
    let head = Some(Box::new(ListNode::new(2)));

    assert_eq!(Solution::is_palindrome(head), false);
}