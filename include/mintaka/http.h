// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#include <mintaka/prelude.h>
#include <mintaka/native.h>

#ifdef MINTAKA_CPP

namespace mintaka
{
    class HttpClient
    {
    public:
        HttpClient()
        {
        }

        ~HttpClient()
        {
        }

        FHttpResponse get(std::string &url)
        {
            return mintaka_http_get(url.c_str());
        }

        FHttpResponse post(std::string &url, json &body)
        {
            std::string body_str = body.dump();
            return mintaka_http_post(url.c_str(), body_str.c_str());
        }
    };

    class HttpRequest
    {

    public:
        HttpRequest()
        {
        }

        ~HttpRequest()
        {
        }
    };

    class HttpResponse
    {
    public:
        HttpResponse()
        {
        }

        ~HttpResponse()
        {
        }
    };

    class HttpHeaders
    {
    public:
        HttpHeaders()
        {
        }

        ~HttpHeaders()
        {
        }
    };

    class HttpHeader
    {
    public:
        HttpHeader()
        {
        }

        ~HttpHeader()
        {
        }
    };
}

#endif
