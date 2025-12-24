// SPDX-License-Identifier: 0BSD
// Drop Root Capabilities
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

#![no_std]

//! On Linux, the "root" user (UID 0) has some special capabilities that "regular" users do **not** normally have. This can result in weird behavior, e.g., if unit tests (or integration tests) are executed in the context of the "root" user, as Docker&REG; containers do by default! For example, a file that **should not** be accessible (according to its access permissions) may suddenly become accessible – because the "root" user has the `CAP_DAC_OVERRIDE` capability, which allows them to access the file *regardless of the access permissions*. As a result, a test case that expects `File::open()` to return a "permission denied" error will suddenly start to fail &#x1F628;
//!
//! This crate uses the Linux syscall [`prctl()`](https://man7.org/linux/man-pages/man2/prctl.2.html) with argument [`PR_CAPBSET_DROP`](https://man7.org/linux/man-pages/man2/PR_CAPBSET_DROP.2const.html) to drop the "root"-specific capabilities at application startup and thus restores the expected behavior. It does *nothing* on other platforms.
//!
//! ## Usage
//!
//! Simply add the following code to the top of your test module(s):
//!
//! ```
//! #[used]
//! static DROP_ROOT_CAPS: () = drop_root_caps::set_up();
//! ```
//!
//! ## See also
//!
//! &#x1F517; <https://crates.io/crates/drop-root-caps>  
//! &#x1F517; <https://github.com/lordmulder/drop-root-caps>

#[cfg(target_os = "linux")]
mod linux {
    use core::hint::black_box;
    use ctor::ctor;
    use libc::{c_long, prctl, PR_CAPBSET_DROP};

    // Capability constants
    // See linux/include/uapi/linux/capability.h for details!
    const CAP_CHOWN: c_long = 0;
    const CAP_DAC_OVERRIDE: c_long = 1;
    const CAP_DAC_READ_SEARCH: c_long = 2;
    const CAP_FOWNER: c_long = 3;
    const CAP_FSETID: c_long = 4;
    const CAP_LINUX_IMMUTABLE: c_long = 9;
    const CAP_MKNOD: c_long = 27;
    const CAP_MAC_OVERRIDE: c_long = 32;

    /// The initialization function that will run before the "main" function (or any test function)
    #[ctor]
    unsafe fn initialize() {
        for capability in [CAP_CHOWN, CAP_DAC_OVERRIDE, CAP_DAC_READ_SEARCH, CAP_FOWNER, CAP_FSETID, CAP_LINUX_IMMUTABLE, CAP_MAC_OVERRIDE, CAP_MKNOD] {
            black_box(prctl(PR_CAPBSET_DROP, capability, 0 as c_long, 0 as c_long, 0 as c_long));
        }
    }
}

/// Dummy set-up function to ensure that our crate will actually be linked
pub const fn set_up() {}
