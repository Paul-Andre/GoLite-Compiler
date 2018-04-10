package main

// a (pretty bad) random number generator
var rngState int = 2134123413

func randomInt() int {
	rngState *= 79
	rngState >>= 1
	rngState &= 0x0fffffff
	return rngState % 100000
}

// Does quicksort on the section of the array between begin and end inclusively
// Assumes that both bounds are inclusive
func quicksort(array []int, begin, end int) {
	if end <= begin {
		return
	}
	// Select the pivot to be the
	var pivot = array[begin]
	var i = begin + 1
	var j = end
	for i < j {
		for i < j && array[i] <= pivot {
			i++
		}

		for i < j && array[j] > pivot {
			j--
		}

		array[i], array[j] = array[j], array[i]
	}
	if array[i] > pivot {
		i--
	}
	array[begin], array[i] = array[i], array[begin]

	quicksort(array, begin, i-1)
	quicksort(array, i+1, end)
}

func printArray(a []int, length int) {
	for i := 0; i < length; i++ {
		print(a[i], " ")
	}
}

func bench(length int, doPrint bool, name string) {
	var a []int
	for i := 0; i < length; i++ {
		a = append(a, randomInt())
	}
	print("Done initializing ", name, " ")
	if doPrint {
		printArray(a, length)
	}

	println()

	quicksort(a, 0, length-1)

	print("Done sorting ", name, " ")
	if doPrint {
		printArray(a, length)
	}
	println()

}

func main() {
	bench(10, true, "tiny array ")
	bench(100, true, "small array")
	bench(1000000, false, "big array")
	bench(10000000, false, "huge array")
}
