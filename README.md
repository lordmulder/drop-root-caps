# Drop Root Capabilities

[![no_std](https://img.shields.io/badge/rust-no__std-orchid?logo=rust)](https://docs.rust-embedded.org/book/intro/no-std.html)
[![Crates.io](https://img.shields.io/crates/v/drop-root-caps)](https://crates.io/crates/drop-root-caps)
[![Downloads](https://img.shields.io/crates/d/drop-root-caps)](https://crates.io/crates/drop-root-caps)
[![License](https://img.shields.io/crates/l/sponge-hash-aes256)](https://opensource.org/license/0BSD)

A simple crate to drop "root" user capabilities on Linux.

On Linux, the "root" user (UID 0) has some special capabilities that "regular" users do **not** normally have. This can result in weird behavior, e.g., if unit tests (or integration tests) are executed in the context of the "root" user, as GitHub actions do by default! For example, a file that **should not** be accessible (according to its access permissions) may suddenly become accessible – because the "root" user has the `CAP_DAC_OVERRIDE` capability, which allows the "root" user to access the file *regardless of the access permissions*. As a result, a test case that expects the `File::open()` to return a "permission denied" error (and rightfully so!) will suddenly start to fail &#x1F628;

This crate uses the syscall [`prctl()`](https://man7.org/linux/man-pages/man2/prctl.2.html) with argument [`PR_CAPBSET_DROP`](https://man7.org/linux/man-pages/man2/PR_CAPBSET_DROP.2const.html) to drop the "root"-specific capabilities at application startup and thus restores the expected behavior.

## Usage

Add the following to your **`Cargo.toml`** file:

```
[dev-dependencies]
drop-root-caps = "1.0.2"
```

Also, you must add following "boilerplate" code to your test module, because otherwise the Rust compiler will simply optimizes away the `drop-root-caps` dependency &#128556;

```
#[allow(unused_imports)]
use drop_root_caps;
```

*Note:* You probably do **not** want to add this crate to your `[dependencies]`, *only* to the `[dev-dependencies]` &#x1F4A1;

## License

This software is released under the BSD Zero Clause (“0BSD”) License.

Copyright (C) 2025 by LoRd_MuldeR &lt;mulder2@gmx.de&gt;.
