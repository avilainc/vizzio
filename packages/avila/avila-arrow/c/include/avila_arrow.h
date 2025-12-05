/**
 * Avila Arrow C API
 *
 * C bindings for the avila-arrow library.
 */

#ifndef AVILA_ARROW_H
#define AVILA_ARROW_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Get library version
 */
const char* avila_arrow_version(void);

/**
 * Initialize the library
 */
int32_t avila_arrow_init(void);

#ifdef __cplusplus
}
#endif

#endif /* AVILA_ARROW_H */
