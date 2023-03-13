package main

/*
#cgo CFLAGS: -I../include
#cgo LDFLAGS: -L../target/release -lposeidon
#include "poseidon.h"
*/
import "C"

import (
    "bytes"
    "fmt"
    "unsafe"
)

func hash(input []byte) ([]byte) {
    output := make([]byte, 64)
    count := C.c_hash_sw2((*C.uint8_t)(unsafe.Pointer(&input[0])), C.size_t(len(input)), (*C.uint8_t)(unsafe.Pointer(&output[0])), C.size_t(len(output)))
    return output[:count]
}

func main() {
    input := []byte{
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    }
    expected := []byte{
        51, 97, 8, 169, 214, 14, 85, 134,
	227, 177, 119, 133, 150, 87, 35, 133,
	198, 59, 35, 158, 212, 201, 143, 162,
	0, 128, 37, 120, 30, 141, 158, 7,
        132, 185, 111, 187, 76, 21, 86, 162,
	188, 170, 204, 145, 75, 64, 181, 229,
	59, 166, 166, 106, 255, 150, 183, 219,
	150, 63, 15, 61, 0, 13, 132, 3,
    }
    output := hash(input)
    if bytes.Equal(output, expected) {
        fmt.Printf("[Success] starkware::sw2\n")
    } else {
        fmt.Printf("[Failed ] starkware::sw2: Unexpected elements: %v\n", output)
    }
}
