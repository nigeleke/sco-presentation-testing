pub fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::*;

    // #[test]
    // fn add_function_works() {
    //     assert_eq!(add(3, 5), 8);
    // }

    proptest! {
        #[test]
        fn add_function_works(a in i32::MIN..i32::MAX, b in i32::MIN..i32::MAX) {
            let result = add(a, b);
            assert_eq!(result, a.saturating_add(b));
        }
    }
}
