fn prints_and_returns_10(a: i32) -> i32 {
    println!("i got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use std::{collections::btree_map::Values, os::unix::process};

    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}