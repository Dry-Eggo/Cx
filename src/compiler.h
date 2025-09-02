#pragma once

#include "./array.h"
#include "./string.h"

array_of(const char*, string_vec);

enum file_type
{
  FILE_TYPE_C,
  FILE_TYPE_OBJ,
};

struct compile_options
{
  string_vec*  input_paths;
  const char* output_path;
};

struct compile_options create_compile_options();
