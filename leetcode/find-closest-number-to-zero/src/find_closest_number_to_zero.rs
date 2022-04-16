struct Solution;
// Given an integer array nums of size n, return the number with the value closest to 0 in nums. 
// If there are multiple answers, return the number with the largest value.
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut closest = 1000000;
        let mut closest_index = 0;
        for i in 0..nums.len() {
            if (nums[i] - 0).abs() < closest {
                closest = (nums[i] - 0).abs();
                closest_index = i;
            }
        }
        nums[closest_index]
    }
}

#[test]
fn test1() {
    assert_eq!(Solution::find_closest_number(vec![-4,-2,1,4,8]), 1);
}

fn test2() {
    assert_eq!(Solution::find_closest_number(vec![2,-1,1]), 1);
}