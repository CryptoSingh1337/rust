// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp_less_than_lower() {
        let result = clamp(10, 100, 200);
        assert_eq!(result, 100, "should be 100");
    }

    #[test]
    fn check_clamp_greater_than_upper() {
        let result = clamp(250, 100, 200);
        assert_eq!(result, 200, "should be 200");
    }

    #[test]
    fn check_div_by_zero() {
        let result = div(10, 0);
        assert_eq!(result, None, "should be none");
    }

    #[test]
    fn check_concat() {
        let result = concat("hello", "world");
        assert_eq!(result, "helloworld", "should be helloworld");
    }
}
