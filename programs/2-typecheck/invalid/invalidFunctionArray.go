//
package ting
func f() [2]int {
    var a [2]int
    return a
}

func g() {
    f()[2] = 1
}
