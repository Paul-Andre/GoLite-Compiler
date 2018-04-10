package main

type person struct {
        age  int
    }
var rngState int = 2134123413

func randomPerson() person {
	rngState *= 79
	rngState >>= 1
	rngState &= 0x0fffffff
	age := rngState % 100000

	var p person
	person.age = age

	return p
}

func swap(a []person, i, j int) {
	tmp := a[j]
	a[j] = a[i]
	a[i] = tmp
}


func bubblesort(a []person, length int) {
	swapped := true;
    	for swapped {
    		swapped = false
    		for i := 0; i < length - 1; i++ {
    			if a[i + 1].age < a[i].age {
    				swap(a, i, i + 1)
    				swapped = true
    			}
    		}
    }
}

func printArray(a []person, length int) {
	for i := 0; i < length; i++ {
		print(a[i].age, " ")
	}
}

func bench(length int, doPrint bool, name string) {
	var a []person

	for i := 0; i < length; i++ {
		a = append(a, randomPerson())
	}

	print("Done initializing ", name, " ")

	if doPrint {
		printArray(a, length)
	}

	println()

	bubblesort(a, length)

	print("Done sorting ", name, " ")
	if doPrint {
		printArray(a, length)
	}
	println()
}

func main() {
	bench(100000, false, "big person array")
}
