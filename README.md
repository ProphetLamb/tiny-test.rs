# `tiny-test`
`tiny-test` is collection of functions simplifying test assertions in rust.

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
// test case 2: assertion failed for input `"hello world!"`
//         expected `"hello papa!"`
//
```