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

func _(_, _ int) {
  var _, _ int
  var main, init int
  {
    var _, _ int
    var main, init int
    {
      var _, _ int
      var main, init int
    }
  }
}

func f(init, main int) {
}

func main() {
  main()
}
