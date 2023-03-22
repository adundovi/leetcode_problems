// 13. Roman to Integer
// https://leetcode.com/problems/roman-to-integer/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let roman2int = HashMap::from(
                [('M', 1000), ('D', 500), ('C', 100), ('L', 50), ('X', 10), ('V', 5), ('I', 1)]
            );
       
        let chars: Vec<char> = s.chars().collect();
        let mut prev = 0;

        for c in chars {
            //println!("{c}, {result}");
            let curr = roman2int[&c];
            result += curr;
            if prev < curr {
                result -= 2*prev;
            }
            prev = curr;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::roman_to_int(
                "III".to_string()),
                3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::roman_to_int(
                "LVIII".to_string()),
                58);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::roman_to_int(
                "MCMXCIV".to_string()),
                1994);
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::roman_to_int(
                "DCXXI".to_string()),
                621);
    }
}
