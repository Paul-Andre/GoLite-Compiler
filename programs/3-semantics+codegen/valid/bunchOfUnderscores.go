// This test is mainly to 
package main

func init() {
}

func init() {
}

type _ struct {}
type _ int

var _ struct {}

func _() {
}
func _(a, b int) {
}


func _(a, b int) struct{ a, b int;} {
  var c struct{a,b int;}
  return c
}

func _(a, b int) struct{ a, b int;} {
  var c struct{a,b int;}
  return c
}

func a(_, _ int) {
	var i struct {
		_ int
		_ float64
	}
	var j struct {
		_,_,_,_,_ int
	}
	var _ struct {
		_,_,_,_,_ int
	}
	var _ struct {
		_,_,_,_,_ int
	}
  var _, _ int
  var main, init int
  {
    var _, _ int
    var main, init int
    {
      var _, _ int
      var main, init int
    }
		_ = 3
		_,_,_,_,_ = 3,4,5,6,7
		_,_,_,_,_,i := 3,4,5,6,7,8
		_ = i
		var _,_,_,_ = 1,2,3,4
  }
}

func f(init, main int) {
}
var done = false
func main() {
	if !done {
		done = true
		main()
	}
}
