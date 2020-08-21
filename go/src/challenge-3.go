package main

import (
    "encoding/hex"
    "fmt"
    "log"
)

func main() {
    input := "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    inputBytes, err := hex.DecodeString(input)
    if err != nil {
        log.Fatal(err)
    }
    byteStringToXor := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    bytesToXor, err := hex.DecodeString(byteStringToXor)
    if err != nil {
        log.Fatal(err)
    }
    var xoredBytes []([]byte)

    for _, char := range bytesToXor {
        var xoredByCharacter []byte
        for _, b := range inputBytes {
            xoredByCharacter = append(xoredByCharacter, char ^ b)
        }
        xoredBytes = append(xoredBytes, xoredByCharacter)
    }
    //fmt.Printf("%s\n", xoredBytes) 
    //xoredString := hex.EncodeToString(xoredBytes)
   
}
