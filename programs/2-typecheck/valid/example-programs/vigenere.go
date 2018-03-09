package main
// This file implements the historical Vigenère cipher for encoding text
// https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher

var plaintext =
`Go (often referred to as golang) is a programming language created at Google in two thousand and nine by Robert Griesemer, Rob Pike, and Ken Thompson. It is a compiled, statically typed language in the tradition of Algol and C, with garbage collection, limited structural typing, memory safety features and CSP-style concurrent programming features added. The compiler and other language tools originally developed by Google are all free and open source.$`

var key = "LOLNOGENERICS$"


func main() {
    // I will encode only the letters to be in true spirit of the cipher

    print("The original plaintext is:\n")
    print(plaintext)
    print("\n\n")

    // First, I remove everything but the letters and make them all upper case
    var transformedPlaintext = ""
    for i:=0; plaintext[i] != '$'; i++ {
        var character = plaintext[i]
        if character >= 'a' && character <= 'z' {
            transformedPlaintext += string(character - 'a' + 'A')
        } else if character >= 'A' && character <= 'Z' {
            transformedPlaintext += string(character)
        }
    }
    transformedPlaintext += "$"

    print("The transformed plaintext (only letters) is:\n")
    print(transformedPlaintext)
    print("\n\n")

    var ciphertext = ""
    for i,j := 0,0; transformedPlaintext[i] != '$'; {
        var value = transformedPlaintext[i] - 'A'
        var encodedValue = (value + (key[j] - 'A')) % 26
        ciphertext += string(encodedValue + 'A')
        i++
        j++
        if key[j] == '$' {
            j = 0
        }
    }
    ciphertext += "$"


    print("The ciphertext is:\n")
    print(ciphertext)
    print("\n\n")


    var decoded = ""
    for i,j := 0,0; ciphertext[i] != '$'; {
        var value = ciphertext[i] - 'A'
        var decodedValue = (value + 26 - (key[j] - 'A')) % 26
        decoded += string(decodedValue + 'A')
        i++
        j++
        if key[j] == '$' {
            j = 0
        }
    }

    decoded += "$"

    print("The decoded plaintext is:\n")
    print(decoded)
    print("\n\n")

}
