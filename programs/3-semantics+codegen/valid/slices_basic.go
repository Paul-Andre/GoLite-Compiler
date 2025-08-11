//~0
//~0 1
//~0 1 2
//~0 5 2
package main

func main() {
    var a []int
		a = append(a, 0)
		println(a[0])
		a = append(a, 1)
		println(a[0], a[1])
		a = append(a, 2)
		println(a[0], a[1], a[2])
		a[1] = 5
		println(a[0], a[1], a[2])
}
