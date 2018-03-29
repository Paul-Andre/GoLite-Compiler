//~c c a
package main

var a []string;
var b []string;

func f() int {

	a = b	
	return 0
	
	}


func main() {
	a = append(a, "a")
	b = append(b, "b")
	
	var prevA = a
	
	a[f()] = "c"
	println(a[0],b[0],prevA[0])
}
