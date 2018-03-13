// Assigning value of invalid type in struct field
package main

type person struct {
	name string
	age  int
}

func main(){
    var x person
	x.name = 5
}
