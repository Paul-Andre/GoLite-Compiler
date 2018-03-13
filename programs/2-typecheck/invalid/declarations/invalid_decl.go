// Invalid assignment: string <- int (testing that var x = "cnst" gives the right type to x)
package main

func main(){
	var x = "hi";
	x = 5;
}
