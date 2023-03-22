// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/
struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: String = "".to_string();

        let min: usize = strs.iter()
                             .min_by(|s1, s2| s1.chars().count().cmp(&s2.chars().count()))
                             .unwrap_or(&String::new()).len();

        'main: for i in 0..min {
            let curr = strs[0].chars().nth(i).unwrap();
            for s in strs.iter() {
                if curr != s.chars().nth(i).unwrap() {
                    break 'main;
                }
            }
            prefix.push(curr);
        }

        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]
            ),
            "fl".to_string());
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec!["dog".to_string(),"racecar".to_string(),"car".to_string()],
            ),
            "".to_string());
    }
}
