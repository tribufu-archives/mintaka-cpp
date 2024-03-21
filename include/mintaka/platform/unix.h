// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#pragma once

#ifndef MINTAKA_UNIX
#define MINTAKA_UNIX
#endif

#ifndef DLLEXPORT
#define DLLEXPORT __attribute__((visibility("default")))
#endif

#ifndef DLLIMPORT
#define DLLIMPORT __attribute__((visibility("default")))
#endif
