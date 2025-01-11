#[allow(dead_code)]
fn split_words(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_lowercase()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spilt_words() {
        let words = split_words("Mary had a little lamb, she also had a bear.");
        insta::assert_yaml_snapshot!(words);
    }
}
