// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#include <mintaka/prelude.h>

NATIVE_API void mintaka_fs_canonicalize_path(void);

NATIVE_API void mintaka_fs_copy_file(void);

NATIVE_API void mintaka_fs_create_dir(void);

NATIVE_API void mintaka_fs_create_dir_all(void);

NATIVE_API void mintaka_fs_create_hard_link(void);

NATIVE_API void mintaka_fs_create_symlink(void);

NATIVE_API void mintaka_fs_delete_dir(void);

NATIVE_API void mintaka_fs_delete_dir_all(void);

NATIVE_API void mintaka_fs_delete_file(void);

NATIVE_API void mintaka_fs_get_metadata(void);

NATIVE_API void mintaka_fs_read_dir(void);

NATIVE_API void mintaka_fs_read_file(void);

NATIVE_API void mintaka_fs_read_file_to_string(void);

NATIVE_API void mintaka_fs_read_link(void);

NATIVE_API void mintaka_fs_set_permissions(void);

NATIVE_API void mintaka_fs_write_file(void);

NATIVE_API void mintaka_http_delete(void);

NATIVE_API void mintaka_http_get(void);

NATIVE_API void mintaka_http_options(void);

NATIVE_API void mintaka_http_patch(void);

NATIVE_API void mintaka_http_post(void);

NATIVE_API void mintaka_http_put(void);
