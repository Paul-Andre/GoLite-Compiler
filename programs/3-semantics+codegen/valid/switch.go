//~RIGHT
//~RIGHT
//~RIGHT
//~RIGHT
//~RIGHT
//~glob is 2

package main

var glob int = 0;

func f() int {
	glob++
	return 0
}

func main() {
	switch true {
		case false: println("WRONG")
		default: println("RIGHT")
	}
	switch true {
		default: println("RIGHT")
		case false: println("WRONG")
	}
	switch 4 {
		case 42: println("The meaning of life and everything")
		case 24: println("gnihtyreve dna efil fo gninaem ehT")
		case 5: println("long")
		default: println("RIGHT")
		case 3: println("short")
    }
	switch 4 {
		case 42: println("The meaning of life and everything")
		case 24: println("gnihtyreve dna efil fo gninaem ehT")
		case 5: println("long")
		default: println("wut")
		case 3: println("short")
		case 4: println("RIGHT")
	}
	switch glob+f() {
		case f(): println("gnihtyreve dna efil fo gninaem ehT")
		case glob: println("wut")
		case 0: println("short")
		case 1: println("RIGHT")
		case 2: println("how")
		case f(): println("gnihtyreve dna efil fo gninaem ehT")
	}
    println("glob is", glob)
}
