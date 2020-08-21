package main

import (
    "encoding/hex"
    "fmt"
    "log"
)

func main() {
    input := "1c0111001f010100061a024b53535009181c"
    inputBytes, err := hex.DecodeString(input)
    if err != nil {
        log.Fatal(err)
    }
    byteStringToXor := "686974207468652062756c6c277320657965"
    bytesToXor, err := hex.DecodeString(byteStringToXor)
    if err != nil {
        log.Fatal(err)
    }
    var xoredBytes []byte
    for i, b := range inputBytes {
        xoredBytes = append(xoredBytes, b ^ bytesToXor[i])
    }
    xoredString := hex.EncodeToString(xoredBytes)
    fmt.Printf("%s\n", xoredString)
}
