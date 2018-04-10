//~1 2
//~2 1
//~1 2 123

package main

func main() {
	var a, b = 1, 2
	println(a, b)
	a, b = b, a
	println(a, b)
	a, b, c := b, a, 123
	println(a,b,c)

	/*
	println()
	
	for i, j := 0, 0; i < 10; i, j = j, i {
		println(i, j)
		i++
	}
	*/
}
