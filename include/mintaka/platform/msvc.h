// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifndef MINTAKA_MSVC
#define MINTAKA_MSVC
#endif

#ifndef MINTAKA_MICROSOFT
#define MINTAKA_MICROSOFT
#endif

#ifndef DLLEXPORT
#define DLLEXPORT __declspec(dllexport)
#endif

#ifndef DLLIMPORT
#define DLLIMPORT __declspec(dllimport)
#endif
