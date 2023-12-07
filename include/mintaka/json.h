// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#include <mintaka/prelude.h>
#include <mintaka/native.h>

#ifdef MINTAKA_CPP

#include <nlohmann/json.hpp>
using json = nlohmann::json;

namespace mintaka
{
    namespace Json
    {
        static std::string from_string(std::string &str)
        {
            return json::parse(str);
        }

        static std::string to_string(json value)
        {
            return value.dump();
        }
    };
}

#endif
