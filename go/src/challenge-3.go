package main

import (
    "encoding/hex"
    "fmt"
    "log"
    "strings"
)

func main() {
    input := "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    inputBytes, err := hex.DecodeString(input)
    if err != nil {
        log.Fatal(err)
    }
    bytesToXor := []byte("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    var xoredBytes []([]byte)

    for _, char := range bytesToXor {
        var xoredByCharacter []byte
        for _, b := range inputBytes {
            xoredByCharacter = append(xoredByCharacter, char ^ b)
        }
        xoredBytes = append(xoredBytes, xoredByCharacter)
    }
    m := make(map[string]int)
    for _, entry := range xoredBytes {
        entry := string(entry)
    // ETAOIN SHRDLU
        score := 0
        for _, char := range entry {
            if strings.ToLower(string(char)) == string("e") {
                score += 1 
            } else if strings.ToLower(string(char)) == string("t") {
                score += 1
            } else if strings.ToLower(string(char)) == string("a") {
                score += 1
            } else if strings.ToLower(string(char)) == string("o") {
                score += 1
            } else if strings.ToLower(string(char)) == string("i") {
                score += 1
            } else if strings.ToLower(string(char)) == string("n") {
                score += 1
            } else if strings.ToLower(string(char)) == string("s") {
                score += 1
            } else if strings.ToLower(string(char)) == string("h") {
                score += 1
            } else if strings.ToLower(string(char)) == string("r") {
                score += 1
            }
        }
        m[entry] = score
    }
    var maxScore int
    var maxScoreString string
    for k, v := range m {
        if v > maxScore {
            maxScore = v
            maxScoreString = k
        }
    }
    fmt.Printf("%s", maxScoreString)
}
