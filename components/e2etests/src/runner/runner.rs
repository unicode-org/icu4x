use super::*;

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

//
// tests
//

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_tests;
