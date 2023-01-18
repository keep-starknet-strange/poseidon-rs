#ifndef POSEIDON_H
#define POSEIDON_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

size_t c_hash_s128b(uint8_t *input, size_t input_len, uint8_t *output, size_t output_len);
size_t c_hash_sw2(uint8_t *input, size_t input_len, uint8_t *output, size_t output_len);

#ifdef __cplusplus
}
#endif

#endif
