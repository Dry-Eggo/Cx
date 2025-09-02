#include <stdio.h>

#include "./lexer.h"

char* current_source = NULL;
char* source_end = NULL;
const char* current_path = NULL;
struct token current_token = {0};
token_list*  tokens = NULL;

int lline = 1;
int lcol = 0;
int lpos  = 0;

int pline = 1;
int pcol = 0;
int ppos  = 0;

extern int file_exists(const char*);

int init_lexer(const char* p)
{
  if (!file_exists(p)) return LEX_ERR_NO_SUCH_FILE;
  
  FILE* f = fopen(p, "rb");
  if (!f) {
    fclose(f);
    return 1;
  }

  
  fseek(f, 0, SEEK_END);
  int size = ftell(f);
  rewind(f);
  
  current_source = (char*)malloc(size*sizeof(char));
  fread((void*)current_source, size, 1, f);  
  fclose(f);
  source_end = current_source + size;
  current_path = p;
  tokens = init_token_list();
  return LEX_ERR_NONE;
}

void save_lexer_position()
{
  pline = lline;
  pcol  = lcol;
  ppos  = lpos;
}

struct sloc get_current_location()
{
  struct sloc loc = {0};
  loc.begin_line = pline;
  loc.end_line   = lline;
  loc.begin_col  = pcol;
  loc.end_col    = lcol - 1;
  return loc; 
}

struct token get_token(const char* data, enum token_type type)
{
  struct token tok = {0};
  tok.location = get_current_location();
  tok.data     = data;
  tok.type     = type;
  return tok;
}

enum lex_error lex_source_file(const char* path, struct compile_options opt)
{
  enum lex_error err = LEX_ERR_NONE;
  if ((err = init_lexer(path)) != LEX_ERR_NONE) return err;
  
  return LEX_ERR_NONE;
}
token_list* get_tokens()
{
  return tokens;
}
