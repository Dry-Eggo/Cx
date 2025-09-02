#include "./compiler.h"

struct compile_options create_compile_options()
{
  struct compile_options opt = {0};
  opt.input_paths = init_string_vec();
  return opt;
}
