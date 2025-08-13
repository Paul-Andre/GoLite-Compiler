//~inter 0
//~1inc_glob1
//~inter 1
//~2 inc_glob 2
//~inter 2
//~3 inc_glob 3
//~inter 3
//~4
//~inter 4
//~5
//~inter 5
//~6
//~inter 6
//~7
//~inter 7
//~9
//~inter 9
//~11 inc_glob 11
//~inter 11
//~12 inc_glob 13
//~inter 13
//~15 inc_glob 15
//~inter 15
//~16 inc_glob 17
//~inter 17

package main

var glob = 0

func inc_glob() string {
	glob++
	return "inc_glob"
}

func inc_glob_zero() int {
	glob++
	return 0
}

func inc_glob_one() int {
	glob++
	return 1
}

func custom_print(a int, b string, c int) {
	println(a, b, c)
}

func add(a int, b int) int {
	return a + b
}

func main() {
	println("inter", glob)

	print(glob, inc_glob(), glob)
	println()

	println("inter", glob)

	println(glob, inc_glob(), glob)

	println("inter", glob)

	custom_print(glob, inc_glob(), glob)

	println("inter", glob)

	var a = glob + inc_glob_zero()
	println(a)

	println("inter", glob)

	var b = inc_glob_zero() + glob
	println(b)

	println("inter", glob)

	println(glob + inc_glob_zero())

	println("inter", glob)

	println(inc_glob_zero() + glob)

	println("inter", glob)

	glob += inc_glob_one()
	println(glob)

	println("inter", glob)

	println(glob+inc_glob_zero(), inc_glob(), glob)

	println("inter", glob)

	println(add(glob, inc_glob_zero()), inc_glob(), glob)

	println("inter", glob)

	custom_print(glob+inc_glob_zero(), inc_glob(), glob)

	println("inter", glob)

	custom_print(add(glob, inc_glob_zero()), inc_glob(), glob)

	println("inter", glob)
}
