# sco-presentation-testing

Testing presentation to South Coast Software Developers

## Test Driven Development

* Write test first
  ```rust add.rs
  #[cfg(test)]
  mod test {
      use super::*;
      use pretty_assertions::assert_ne;

      #[test]
      fn add_function_works() {
          assert_ne!(add(2, 2), 5);
      }
  }
  ```

* Test fails: compilation error.
  ```
  8 |         assert_ne!(add(2, 2), 5);
    |                    ^^^ not found in this scope
  ```

* Implement `add`
  ```rust add.rs
  fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

* Test passes:
  ```
  running 1 test
  test test::add_function_works ... ok
  ```

* but, compiler warning:
  ```
  Compiling sco-presentation-testing v0.1.0 (/home/nigel/Documents/sco-presentation-testing)
  warning: function `add` is never used
  --> src/add.rs:1:4
  |
  1 | fn add(a: i32, b: i32) -> i32 {
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default
  ```

* use code; declare usable publicly -
  ```rust main.rs
  use add::*;

  pub fn main() {
      let a = 3;
      let b = 5;
      let result = add(a, b);
      println!("a: {a} + b: {b} => result: {result}");
  }
  ```
  ```rust add.rs
  pub fn add...
  ```

## Code Coverage

* `cargo tarpaulin`
  ```
  2025-01-11T03:11:55.199261Z  INFO cargo_tarpaulin::report: Coverage Results:
  || Uncovered Lines:
  || src/main.rs: 5-9
  || Tested/Total Lines:
  || src/add.rs: 2/2 +0.00%
  || src/main.rs: 0/5 +0.00%
  ||
  28.57% coverage, 2/7 lines covered, +0.00% change in coverage
  ```

* Coverage fails as main not adequately tested.
  ```rust main.rs
  #[cfg(test)]
  mod test {
      use super::*;
      use pretty_assertions::assert_eq;

      #[test]
      fn main_can_be_invoked() {
          assert_eq!(main(), ());
      }
  }
  ```
  ```
  2025-01-11T03:15:28.441144Z  INFO cargo_tarpaulin::report: Coverage Results:
  || Uncovered Lines:
  || Tested/Total Lines:
  || src/add.rs: 2/2 +0.00%
  || src/main.rs: 5/5 +100.00%
  ||
  100.00% coverage, 7/7 lines covered, +71.43% change in coverage
  ```

* 100% fully tested and utilised code; what could go wrong?

## Mutants Testing

* Add `#[cfg_attr(test, mutants::skip)]` annotation to `main` while we focus on `add`

* `cargo mutants`
  ```
  Found 5 mutants to test
  ok       Unmutated baseline in 1.8s build + 0.1s test
  INFO Auto-set test timeout to 20s
  MISSED   src/add.rs:2:5: replace add -> i32 with 0 in 0.4s build + 0.1s test
  MISSED   src/add.rs:2:5: replace add -> i32 with -1 in 0.4s build + 0.1s test
  MISSED   src/add.rs:2:5: replace add -> i32 with 1 in 0.4s build + 0.1s test
  MISSED   src/add.rs:2:7: replace + with * in add in 0.3s build + 0.1s test
  MISSED   src/add.rs:2:7: replace + with - in add in 0.3s build + 0.1s test
  5 mutants tested in 4s: 5 missed
  ```

* Change test:
  ```rust
  assert_eq!(add(2, 2), 4);
  ```

* `cargo mutants` still fails.

* Change test:
  ```rust
  assert_eq!(add(3, 5), 8);
  ```

* `cargo mutants`
  ```
  Found 5 mutants to test
  ok       Unmutated baseline in 1.0s build + 0.1s test
  INFO Auto-set test timeout to 20s
  5 mutants tested in 3s: 5 caught
  ```

* but is this good enough?

## Property-based Testing

* Improve add test
  ```rust
  proptest! {
      #[test]
      fn add_function_works(a in 0..100, b in 0..100) {
          let result = add(a, b);
          assert_eq!(result, a + b);
      }
  }
  ```

* Test passes
* but is this good enough?
  ```rust
  fn add_function_works(a in i32::MIN..i32::MAX, b in i32::MIN..i32::MAX) {
  ```
  ```
  thread 'add::test::add_function_works' panicked at src/add.rs:11:5:
  Test failed: attempt to add with overflow.
  ```

* Amend test to state how overflow should be handled:
  ```rust
  assert_eq!(result, a.saturating_add(b));
  ```
* Test fails; code needs amending too:
  ```rust
  pub fn add(a: i32, b: i32) -> i32 {
      a.saturating_add(b)
  }
  ```

## Snapshot Testing

* `cargo install cargo-insta'
* Start with test:
  ```rust
  #[cfg(test)]
  mod test {
      use super::*;

      #[test]
      fn test_spilt_words() {
          let words = split_words("Mary had a little lamb, she also had a bear");
          insta::assert_yaml_snapshot!(words);
      }
  }
  ```

* and function:
  ```rust
  fn split_words(s: &str) -> Vec<String> {
      s.split_whitespace().map(|s| s.into()).collect()
  }
  ```

* `cargo test` fails:
  ```
  stored new snapshot /home/nigel/Documents/sco-presentation-testing/src/snapshots/sco_presentation_testing__ui__test__spilt_words.snap.new
  test ui::test::test_spilt_words ... FAILED
  + more...
  ```

* `cargo insta review` and accept.

* `cargo test` passes

* amend function:
  ```rust
  #[allow(dead_code)]
  fn split_words(s: &str) -> Vec<String> {
      s.split_whitespace().map(|s| s.to_lowercase()).collect()
  }
  ```

* `cargo test` fails

* `cargo insta review` shows differences

* new test (note the @""):
  ```rust
  #[test]
  fn test_timestamp_and_uuid() {
      let id = create_id();
      insta::assert_yaml_snapshot!(id, @"");
  }
  ```

* and the code
  ```rust
  #[allow(dead_code)]
  fn create_id() -> String {
      let uuid = uuid::Uuid::new_v4();
      let time = chrono::Local::now();
      format!("timestamp {} id {}", time.format("%Y-%m-%d %H:%M:%S"), uuid)
  }
  ```

* `insta` updated the expected result in code, not in a snapshot file.

* `cargo test` will always fail and `cargo insta review` will always require reviewing.

* add redactions:
  ```rust
  insta::with_settings!({filters => vec![
      (r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}", "<timestamp>"),
      (r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}", "<uuid>")
  ]}, {
      insta::assert_yaml_snapshot!(id, @"")
  });
  ```

* `cargo test` passes after `cargo insta review`
