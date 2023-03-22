// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_s = format!("{}", x);
        let x_s_rev = x_s.chars().rev().collect::<String>();
        x_s == x_s_rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_palindrome(
                121),
                true);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::is_palindrome(
                -121),
                false);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_palindrome(
                10),
                false);
    }
}
