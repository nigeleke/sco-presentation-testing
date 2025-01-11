use chrono::*;
use uuid::*;

#[allow(dead_code)]
fn split_words(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_lowercase()).collect()
}

#[allow(dead_code)]
fn create_id() -> String {
    let uuid = Uuid::new_v4();
    let time = Local::now();
    format!("timestamp {} id {}", time.format("%Y-%m-%d %H:%M:%S"), uuid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spilt_words() {
        let words = split_words("Mary had a little lamb, she also had a bear.");
        insta::assert_yaml_snapshot!(words);
    }

    #[test]
    fn test_timestamp_and_uuid() {
        let id = create_id();

        insta::with_settings!({filters => vec![
            (r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}", "<timestamp>"),
            (r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}", "<uuid>")
        ]}, {
            insta::assert_yaml_snapshot!(id, @r#""timestamp <timestamp> id <uuid>""#)
        });
    }
}
