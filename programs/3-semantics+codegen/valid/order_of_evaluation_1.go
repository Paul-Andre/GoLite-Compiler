//~lhs1
//~rhs1
//~something
//~call3
//~call4
//~true
//~0
//~0
//~false
//~lhs1
//~call3
//~false
//~7
//~7
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
	var d bool
	
        a, d, d = bar("lhs1") == 2 || bar("rhs1") != 3 || bar("call xxxx") == 1000,
		g != 123421 && bar("something")==4,
		bar("call3") == 1000 || bar("call4") == 1000

        println(a)
	println(b)
	println(c)
	println(d)
		
	a, b, c = bar("lhs1") == 4 && bar("rhs1") == 6, g, bar("call3")

	println(a)
	println(b)
	println(c)
}
