struct Solution;

// You are given a 0-indexed integer array tasks, where tasks[i] represents the difficulty level of a task. 
// In each round, you can complete either 2 or 3 tasks of the same difficulty level.
// Return the minimum rounds required to complete all the tasks, or -1 if it is not possible to complete all the tasks.

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        0
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::minimum_rounds(vec![2,2,3,3,2,4,4,4,4,4]), 4);
}

#[test]
fn test2() {
    assert_eq!(Solution::minimum_rounds(vec![2,3,3]), -1);
}
