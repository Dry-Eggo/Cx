#pragma once

#include <string.h>
#include <stdlib.h>

#define array_of(T, name)						\
  typedef struct {							\
    int    length;							\
    int    capacity;							\
    T*     data;							\
  } name;								\
									\
  static inline name* init_##name() {					\
    name* arr = (name*)malloc(sizeof(name));				\
    arr->length = 0;							\
    arr->capacity = 255;						\
    arr->data = malloc(sizeof(T)*arr->capacity);			\
    return arr;								\
  }									\
									\
  static inline void name##_push(name* arr, T obj) {			\
    if (arr->length >= arr->capacity) {					\
      size_t new_capacity = arr->capacity*2;				\
      T* new_data = malloc(sizeof(T)*new_capacity);			\
      memcpy(new_data, arr->data, arr->length*sizeof(T));		\
      arr->data = new_data;						\
      arr->capacity = new_capacity;					\
    }									\
    arr->data[arr->length++] = obj;					\
  }									\
  									\
  static inline T name##_get(name* arr, size_t n) {			\
    return arr->data[n];						\
  }									
  
