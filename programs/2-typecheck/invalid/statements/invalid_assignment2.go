// Invalid types: same names, but different scopes
package ting
type mytype int
var a mytype

func f() {
    type mytype int
    var b mytype = mytype(1)
    a = b
}
