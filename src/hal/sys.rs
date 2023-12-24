// Copyright (c) Tribufu. All Rights Reserved.

use sysinfo::{System, SystemExt};

pub(crate) fn get_system_info() -> System {
    let mut sys = System::new();
    sys.refresh_all();
    sys
}

/// Returns the uptime of the current system.
pub fn get_system_uptime() -> u64 {
    get_system_info().uptime()
}

/// Returns the name of the current system.
pub fn get_system_name() -> Option<String> {
    get_system_info().name()
}

/// Returns the OS version of the current system.
pub fn get_system_version() -> Option<String> {
    get_system_info().os_version()
}

/// Returns the OS edition of the current system.
pub fn get_system_edition() -> Option<String> {
    get_system_info().long_os_version()
}

/// Returns the kernel version of the current system.
pub fn get_kernel_version() -> Option<String> {
    get_system_info().kernel_version()
}

/// Returns the host name of the current system.
pub fn get_host_name() -> Option<String> {
    get_system_info().host_name()
}
