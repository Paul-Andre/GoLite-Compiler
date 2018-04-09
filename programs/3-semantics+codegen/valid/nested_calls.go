//~0 0
//~2 1 2 6
//~5 4 14 3 2


package main

var globf int = 0;
var globh int = 0;

func f() int {
	globf++
	return 2
}

func h(a int) int {
	globh++
	return a*3
}

func i() []int {
	var ting []int
	ting = append(ting, 0)
	ting = append(ting, 1)
	ting = append(ting, 2)
	ting = append(ting, 3)
	ting = append(ting, 4)
	ting = append(ting, 5)
	ting = append(ting, 6)
	ting = append(ting, 7)
	return ting
}

func main() {
	println(globf, globh)
	println(globf, globh, f(), h(f()))
	println(globf, globh, h(globf) + globf + f() + i()[h(f()) - globf], h(1), f())
}
