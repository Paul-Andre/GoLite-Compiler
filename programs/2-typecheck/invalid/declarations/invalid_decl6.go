// invalid declaration: int <- string
package main

func assign() string{
	return "hi"
}
func main(){
	var x int = assign()
}

