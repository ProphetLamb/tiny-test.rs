# `tiny-test`
`tiny-test` is collection of functions simplifying test assertions in rust.

## Motivation

When writing unit-tests, especially for parsers and similar problems, programmers tend to write a single-test-multiple-data structure.

This generally is done, by calling the test with a variety of inputs and expecting some output. This works completely fine when all tests succeed, but many trivial implementation `panic!` on the first failure, the result is, that the remaining test-cases are not executed at all. Often they thus don't report the specific test data, such as input or the number of the test case that yielded the error.

This crate aims to solve all of these problems.

## Features

- All test-cases are run, regardless of a failed assertion.
- When debugging `panic!`s on failed assertion, right after the test-case.
- Failed test-cases are reported easily understandable manner.
- Failed test-cases include the test-case number, input, expectation and result, that caused the failure. 

## Usage

### `collect_fails!`

**Basic usage:**
```rust
#[test]
fn test_parse_fragment_any() {
    report_fails(collect_fails!(
        // input type
        &str,
        // output type
        IResult<&str, Fragment, ()>,
        // test cases in format (input, expected)
        vec![
            ("/", Ok(("", Fragment::Separator))),
            ("///", Ok(("", Fragment::Separator))),
            ("path/to/file", Ok(("/to/file", Fragment::Plain("path"))))
        ].into_iter(),
        // test function
        parse_fragment
    ));
}
```

**Custom assertion:**
```rust
fn test_in_range() {
    report_fails(collect_fails!(
        usize,
        std::ops::Range<usize>,
        usize,
        vec![(2, 1..4), (3, 4..6), (0, 1..3)].into_iter(),
        |input| input + 2,
        |output: &usize, expected: &Range<usize>| expected.contains(output)
    ));
}
```

## `report_fails`

Usually used in combination with `collect_fails!`

**Basic usage:**
```rust
report_fails(vec![
    ("input string", "expected string", "", 1),
    ("hello world!", "hello papa!", "hello mom!", 2),
])

// One or more assertions failed:
// test case 1: assertion failed for input `"input string"`
//         expected `"expected string"`
//         result `""`
//
// test case 2: assertion failed for input `"hello world!"`
//         expected `"hello papa!"`
//         result `"hello mom!"`
//
```