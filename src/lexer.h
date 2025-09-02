#pragma once

#include "./compiler.h"

struct sloc
{
  int begin_line, end_line;
  int begin_col,  end_col;

  const char* file_name;
};

enum token_type
{
  /* keywords */
  TINT, TCHAR, TVOID, TLONG, TDOUBLE,
  TUNSIGNED, TSIGNED, TSHORT,
  TSTRUCT, TENUM, TCONST,

  /* values */
  TINT_LIT, TSTRING_LIT, TCHAR_LIT,

  /* punctuations */
  TLPAREN, TRPAREN, TLBRACE, TRBRACE,
  TCOMMA, TSEMI, TCOLON,
  TLSBRACE, // [
  TRSBRACE, // ]

  /* operators */
  TADD, TSUB,
  TMUL, TDIV,
  TE, TEQ_EQ,

  /* sentinels */
  TEOF,
};

struct token
{
  enum token_type type;
  struct sloc     location;
  const char*     data;
};
array_of(struct token, token_list);

enum lex_error {
  LEX_ERR_NO_SUCH_FILE,
  LEX_ERR_NONE,
};

enum lex_error lex_source_file(const char* path, struct compile_options options);
token_list*     get_tokens();
