//~1
//~2
//~outer
//~inner
//~inner
//~outer
//~outer
//~10

package main

var v = 1

func main() {

	// Displays variables being able to reach within scopes of functions
	println(v)

	// Displays shadowing
	v := 2  // short variable declaration
	println(v)

	// Displays more complex shadowing
	c := "outer"
	println(c)
	{
		c := "inner"
		println(c)
		{
			println(c)
		}
	}
	{
		println(c)
	}
	println(c)

	if x:=10; true {
		println(x)
	}

	// Throws error!
	//print(x)

}


