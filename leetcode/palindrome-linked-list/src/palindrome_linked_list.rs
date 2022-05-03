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
        if head.unwrap().next.is_none() {
            return true;
        }
        let mut slow = &head;
        let mut fast = &head;
        while fast.unwrap().next.is_some() {
            slow = &slow.unwrap().next;
            fast = &fast.unwrap().next.unwrap().next;
        }
        let mut reversed = slow.unwrap().next.unwrap();
        let mut reversed_head = &reversed;
        while reversed.next.is_some() {
            let next = &reversed.next.unwrap();
            reversed.next = next.next.clone();
            next.next = reversed_head.clone();
            reversed_head = next;
        }
        
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
