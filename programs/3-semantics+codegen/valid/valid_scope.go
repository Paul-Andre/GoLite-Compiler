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
	print(v)

	// Displays shadowing
	v := 2  // short variable declaration
	print(v)

	// Displays more complex shadowing
	c := "outer"
	print(c)
	{
		c := "inner"
		print(c)
		{
			print(c)
		}
	}
	{
		print(c)
	}
	print(c)

	if x:=10; true {
		print(x)
	}

	// Throws error!
	//print(x)

}


