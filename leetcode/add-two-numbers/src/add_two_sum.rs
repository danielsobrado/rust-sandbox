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
    //  l1 = 2->4->3
    let n = ListNode::new(3);
    let n = ListNode { val: 4, next: Some(Box::new(n)) };
    let n = ListNode { val: 2, next: Some(Box::new(n)) };    
    let l1 = Some(Box::new(n));

    let n = ListNode::new(4);
    let n = ListNode { val: 6, next: Some(Box::new(n)) };
    let n = ListNode { val: 5, next: Some(Box::new(n)) };
    let l2 = Some(Box::new(n));

    let n = ListNode::new(8);
    let n = ListNode { val: 0, next: Some(Box::new(n)) };
    let n = ListNode { val: 7, next: Some(Box::new(n)) };
    let sol = Some(Box::new(n));

    assert_eq!(Solution::add_two_numbers(l1, l2), sol);
}

#[test]
fn test2() {
  let n = ListNode::new(0);
  let l1 = Some(Box::new(n));

  let n = ListNode::new(0);
  let l2 = Some(Box::new(n));

  let n = ListNode::new(0);
  let sol = Some(Box::new(n));

  assert_eq!(Solution::add_two_numbers(l1, l2), sol);
}

#[test]
fn test3() {
  let n = ListNode::new(9);
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };    
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };    
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };    
  let l1 = Some(Box::new(n));

  let n = ListNode::new(9);
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let l2 = Some(Box::new(n));

  let n = ListNode::new(1);
  let n = ListNode { val: 0, next: Some(Box::new(n)) };
  let n = ListNode { val: 0, next: Some(Box::new(n)) };
  let n = ListNode { val: 0, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 9, next: Some(Box::new(n)) };
  let n = ListNode { val: 8, next: Some(Box::new(n)) };
  let sol = Some(Box::new(n));

  assert_eq!(Solution::add_two_numbers(l1, l2), sol);
}