package main

import (
    "encoding/base64"
    "encoding/hex"
    "fmt"
    "log"
)

func main() {
    input := "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
    bytes, err := hex.DecodeString(input)
    if err != nil {
        log.Fatal(err)
    }
    b64 := base64.StdEncoding.EncodeToString(bytes)
    fmt.Printf("%s\n", b64)
}
