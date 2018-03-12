package main

type person struct {
	name string
	age  int
}


func main(){
  // We don't support struct literals and without that this test is pointless
  x := person{5, 5}
}



