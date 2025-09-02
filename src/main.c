#include <stdio.h>
#include <stdarg.h>
#include <sys/stat.h>

#include "./array.h"
#include "./string.h"
#include "./compiler.h"
#include "./lexer.h"

const char* prog_name = NULL;
struct compile_options options = {0};

void error(const char* fmt, ...)
{
  va_list args;
  va_start(args, fmt);
  fprintf(stderr, "error: ");
  vfprintf(stderr, fmt, args);
  fprintf(stderr, "\n");
}

void print_usage(int err)
{
  FILE* file = err == 0 ? stdout: stderr;
  fprintf(file, "usage: %s [options] <input>\n", prog_name);
  fprintf(file, "options:\n");
  fprintf(file, "    -i <path>          compile single source file at `path`\n");
}

int parse_arguments(int argc, char **argv)
{
  prog_name = argv[0];
  if (argc == 1) {
    /* just print help an exit */
    print_usage(0);
    exit(0);
  }

  int index = 1;
  while (index < argc) {
    const char* arg = argv[index++];
    
    if (!strcmp(arg, "-i")) {
      if (index >= argc) {
	error("'-i' expected an argument");
	goto err;
      }
      string_vec_push(options.input_paths, argv[index++]);
    }
    else if (!strcmp(arg, "-o")) {
      if (index >= argc) {
	error("'-o' expected an argument");
	goto err;
      }
      options.output_path = argv[index++];
    }
    else {
      if (*arg && *arg == '-') {
	print_usage(1);
	error("unknown flag or argument: '%s'", arg);
	goto err;
      }
      else {
	string_vec_push(options.input_paths, arg);	
      }
    }
  }

  return 0;
 err:
  return 1;
}

const char* extension(const char* p)
{
  const char* e = strrchr(p, '.');
  if (e) return strdup(e + 1);
  return "";
}

int file_exists(const char* path)
{
  struct stat tmp;
  return (stat(path, &tmp) == 0);
}

int main(int argc, char **argv)
{
  options = create_compile_options();
  if (parse_arguments(argc, argv) != 0) return 1;

  for (int n = 0; n < options.input_paths->length; n++) {
    const char* input_path = string_vec_get(options.input_paths, n);

    if (!strcmp(extension(input_path), "c")) {
      enum lex_error err;
      if ((err = lex_source_file(input_path, options)) != LEX_ERR_NONE) {
	if (err == LEX_ERR_NO_SUCH_FILE) {
	  error("no such file at path: `%s`", input_path);
	} else
	  error("failed to lex file with errors: '%s'", input_path);
	return 1;
      }
    }
  }
}
