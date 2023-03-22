// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let l = s.len();

        let s = s.replace("{}", "");
        let s = s.replace("[]", "");
        let s = s.replace("()", "");

        if l == s.len() {
            return false;
        }

        return Self::is_valid(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_valid(
                "()".to_string()),
                true);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::is_valid(
                "()[]{}".to_string()),
                true);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_valid(
                "(]".to_string()),
                false);
    }
}
