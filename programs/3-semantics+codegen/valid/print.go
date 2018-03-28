//~true false
//~truefalse
//~---------
//~Value before print: 0
//~1 666 1
//~---------
//~Starting index: 0
//~2 0 2 0 2


package main

var glob int = 0
var index int = 0
var arr [5]int

func foo() int {
	glob++
	return 666
}

func bar() int {
	index++
	return 0
}

func main() {
	var a bool = true
	var b bool = false
	println(a, b)
	print(a, b)
	println("")
	println("---------")
	
	println("Value before print:", glob)
	println(glob, foo(), glob)
	println("---------")
	
	arr[0] = 0
	arr[1] = 1
	arr[2] = 2
	arr[3] = 3
	arr[4] = 4
	
	println("Starting index:", index)
	println(index, bar(), index, arr[bar()], index)
}

