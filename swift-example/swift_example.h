#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef void *(*Create)(void);

typedef void (*SetResult)(void *ptr, int result);

void register_plugin_impl(Create new_, SetResult set_result);

void *create(void);

int sum(int *ptr, int length);

void set_result(void *ptr, int result);
