/*!
This crate offers utility functions for zipping `Option` and `Result`
values into tuples.

This is intended to be used when different actions yields multiple
`Option` or `Result` values but the program flow can only proceed if
all of them are `Some` or `Ok`.

When it is used with a `Result` based value, it requires that all the
`Result` values being zipped have the same error type and it will
evaluate to the first error or to the tuple with all the `Ok` values.

With the crate you can do:
```rust
use storage_zip::OptionZip;

let option_value_one = Some(0);
let option_value_two = Some(5);

let zipped_options = Option::zip(option_value_one, option_value_two);
assert_eq!(zipped_options, Some((0, 5)));
```

The crate also provides zip functions with more arguments:

```rust
use storage_zip::ResultZip;
use std::fs::File;
use std::io;

let f1 = File::open("file1");
let f2 = File::open("file2");
let f3 = File::open("file3");

let zipped_result: Result<(File, File, File), io::Error> = Result::zip3(f1, f2, f3);
```
*/

#![no_std]

mod option_zip;
mod result_zip;

pub use option_zip::OptionZip;
pub use result_zip::ResultZip;

#[test]
fn option_zip() {
    let option_value_one = Some(0);
    let option_value_two = Some(5);

    let zipped_options = Option::zip(option_value_one, option_value_two);
    assert_eq!(zipped_options, Some((0, 5)));
}

#[test]
fn option_zip_with_different_types() {
    let option_value_one = Some(0);
    let option_value_two = Some(5.);
    let option_value_three = Some("Three");

    let zipped_options = Option::zip3(option_value_one, option_value_two, option_value_three);
    assert_eq!(zipped_options, Some((0, 5., "Three")));
}

#[test]
fn option_zip_with_none() {
    let option_value_one = Some(0);
    let option_value_two: Option<i32> = None;

    let zipped_options = Option::zip(option_value_one, option_value_two);
    assert_eq!(zipped_options, None);

    let option_value_one: Option<i32> = None;
    let option_value_two = Some(5);

    let zipped_options = Option::zip(option_value_one, option_value_two);
    assert_eq!(zipped_options, None);
}

#[test]
fn result_zip() {
    let result_value_one = Result::<i32, ()>::Ok(0);
    let result_value_two = Result::<i32, ()>::Ok(5);

    let zipped_options = Result::zip(result_value_one, result_value_two);
    assert_eq!(zipped_options, Ok((0, 5)));
}

#[test]
fn result_zip_with_different_types() {
    let result_value_one = Result::<i32, ()>::Ok(0);
    let result_value_two = Result::<f32, ()>::Ok(5.);
    let result_value_three = Result::<&str, ()>::Ok("three");

    let zipped_options = Result::zip3(result_value_one, result_value_two, result_value_three);
    assert_eq!(zipped_options, Ok((0, 5., "three")));
}

#[test]
fn result_zip_with_err() {
    let result_value_one = Result::<i32, ()>::Err(());
    let result_value_two = Result::<i32, ()>::Ok(5);

    let zipped_options = Result::zip(result_value_one, result_value_two);
    assert_eq!(zipped_options, Err(()));

    let result_value_one = Result::<i32, ()>::Ok(0);
    let result_value_two = Result::<i32, ()>::Err(());

    let zipped_options = Result::zip(result_value_one, result_value_two);
    assert_eq!(zipped_options, Err(()));
}
