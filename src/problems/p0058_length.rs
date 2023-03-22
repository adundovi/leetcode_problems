// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/
struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().expect("There will be at least one word in s").len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::length_of_last_word(
                "Hello World".to_string()),
                5);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::length_of_last_word(
                "   fly me   to   the moon  ".to_string()),
                4);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::length_of_last_word(
                "luffy is still joyboy".to_string()),
                6);
    }
}
