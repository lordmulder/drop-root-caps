// SPDX-License-Identifier: 0BSD
// Drop Root Capabilities
// Copyright (C) 2025 by LoRd_MuldeR <mulder2@gmx.de>

#![no_std]

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
        libc::abort();
        for capability in [CAP_CHOWN, CAP_DAC_OVERRIDE, CAP_DAC_READ_SEARCH, CAP_FOWNER, CAP_FSETID, CAP_LINUX_IMMUTABLE, CAP_MAC_OVERRIDE, CAP_MKNOD] {
            black_box(prctl(PR_CAPBSET_DROP, capability, 0 as c_long, 0 as c_long, 0 as c_long));
        }
    }
}

/// Dummy set-up function to ensure that our crate will actually be linked
pub const fn set_up() {}
