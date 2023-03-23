// 69. Sqrt(x)
// https://leetcode.com/problems/sqrtx/
struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut p = 1;

        while (p * p < x) && (p < 46340) {
            p += 1;
            println!("{p}");
        }
        if p * p > x {
            p - 1
        } else {
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::my_sqrt(
                4),
                2);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::my_sqrt(
                8),
                2);
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::my_sqrt(
                49),
                7);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::my_sqrt(
                2147483647),
                46340);
    }
}
