struct Solution;

// Problem URL: https://leetcode.com/problems/merge-two-sorted-lists/
// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
// Return the head of the merged linked list.

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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

#[test]
fn test1() {
  let n = ListNode::new(1);
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let n = ListNode { val: 4, next: Some(Box::new(n)) };
  let list1 = Some(Box::new(n));

  let n = ListNode::new(1);
  let n = ListNode { val: 3, next: Some(Box::new(n)) };
  let n = ListNode { val: 4, next: Some(Box::new(n)) };
  let list2 = Some(Box::new(n));

  let n = ListNode::new(1);
  let n = ListNode { val: 1, next: Some(Box::new(n)) };
  let n = ListNode { val: 2, next: Some(Box::new(n)) };
  let n = ListNode { val: 3, next: Some(Box::new(n)) };
  let n = ListNode { val: 4, next: Some(Box::new(n)) };
  let n = ListNode { val: 4, next: Some(Box::new(n)) };
  let result = Some(Box::new(n));

    assert_eq!(Solution::merge_two_lists(list1, list2), result);
}

#[test]
fn test2() {
    assert_eq!(Solution::merge_two_lists(None, None), None);
}

#[test]
fn test3() {
    let list2 = Some(Box::new(ListNode::new(2)));
    let result = Some(Box::new(ListNode::new(0)));

    assert_eq!(Solution::merge_two_lists(None, list2), result);
}

