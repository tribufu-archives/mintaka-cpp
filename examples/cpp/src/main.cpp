// Copyright (c) Tribufu. All Rights Reserved.

#include <mintaka/framework.h>

using namespace mintaka;

int main(int argc, char **argv)
{
    mintaka_fs_canonicalize_path();
    mintaka_fs_copy_file();
    mintaka_fs_create_dir();
    mintaka_fs_create_dir_all();
    mintaka_fs_create_hard_link();
    mintaka_fs_create_symlink();
    mintaka_fs_delete_dir();
    mintaka_fs_delete_dir_all();
    mintaka_fs_delete_file();
    mintaka_fs_get_metadata();
    mintaka_fs_read_dir();
    mintaka_fs_read_file();
    mintaka_fs_read_file_to_string();
    mintaka_fs_read_link();
    mintaka_fs_set_permissions();
    mintaka_fs_write_file();
    mintaka_http_delete();
    mintaka_http_get();
    mintaka_http_options();
    mintaka_http_patch();
    mintaka_http_post();
    mintaka_http_put();

    json json_obj = {
        {"nome", "John"},
        {"idade", 25},
        {"cidade", "Exemplo"}};

    std::string json_str = Json::to_string(json_obj);

    std::cout << json_str << std::endl;

    return 0;
}
