struct Solution;

// You are given a tree (i.e. a connected, undirected graph that has no cycles) rooted at node 0 consisting of n nodes
// numbered from 0 to n - 1. The tree is represented by a 0-indexed array parent of size n, 
// where parent[i] is the parent of node i. Since node 0 is the root, parent[0] == -1.
// You are also given a string s of length n, where s[i] is the character assigned to node i.
// Return the length of the longest path in the tree such that no pair of adjacent nodes on the path have 
// the same character assigned to them.
 
impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        0
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::longest_path(vec![-1,0,0,1,1,2], String::from("abacbe")), 3);
}

#[test]
fn test2() {
    assert_eq!(Solution::longest_path(vec![-1,0,0,0], String::from("aabc")), 3);
}
