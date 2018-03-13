// Invalid declaration: assigning an int to an int_type
package main

func main() {
	type int_type int
	var y int_type = 1
}

