package main

/*
#cgo CFLAGS: -I../../include
#cgo LDFLAGS: -L../../target/release -lposeidon
#include "poseidon.h"
*/
import "C"

import (
    "bytes"
    "fmt"
    "unsafe"
)

func hash(input []byte) ([]byte) {
    output := make([]byte, 32)
    count := C.c_hash_s128b((*C.uint8_t)(unsafe.Pointer(&input[0])), C.size_t(len(input)), (*C.uint8_t)(unsafe.Pointer(&output[0])), C.size_t(len(output)))
    return output[:count]
}

func main() {
    input := []byte{
        7, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        98, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    }
    expected := []byte{
        235, 45, 90, 171, 223, 52, 1, 195,
        131, 207, 44, 19, 33, 28, 64, 72,
        83, 235, 187, 118, 177, 146, 208, 154,
        187, 33, 119, 100, 131, 7, 112, 24,
    }
    output := hash(input)
    if bytes.Equal(output, expected) {
        fmt.Printf("[Success] s128b\n")
    } else {
	fmt.Printf("[Failed ] s128b: Unexpected elements: %v\n", output)
    }
}
