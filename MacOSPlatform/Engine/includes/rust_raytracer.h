#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Rust_Color {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Rust_Color;

typedef struct Rust_Array_Color {
  size_t count;
  struct Rust_Color *data;
} Rust_Array_Color;

typedef struct Rust_Bitmap {
  size_t width;
  struct Rust_Array_Color pixels;
} Rust_Bitmap;

typedef struct Rust_NVec3 {
  float x;
  float y;
  float z;
} Rust_NVec3;

#define Rust_X_AXIS (Rust_NVec3){ .x = 1.0, .y = 0.0, .z = 0.0 }

#define Rust_Y_AXIS (Rust_NVec3){ .x = 0.0, .y = 1.0, .z = 0.0 }

#define Rust_Z_AXIS (Rust_NVec3){ .x = 0.0, .y = 0.0, .z = 1.0 }

struct Rust_Bitmap create_bitmap(size_t width, size_t height, const char *source);

char *rust_hello(const char *to);

void rust_hello_free(char *s);
