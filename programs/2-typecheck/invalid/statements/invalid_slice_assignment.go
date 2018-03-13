// Mismatched assignment operands (int <- string)
package main

func main(){
	var x []int

	x = append(x, 5)
	x[0] = "hi"
}

