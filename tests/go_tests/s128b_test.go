package main

import (
    "bytes"
    "testing"
)

func TestHash(t *testing.T) {
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
    if !bytes.Equal(output, expected) {
	t.Errorf("Unexpected output %v. Expected %v", output, expected)
    }
}
