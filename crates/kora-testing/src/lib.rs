//! Built-in Testing Framework, Assertions, and Mocking Primitives for Kora.

pub struct TestRunner;

impl TestRunner {
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(left: T, right: T, msg: &str) {
        if left != right {
            panic!("Assertion failed ({msg}): left={left:?}, right={right:?}");
        }
    }

    pub fn assert_true(cond: bool, msg: &str) {
        if !cond {
            panic!("Assertion failed ({msg}): expected true, got false");
        }
    }
}

