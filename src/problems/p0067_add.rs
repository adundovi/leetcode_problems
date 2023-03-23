// 67. Add Binary
// https://leetcode.com/problems/add-binary/
struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();
        let mut result: Vec<u32> = Vec::new();
        let mut rem: u32 = 0;

        loop {
            let x = a_chars.next();
            let y = b_chars.next();
            
            if x == None && y == None && rem == 0 {
                break;
            }

            let x_v: u32 = x.unwrap_or('0').to_digit(10).expect("Not a digit");
            let y_v: u32 = y.unwrap_or('0').to_digit(10).expect("Not a digit");
            
            //println!("x = {} y = {} r = {}", x_v, y_v, rem);

            result.push((x_v + y_v + rem) % 2);
            rem = (x_v + y_v + rem) / 2;
        }

        result.into_iter().rev()
              .map(|d| std::char::from_digit(d, 10).unwrap())
              .collect()
        /* conversion doesn't work for a long strings
        let a_bin = isize::from_str_radix(&a, 2).expect("Not a binary number");
        let b_bin = isize::from_str_radix(&b, 2).expect("Not a binary number");
        format!("{:b}", a_bin + b_bin)
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::add_binary(
                "11".to_string(), "1".to_string()),
                "100".to_string());
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::add_binary(
                "1010".to_string(), "1011".to_string()),
                "10101".to_string());
    }
    #[test]
    fn test1() {
        assert_eq!(Solution::add_binary(
                "0".to_string(), "0".to_string()),
                "0".to_string());
    }
}
