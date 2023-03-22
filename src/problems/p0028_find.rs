// 28. Find the Index of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(n) => n as i32,
            None => -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::str_str(
            "sadbutsad".to_string(),
            "sad".to_string()), 0);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::str_str(
            "leetcode".to_string(),
            "leeto".to_string()), -1);
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::str_str(
            "thisisacode".to_string(),
            "code".to_string()), 7);
    }
}
