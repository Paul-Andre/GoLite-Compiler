package main

var _ int
var _ = 1
var _ float64 = 0.2

type _ int

func _(_,_,_ int) {
  _,_,_ = 1,2,3
  var _,_,_ = 1,2,3
  a,_ := 1,2
  // _,_ := 1,2  // This is technically wrong, but the reference compiler accepts it
}
