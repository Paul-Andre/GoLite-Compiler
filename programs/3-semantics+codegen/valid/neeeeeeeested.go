//~268435456
//~268435456

package main

var glob int = 2;

func f(a int) int {
	glob+=a
	return glob
}

func main() {
	println(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(f(glob))))))))))))))))))))))))))))
	println(glob)
}
