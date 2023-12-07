// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifndef EXTERN_C
#ifdef __cplusplus
#define EXTERN_C extern "C"
#define MINTAKA_CPP
#else
#define EXTERN_C
#endif
#endif

// Declare Enum

#ifdef __cplusplus

#define MINTAKA_ENUM_START(name) enum class name {
#define MINTAKA_ENUM_END(name) }

#else

#define MINTAKA_ENUM_START(name) typedef enum name {
#define MINTAKA_ENUM_END } name

#endif

#define MINTAKA_DECLARE_ENUM(name, ...) MINTAKA_ENUM_START(name) __VA_ARGS__ MINTAKA_ENUM_END

// Declare Struct

#define MINTAKA_PASTE(...) __VA_ARGS__

#define MINTAKA_DECLARE_STRUCT(name, body) \
    EXTERN_C typedef struct name           \
    {                                      \
        MINTAKA_PASTE body                 \
    } name

// Experimental

#define MINTAKA_CLASS(...)
#define MINTAKA_STRUCT(...)
#define MINTAKA_ENUM(...)
#define MINTAKA_FUNCTION(...)
#define MINTAKA_PROPERTY(...)

// Import/Export API

#define NATIVE_API EXTERN_C

#ifdef MINTAKA_LIBRARY
#define MINTAKA_API DLLEXPORT
#else
#define MINTAKA_API DLLIMPORT
#endif

// Macros Utils

#define MINTAKA_EXPAND_MACRO(x) x
#define MINTAKA_STRINGIFY_MACRO(x) #x
