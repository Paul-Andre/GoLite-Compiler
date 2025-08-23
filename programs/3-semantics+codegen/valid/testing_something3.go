//~lhs1
//~rhs1
//~call3
//~false
//~3
//~3
//~lhs1
//~rhs1
//~call3
//~false
//~6
//~6

package main

var g int = 0

func bar(a string) int {
	println(a)
	g++
	return g
}

func main() {
        var a bool

	var b, c int

        a, b, c = bar("lhs1") == 2 || bar("rhs1") == 3, g, bar("call3")

        println(a)
	println(b)
	println(c)
	a, b, c = bar("lhs1") == 4 && bar("rhs1") == 6, g, bar("call3")

	println(a)
	println(b)
	println(c)
}
