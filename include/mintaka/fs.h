// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#include <mintaka/prelude.h>
#include <mintaka/native.h>

#ifdef MINTAKA_CPP

namespace mintaka
{
    class File
    {
    private:
        std::string path;

    public:
        File(std::string path)
        {
            this->path = path;
        }

        ~File()
        {
        }
    };
}

#endif
