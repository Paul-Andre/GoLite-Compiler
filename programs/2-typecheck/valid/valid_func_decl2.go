package main
func f() []int {
    var a []int
    return a
}



func main(){
    f()[1] = 2
	var x = 1
    var a []int
    a[2] = 2
    type t float64
    var b = t(34.0)
    var c []t
    c[2] = b

}

func x(a int) int {
	return a
}

var true bool
func g() {
    type t int
    type u []t
    var a []u
    a[0][0] = t(1)
    true = false
}

func h() {
}
