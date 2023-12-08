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

NATIVE_API void mintaka_http_free_response(struct FHttpResponse response);

NATIVE_API struct FHttpResponse mintaka_http_get(const char *url);

NATIVE_API struct FHttpResponse mintaka_http_post(const char *url, const char *body);
