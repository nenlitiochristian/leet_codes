use crate::problems::{TestCases, Testable};

pub struct Solution {}

impl Testable<&str, String> for Solution {
    fn run_tests() {
        let tests = vec![TestCases::new("aaba*", "aab"), TestCases::new("aab", "aaa")];

        for (i, test) in tests.iter().enumerate() {
            println!(
                "Test Case {i}: Input {:#?}, Output {:#?}, Expected {:#?}",
                test.input,
                Solution::test(test.input),
                test.output
            );
        }
    }

    fn test(input: &str) -> String {
        Solution::clear_stars(input.to_owned())
    }
}

impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut char_count: Vec<i32> = Vec::with_capacity(26);
        for _ in 'a'..='z' {
            char_count.push(0);
        }
        
        let mut answer = String::new();
        // iterate until stars
        for char in s.chars() {
            if char != '*' {
                char_count[Self::get_char_index(char)] += 1;
                continue;
            }

            for c in 'a'..='z' {
                if char_count[Self::get_char_index(c)] > 0 {
                    char_count[Self::get_char_index(c)] -= 1;
                    break;
                }
            }
        }

        for char in s.chars().rev() {
            if char == '*' {
                continue;
            }

            if char_count[Self::get_char_index(char)] <= 0 {
                continue;
            }
            char_count[Self::get_char_index(char)] -= 1;
            answer.push(char);
        }
        answer
    }

    fn get_char_index(char: char) -> usize {
        (char as u8 - b'a') as usize
    }
}
