// Pass by reference of structures
//~0 123
//~0 123
//~0
//~743 634
package main

type S struct {
	a int
}

var global1, global2 S

func mutateParameter(s S) {
	s.a = 10231
}

func passGlobal() S {
	return global1
}

func main() {
	{
		var local1, local2 S
		local1 = local2
		local2.a = 123
		println(local1.a, local2.a)
	}
	{
		var local1, local2 S
		local2 = local1
		local2.a = 123
		println(local1.a, local2.a)
	}
	{
		var local1 S
		mutateParameter(local1)
		println(local1.a)
	}
	{
		global1.a = 743
		var g = passGlobal()
		g.a = 634
		println(global1.a,g.a)
	}
}

