#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Rust_Camera Rust_Camera;

typedef struct Rust_World Rust_World;

typedef struct Rust_WorldHandle {
  struct Rust_World *world;
  struct Rust_Camera *camera;
} Rust_WorldHandle;

typedef struct Rust_ColorU8 {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Rust_ColorU8;

typedef struct Rust_CFramebuffer {
  size_t width;
  size_t height;
  struct Rust_ColorU8 *pixels;
} Rust_CFramebuffer;

typedef struct Rust_NVec3 {
  float x;
  float y;
  float z;
} Rust_NVec3;

#define Rust_X_AXIS (Rust_NVec3){ .x = 1.0, .y = 0.0, .z = 0.0 }

#define Rust_Y_AXIS (Rust_NVec3){ .x = 0.0, .y = 1.0, .z = 0.0 }

#define Rust_Z_AXIS (Rust_NVec3){ .x = 0.0, .y = 0.0, .z = 1.0 }

struct Rust_WorldHandle *load_world(const char *source);

struct Rust_Camera *move_camera_position(struct Rust_Camera *camera, float x, float y, float z);

struct Rust_CFramebuffer render(struct Rust_CFramebuffer framebuffer,
                                const struct Rust_WorldHandle *handle);
