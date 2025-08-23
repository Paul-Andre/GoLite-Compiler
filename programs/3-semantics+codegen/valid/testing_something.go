//~hello world\nC 98
//~true
//~false
//~true false true true false

package main

func main() {

	var a string = "hello "
	var b string = `world\n`
	var d = rune(67)

	var c rune = 'b'

	println(a+b+string(d), c)
	println("h" <= a)
	println("i" <= a)
	println("alex" > "^lex", "^lex" < "Alex", "Blex" < "alex", "alex" <= "alex", "alex " <= "alex")
}
