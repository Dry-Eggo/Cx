#pragma once


struct string {
  char* buffer;
  int   len;
  int   cap;
};

const char* string_to_cstr(struct string* string);
void string_append_char(struct string* string, char c);
void string_append(struct string* string, const char* fmt, ...);
void string_free(struct string* s);
