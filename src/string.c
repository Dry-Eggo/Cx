#include "./string.h"

#include <stdlib.h>
#include <string.h>
#include <stdarg.h>
#include <stdio.h>

static void init_string(struct string* string)
{
  string->len = 0;
  string->cap = 255;
  string->buffer = (char*)malloc(sizeof(char)*string->cap);
}

void string_append_char(struct string* s, char c)
{
  if (s->cap <= 0) init_string(s);
  if (s->len >= s->cap) {
    int new_cap = s->cap*2;
    char* new_buffer = (char*)malloc(sizeof(char)*new_cap);
    memcpy(new_buffer, s->buffer, s->len*sizeof(char));
    free(s->buffer);
    s->buffer = new_buffer;
    s->cap    = new_cap;
  }
  s->buffer[s->len++] = c;
}

void string_append(struct string* string, const char* fmt, ...)
{
  if (string->cap <= 0) init_string(string);
  va_list args;
  va_start(args, fmt);

  int size = vsnprintf(NULL, 0, fmt, args);

  char* str = (char*)malloc(size + 1);
  va_start(args, fmt);
  vsprintf(str, fmt, args);

  int n = 0;
  while (str[n]) string_append_char(string, str[n++]);
  free(str);
}

void string_free(struct string* s)
{
  if (s) {
    free(s->buffer);
    s->len = 0;
    s->cap = 0;
  }
}

const char* string_to_cstr(struct string* string)
{
  if (string->cap <= 0) init_string(string);
  char* buf = string->buffer;
  buf[string->len] = 0;
  return buf;
}
