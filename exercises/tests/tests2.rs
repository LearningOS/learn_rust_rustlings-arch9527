// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(add_two(6), 8);
    }
}
