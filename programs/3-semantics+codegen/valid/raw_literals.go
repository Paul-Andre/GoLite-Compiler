//~7
//~8
//~12
//~10
//~13
//~9
//~11
//~92
//~39
//~Interpreted (except f): ^G^H^K^M
//~    \" END
//~Raw: \a\b\f\n\r\t\v\\\'\" END
//~^G ^H
//~ ^M      ^K \ '
//~^? 127

package main
func main() {
//Runes
    println('\a')
    println('\b')
    println('\f')
    println('\n')
    println('\r')
    println('\t')
    println('\v')
    println('\\')
    println('\'')
//Strings
    println("Interpreted (except f): \a\b\v\r\n\t\\\" END")
    println(`Raw: \a\b\f\n\r\t\v\\\'\" END`)
//Casting runes to strings
    var t1 = '\a'
	var t2 = '\b'
	var t3 = '\n'
	var t4 = '\r'
	var t5 = '\t'
	var t6 = '\v'
	var t7 = '\\'
	var t8 = '\''
	println(string(t1),string(t2),string(t3),string(t4),
		    string(t5),string(t6),string(t7),string(t8))
	println(string(127), 127)
}

