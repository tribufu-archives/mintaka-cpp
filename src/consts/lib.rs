// Copyright (c) Tribufu. All Rights Reserved.

/// The version of the framework.
pub const MINTAKA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The target triple that is being compiled for.
pub const TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");

/// The LLVM version of the Rust compiler.
pub const LLVM_VERSION: &str = env!("VERGEN_RUSTC_LLVM_VERSION");

/// The version of the Rust compiler.
pub const RUSTC_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

/// The release channel of the Rust compiler.
pub const RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");

/// The git commit hash of the Rust compiler.
pub const RUSTC_COMMIT: &str = env!("VERGEN_RUSTC_COMMIT_HASH");

/// The current cargo build profile.
pub const CARGO_PROFILE: &str = env!("VERGEN_CARGO_PROFILE");

/// The timestamp that has been compiled.
pub const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        assert_eq!(MINTAKA_VERSION, env!("CARGO_PKG_VERSION"));
    }
}
