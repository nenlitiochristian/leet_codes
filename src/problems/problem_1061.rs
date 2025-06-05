use crate::problems::{TestCases, Testable};
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut translation_set: HashMap<char, char> = HashMap::new();
        for c in b'a'..=b'z' {
            let char = c as char;
            translation_set.insert(char, char);
        }

        // s1 and s2 are guaranteed to be of the same length
        for i in 0..s1.len() {
            let a = s1[i..].chars().next().unwrap();
            let b = s2[i..].chars().next().unwrap();
            Self::join(&mut translation_set, &a, &b);
        }

        let mut answer = String::new();
        for c in base_str.chars() {
            answer += &Self::parent(&translation_set, &c).to_string();
        }
        answer
    }

    fn join(set: &mut HashMap<char, char>, a: &char, b: &char) {
        let parent_a = Self::parent(set, a);
        let parent_b = Self::parent(set, b);

        // println!("{} {}", parent_a, parent_b);
        if parent_a < parent_b {
            set.insert(parent_b, parent_a);
        } else {
            set.insert(parent_a, parent_b);
        }
    }

    fn parent(set: &HashMap<char, char>, val: &char) -> char {
        let mut root = val;
        while set.get(root).unwrap() != root {
            root = set.get(root).unwrap();
        }
        root.to_owned()
    }
}

impl Testable<(&str, &str, &str), String> for Solution {
    fn run_tests() {
        let tests = vec![
            TestCases::new(("parker", "morris", "parser"), "makkek"),
            TestCases::new(("hello", "world", "hold"), "hdld"),
        ];

        for (i, test) in tests.iter().enumerate() {
            println!(
                "Test Case {i}: Input {:#?}, Output {:#?}, Expected {:#?}",
                test.input,
                Solution::test(test.input.clone()),
                test.output
            );
        }
    }

    fn test(input: (&str, &str, &str)) -> String {
        Solution::smallest_equivalent_string(
            input.0.to_owned(),
            input.1.to_owned(),
            input.2.to_owned(),
        )
    }
}
