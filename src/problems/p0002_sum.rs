// 1. Two Sum
// https://leetcode.com/problems/two-sum/
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, val_i) in nums.iter().enumerate() {
            for (j, val_j) in nums[i+1..].iter().enumerate() {
                if val_i + val_j == target {
                    return vec![i as i32, (j+i+1) as i32];
                }
            }
        }
        return vec![0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::two_sum(
                vec![2,7,11,15],
                9),
                vec![0,1]);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::two_sum(
                vec![3,2,4],
                6),
                vec![1,2]);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::two_sum(
                vec![3,3],
                6),
                vec![0,1]);
    }
}
