struct Solution;

// You are given a 2D integer array grid of size m x n, where each cell contains a positive integer.
// A cornered path is defined as a set of adjacent cells with at most one turn. 
// More specifically, the path should exclusively move either horizontally or vertically up to the turn (if there is one), 
// without returning to a previously visited cell. 
// After the turn, the path will then move exclusively in the alternate direction: 
// move vertically if it moved horizontally, and vice versa, also without returning to a previously visited cell.
// The product of a path is defined as the product of all the values in the path.
// Return the maximum number of trailing zeros in the product of a cornered path found in grid.
// Note:
// Horizontal movement means moving in either the left or right direction.
// Vertical movement means moving in either the up or down direction.
impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::max_trailing_zeros(vec![
        vec![23,17,15,3,20],
        vec![8,1,20,27,11],
        vec![9,4,6,2,21],
        vec![40,9,1,10,6],
        vec![22,7,4,5,3]]), 3);
}

#[test]
fn test2() {
    assert_eq!(Solution::max_trailing_zeros(vec![
        vec![4,3,2],
        vec![7,6,1],
        vec![8,8,8]]), 0);
}
