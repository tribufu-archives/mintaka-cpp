// Copyright (c) Tribufu. All Rights Reserved.

#include <mintaka/framework.h>

using namespace mintaka;

int main(int argc, char **argv)
{
    std::string url = "https://api.tribufu.com/v1/profiles/Tribufu";

    HttpClient *client = new HttpClient();
    FHttpResponse response = client->get(url);
    std::cout << "status_code: " << response.status_code << std::endl;

    if (response.body != nullptr)
    {
        json response_body = json::parse(response.body);
        std::string json_str = response_body.dump(4);
        std::cout << json_str << std::endl;
    }

    mintaka_http_free_response(response);

    return 0;
}
