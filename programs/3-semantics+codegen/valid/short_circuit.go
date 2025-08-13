//~a = false  b = false
//~Executing: a
//~a && b = false
//~-----
//~a = false  b = true
//~Executing: a
//~a && b = false
//~-----
//~a = true  b = false
//~Executing: a, b
//~a && b = false
//~-----
//~a = true  b = true
//~Executing: a, b
//~a && b = true
//~-----
//~a = false  b = false
//~Executing: a, b
//~a || b = false
//~-----
//~a = false  b = true
//~Executing: a, b
//~a || b = true
//~-----
//~a = true  b = false
//~Executing: a
//~a || b = true
//~-----
//~a = true  b = true
//~Executing: a
//~a || b = true
//~-----
package main

func shout(s string, b bool) bool {
	print(s)
	return b
}

func run_and(a bool, b bool) {
	println("a =", a, " b =", b)
	print("Executing: ")
	if shout("a", a) && shout(", b", b) {
		println()
		println("a && b = true")
	} else {
		println()
		println("a && b = false")
	}
	println("-----")
}

func run_or(a bool, b bool) {
	println("a =", a, " b =", b)
	print("Executing: ")
	if shout("a", a) || shout(", b", b) {
		println()
		println("a || b = true")
	} else {
		println()
		println("a || b = false")
	}
	println("-----")
}

func main() {
	run_and(false, false)
	run_and(false, true)
	run_and(true, false)
	run_and(true, true)

	run_or(false, false)
	run_or(false, true)
	run_or(true, false)
	run_or(true, true)
}
