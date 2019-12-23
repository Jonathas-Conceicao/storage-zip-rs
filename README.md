[![Crates.io](https://img.shields.io/crates/v/storage-zip.svg)](https://crates.io/crates/storage-zip)
[![Documentation](https://docs.rs/storage_zip/badge.svg)](https://docs.rs/storage_zip)

# storage_zip

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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
