//~lhs1
//~call3
//~rhs1
//~true
//~3
//~2
//~lhs1
//~call3
//~rhs1
//~true
//~6
//~5

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
