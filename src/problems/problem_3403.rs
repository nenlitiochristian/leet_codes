use crate::problems::{TestCases, Testable};

pub struct Solution {}

impl Testable<(String, i32), String> for Solution {
    fn run_tests() {
        let tests = vec![
            TestCases::new(("dbca".to_owned(), 2), "dbc"),
            TestCases::new(("gggg".to_owned(), 4), "g"),
            TestCases::new(
                (
                    "cgwzebexlahnfqsetbeaybmfdzywpvehjybpwiotnciddjonfi".to_owned(),
                    21,
                ),
                "zywpvehjybpwiotnciddjonfi",
            ),
            TestCases::new(
                (
                    "acvrtkvzldieqsbzouuqjgfyzqetgocixccgyybiwzlnzceshrjtorszjmxggixpbmvkyevpuwzstmeqhdgzrfmqmfwkiyklshnpdpiutvxctgneuiiyrmlbfhxknnbsbxsbvbgzemmuaqmozohaihhpcdlptxirnnrsysromlktihxkjsxcrlswirdummuovkpaelit".to_owned(),
                    75,
                ),
                "zstmeqhdgzrfmqmfwkiyklshnpdpiutvxctgneuiiyrmlbfhxknnbsbxsbvbgzemmuaqmozohaihhpcdlptxirnnrsysromlktihxkjsxcrlswirdummuovkpaelit",
            ),
            TestCases::new(
                ("gh".to_owned(), 1),
                "gh"
            )
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

    fn test(input: (String, i32)) -> String {
        Solution::answer_string(input.0, input.1)
    }
}

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let num_friends: usize = num_friends.try_into().unwrap();
        let mut largest = String::new();

        for i in 0..word.len() {
            let mut curr = &word[i..];
            let friends_left = num_friends - usize::min(num_friends, i + 1);
            // need to leave at least one character to split to each friend
            // println!("{}1", curr);
            if friends_left > 0 {
                let max_len = curr.len() - friends_left;
                curr = &curr[..max_len];
            }

            // println!("{} {} {}", curr, i, friends_left);
            if Self::lexicograph_smaller(&largest, curr) {
                largest = curr.to_owned();
            }
        }
        largest
    }

    fn lexicograph_smaller(a: &str, b: &str) -> bool {
        return a < b;
    }
}
