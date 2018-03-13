// Mismatched types: int <- string (in short var declaration)
package main

func main(){
	var x int
	x, y, z := "hi", 5, 5
}
