use std::fmt::Debug;
use std::fmt::Write;

/// Executes a series of test-cases, collecting error information.
///
/// # Usage
/// - An iterator of input and expected output data is required.
/// - By default compares the result and expected result for equality,
///   a custom assertion function may be provided as sixth parameter.
/// - While debugging, panics on assertion failure, otherwise collects all failed data in a `Vec`
///
/// # Examples
/// **Basic usage:**
/// ```rust
/// #[test]
/// fn test_parse_fragment_any() {
///     report_fails(collect_fails!(
///         // input type
///         &str,
///         // output type
///         IResult<&str, Fragment, ()>,
///         // test cases in format (input, expected)  
///         vec![
///             ("/", Ok(("", Fragment::Separator))),
///             ("///", Ok(("", Fragment::Separator))),
///             ("path/to/file", Ok(("/to/file", Fragment::Plain("path"))))
///         ].into_iter(),
///         // test function
///         parse_fragment
///     ));
/// }
/// ```
///
/// **Custom assertion:**
/// ```rust
/// fn test_in_range() {
///     report_fails(collect_fails!(
///         usize,
///         std::ops::Range<usize>,
///         usize,
///         vec![(2, 1..4), (3, 4..6), (0, 1..3)].into_iter(),
///         |input| input + 2,
///         |output: &usize, expected: &Range<usize>| expected.contains(output)
///     ));
/// }
/// ```
#[macro_export]
macro_rules! collect_fails {
    ($input:ty, $expected:ty, $result:ty, $cases:expr, $test:expr, $assert:expr) => {{
        let mut case_id = 0usize;
        $cases
            .filter_map(|(input, expected)| {
                case_id += 1;
                let result: $result = $test(&input);
                let assert = $assert(&result, &expected);
                debug_assert!(
                    assert,
                    "test case {}: assertion failed for input `{:#?}`\n\texpected `{:#?}`\n\tresult `{:#?}`\n",
                    case_id, &input, &expected, &result
                );
                if assert {
                    None
                } else {
                    Some((input, expected, result, case_id))
                }
            })
            .collect::<Vec<($input, $expected, $result, usize)>>()
    }};
    ($input:ty, $result:ty, $cases:expr, $test:expr) => {
        collect_fails!($input, $result, $result, $cases, $test, |e, r| e == r)
    };
}

/// Constructs a pretty print report of all failed assertions.
/// - **This method does not check for plausible input!**
/// - Panics if `fails.is_empty() == false`.
///
/// # Usage
/// Usually used in combination with `collect_fails`
///
/// **Basic usage:**
/// ```rust
///
/// report_fails(vec![
///     ("input string", "expected string", "", 1),
///     ("hello world!", "hello papa!", "hello mom!", 2),
/// ])
///
/// // One or more assertions failed:
/// // test case 1: assertion failed for input `"input string"`
/// //         expected `"expected string"`
/// //         result `""`

/// // test case 2: assertion failed for input `"hello world!"`
/// //         expected `"hello papa!"`
// //         result `"hello mom!"`
///
/// ```
pub fn report_fails<I: Debug, E: Debug, R: Debug>(fails: Vec<(I, E, R, usize)>) {
    if fails.is_empty() {
        return;
    }
    let mut report = String::with_capacity(1024);
    for (input, expected, result, case_id) in fails {
        if writeln!(
                &mut report,
                "test case {}: assertion failed for input `{:#?}`\n\texpected `{:#?}`\n\tresult `{:#?}`\n",
                case_id, input, expected, result
            )
            .is_err()
            {
                report += &format!("test case {}: assertion failed, unable to print message\n\n", case_id);
            };
    }
    panic!("One or more assertions failed:\n{}", report);
}
