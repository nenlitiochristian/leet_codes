use crate::problems::{TestCases, Testable};

pub struct Solution {}

impl Testable<i32, i32> for Solution {
    fn run_tests() {
        let tests = vec![
            TestCases::new(10, 182),
            TestCases::new(37, 1478),
            TestCases::new(45, 3503),
        ];

        for (i, test) in tests.iter().enumerate() {
            println!(
                "Test Case {i}: Input {}, Output {}, Expected {}",
                test.input,
                Solution::test(test.input),
                test.output
            );
        }
    }

    fn test(input: i32) -> i32 {
        Solution::punishment_number(input)
    }
}

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut total: i32 = 0;
        for i in 1..=n {
            if Self::is_punishment_number(i) {
                total += i * i;
            }
        }
        total
    }

    fn is_punishment_number(n: i32) -> bool {
        let mut squared = n * n;
        let mut single_digits: Vec<i32> = Vec::new();

        while squared > 0 {
            single_digits.push(squared % 10);
            squared /= 10;
        }

        single_digits.reverse();
        // println!("{:?}", single_digits);
        Self::recursive_check(0, 0, &single_digits, n)
    }

    fn recursive_check(i: usize, acc: i32, digits: &Vec<i32>, target: i32) -> bool {
        if i == digits.len() {
            return acc == target;
        }

        if acc > target {
            return false;
        }

        // combine
        if Self::recursive_check(i + 1, acc * 10 + digits[i], digits, target) {
            return true;
        }

        // cut
        if acc != 0 {
            if Self::recursive_check(i, 0, digits, target - acc) {
                return true;
            }
        }

        false
    }
}
