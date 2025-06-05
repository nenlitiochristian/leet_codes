pub mod problem_1061;
pub mod problem_2698;
pub mod problem_3403;

pub struct TestCases<A, B> {
    input: A,
    output: B,
}

impl<A, B> TestCases<A, B> {
    pub fn new(input: A, output: B) -> TestCases<A, B> {
        TestCases { input, output }
    }
}

pub trait Testable<A, B> {
    fn run_tests();
    fn test(input: A) -> B;
}
