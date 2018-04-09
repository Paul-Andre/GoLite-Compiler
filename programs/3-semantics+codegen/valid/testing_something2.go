//~Alice != Carol
//~Bob != Bob2
//~John == John2
//~x == y
//~x == y
//~one == two
//~one != two

package main

type Student struct {
	Name  string
	Score int
}

type arr struct {
	a [5]int
}

var one struct {
	a int
	_ float64
}

var two struct {
	a int
	_ float64
}

func main(){
	var alice Student
	var carol Student
	alice.Name = "Alice"
	carol.Name = "Carol"
	alice.Score = 78
	carol.Score = 92

	if alice == carol {
        println("Alice == Carol")
	} else {
		println("Alice != Carol")
	}

	var bob Student
	bob.Name = "Bob"
	bob.Score = 98
	var bob2 Student
	bob2.Name = "Bob"

	if bob == bob2 {
		println("Bob == Bob2")
	} else {
		println("Bob != Bob2")
	}

	var john Student
	john.Name = "John"
	john.Score = 98
	var john2 Student
	john2.Name = "John"
	john2.Score = 98

	if john == john2 {
		println("John == John2")
	} else {
		println("John != John2")
	}

	var x arr
	var y arr
	if x == y {
		println("x == y")
	} else {
		println("x != y")
	}

	x.a[0] = 65
	y.a[1] = 65

	if x == y {
		println("x == y")
	} else {
		println("x != y")
	}

	one.a = 5
	two.a = 5

	if one == two {
		println("one == two")
	} else {
		println("one != two")
	}

	one.a = 5
	two.a = 6

	if one == two {
		println("one == two")
	} else {
		println("one != two")
	}
}
