mod add;

use add::*;

#[cfg_attr(test, mutants::skip)]
pub fn main() {
    let a = 3;
    let b = 5;
    let result = add(a, b);
    println!("a: {a} + b: {b} => result: {result}");
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn main_can_invoked() {
        assert_eq!(main(), ());
    }
}
