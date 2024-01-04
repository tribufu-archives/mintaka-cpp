// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#include <mintaka/prelude.h>

struct FHttpHeader
{
    const char *name;
    const char *value;
};

struct FHttpHeaders
{
    const struct FHttpHeader *entries;
    uintptr_t count;
};

struct FHttpResponse
{
    int status_code;
    const struct FHttpHeaders *headers;
    const char *body;
};

/**
 * The timestamp that has been compiled.
 */
NATIVE_API const char *mintaka_get_build_timestamp(void);

/**
 * The current cargo build profile.
 */
NATIVE_API const char *mintaka_get_cargo_profile(void);

/**
 * Returns the model of the current CPU.
 */
NATIVE_API const char *mintaka_get_cpu_model(void);

/**
 * Returns the vendor of the current CPU.
 */
NATIVE_API const char *mintaka_get_cpu_vendor(void);

/**
 * Returns the free memory of the current system.
 */
NATIVE_API uint64_t mintaka_get_free_memory(void);

/**
 * Returns the free swap of the current system.
 */
NATIVE_API uint64_t mintaka_get_free_swap(void);

/**
 * Returns the host name of the current system.
 */
NATIVE_API const char *mintaka_get_host_name(void);

/**
 * Returns the kernel version of the current system.
 */
NATIVE_API const char *mintaka_get_kernel_version(void);

/**
 * The LLVM version of the Rust compiler.
 */
NATIVE_API const char *mintaka_get_llvm_version(void);

/**
 * Returns the number of logical cores of the current CPU.
 */
NATIVE_API uint64_t mintaka_get_logical_cores(void);

/**
 * Returns the number of physical cores of the current CPU.
 */
NATIVE_API uint64_t mintaka_get_physical_cores(void);

/**
 * The release channel of the Rust compiler.
 */
NATIVE_API const char *mintaka_get_rust_channel(void);

/**
 * The version of the Rust compiler.
 */
NATIVE_API const char *mintaka_get_rust_version(void);

/**
 * Returns the edition of the current system.
 */
NATIVE_API const char *mintaka_get_system_edition(void);

/**
 * Returns the name of the current system.
 */
NATIVE_API const char *mintaka_get_system_name(void);

/**
 * Returns the uptime of the current system.
 */
NATIVE_API uint64_t mintaka_get_system_uptime(void);

/**
 * Returns the OS version of the current system.
 */
NATIVE_API const char *mintaka_get_system_version(void);

/**
 * The target triple that is being compiled for.
 */
NATIVE_API const char *mintaka_get_target_triple(void);

/**
 * Returns the total memory of the current system.
 */
NATIVE_API uint64_t mintaka_get_total_memory(void);

/**
 * Returns the total swap of the current system.
 */
NATIVE_API uint64_t mintaka_get_total_swap(void);

/**
 * Returns the used memory of the current system.
 */
NATIVE_API uint64_t mintaka_get_used_memory(void);

/**
 * Returns the used swap of the current system.
 */
NATIVE_API uint64_t mintaka_get_used_swap(void);

/**
 * Get the current version of this library.
 */
NATIVE_API const char *mintaka_get_version(void);

NATIVE_API void mintaka_http_free_response(struct FHttpResponse response);

NATIVE_API struct FHttpResponse mintaka_http_get(const char *url);

NATIVE_API struct FHttpResponse mintaka_http_post(const char *url, const char *body);

NATIVE_API void mintaka_log_debug(const char *message);

NATIVE_API void mintaka_log_error(const char *message);

NATIVE_API void mintaka_log_info(const char *message);

NATIVE_API void mintaka_log_init(void);

NATIVE_API void mintaka_log_trace(const char *message);

NATIVE_API void mintaka_log_warn(const char *message);
