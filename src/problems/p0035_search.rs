// 35. Search Insert Position
// https://leetcode.com/problems/search-insert-position/
struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let p = nums.iter().position(|&i| i >= target);
        match p {
            Some(n) => n as i32,
            None => nums.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::search_insert(
                vec![1,3,5,6], 5),
                2);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::search_insert(
                vec![1,3,5,6], 2),
                1);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::search_insert(
                vec![1,3,5,6], 7),
                4);
    }
}
