// 70. Climbing Stairs
// https://leetcode.com/problems/climbing-stairs/
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // to reach the 1st -> 1
        // to reach the 2nd -> 2 (2x1, 1x2)
        if n <= 2 {
            return n;
        }
        let mut ways_to_stair: Vec<i32> = vec![0, 1, 2];
        ways_to_stair.resize_with((n+1) as usize, || {0});
        // to reach the 3rd -> either from (-2) or (-1)
        for i in 3..=n {
            ways_to_stair[i as usize] = ways_to_stair[(i-1) as usize] + ways_to_stair[(i-2) as usize];
        }
        ways_to_stair[n as usize]
    }
}
/* impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let single = 1;
        let double = 2;
        let mut total: i64 = 0;

        fn factorial(k: u128) -> i64 {
            let mut tot = 1;
            if k == 0 {
                return 1;
            }
            for i in 1..(k+1) {
                tot *= i;
            }
            tot
        }

        while single*i <= n {
            if (n - single * i) % double == 0 {
                j = (n - single * i) / double;
                total += factorial((i+j) as i64)/(factorial(i as i64)*factorial(j as i64));
                //println!("1: {i}, 2: {j} => {n} ({total})");
            }
            i += 1;
        }

        total.try_into().unwrap()
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::climb_stairs(
                2),
                2);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::climb_stairs(
                3),
                3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(
                35),
                14930352);
    }
}
