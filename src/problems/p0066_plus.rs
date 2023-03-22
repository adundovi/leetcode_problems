// 66. Plus One
// https://leetcode.com/problems/plus-one/
struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(digits.len() + 1);

        let mut reminder = 1;

        for d in digits.iter().rev() {
            let digit = (d + reminder) % 10;
            result.push(digit);
            reminder = (d + reminder) / 10;
        }
        if reminder == 1 {
            result.push(reminder);
        }

        result.reverse();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::plus_one(
                vec![1,2,3]),
                vec![1,2,4]);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::plus_one(
                vec![4,3,2,1]),
                vec![4,3,2,2]);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::plus_one(
                vec![9]),
                vec![1,0]);
    }
}
