// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifdef _WIN32
#include <mintaka/platform/windows.h>
#endif

#ifdef __MACH__
#include <mintaka/platform/mac.h>
#endif

#ifdef __linux__
#include <mintaka/platform/linux.h>
#endif

#ifdef __FreeBSD__
#include <mintaka/platform/freebsd.h>
#endif

#ifdef __ANDROID__
#include <mintaka/platform/android.h>
#endif

#ifdef __APPLE__
// #include <TargetConditionals.h>
#ifdef TARGET_OS_IPHONE
#include <mintaka/platform/ios.h>
#endif
#endif
