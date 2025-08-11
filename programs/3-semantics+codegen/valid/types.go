//~-2147483648
//~1
//~-2147483648
//~30 0 +0.000000e+000  false
//~0 1 2
//~0 2 2

package main

func main() {

	// integer overflow
	// (Here I use rune instead of int because they are guaranteed to be 32 bit
	{
		var a, b = rune(2147483647), rune(1)
		println(a + b)
	}
	{
		var a, b = rune(2147483647), rune(2147483647)
		println(a * b)
	}
	{
		var a, b = rune(-2147483648), rune(-1)
		println(a / b)
	}

	// Integer division
	print(10 / 3)

	{
		// struct and array initialization

		var a struct {
			a struct {
				a struct {
					a struct {
						a [10]struct {
							a struct {
								a [10]struct {
									a struct {
										a int
										b rune
										c float64
										d string
										e bool
									}
								}
							}
						}
					}
				}
			}
		}

		// Note that the 4th printed value is an empty string
		println(a.a.a.a.a[0].a.a[0].a.a,
			a.a.a.a.a[0].a.a[0].a.b,
			a.a.a.a.a[0].a.a[0].a.c,
			a.a.a.a.a[0].a.a[0].a.d,
			a.a.a.a.a[0].a.a[0].a.e)
	}

	// weird slice behavior
	var a []int
	a = append(a, 0)
	var b = append(a, 1)
	var c = append(a, 2)
	println(a[0], b[1], c[1])
	b = append(b, 900)
	var d = append(b, 1)
	var e = append(b, 2)
	println(a[0], d[3], e[3])

}
