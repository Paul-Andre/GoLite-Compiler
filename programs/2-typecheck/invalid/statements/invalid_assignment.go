package main

func main(){
	type x int
	type y x

	var z y = y(5)

	z = 11
}






