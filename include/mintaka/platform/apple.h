// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifndef MINTAKA_APPLE
#define MINTAKA_APPLE
#endif

#ifndef DLLEXPORT
#define DLLEXPORT __attribute__((visibility("default")))
#endif

#ifndef DLLIMPORT
#define DLLIMPORT __attribute__((visibility("default")))
#endif
