//
package ting
type a struct { b int; }

func h() a {
    var x a
    return x
}

func main() {
    h().b = 1
}
