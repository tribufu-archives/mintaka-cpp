// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#include <mintaka/framework.h>

int main(int argc, char **argv)
{
    const char *url = "https://api.tribufu.com/v1/profiles/Tribufu";

    struct FHttpResponse response = mintaka_http_get(url);
    printf("status_code: %d\n", response.status_code);

    if (response.body != NULL)
    {
        printf("%s\n", response.body);
    }

    mintaka_http_free_response(response);

    return 0;
}
