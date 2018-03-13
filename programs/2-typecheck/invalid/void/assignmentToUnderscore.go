// We should not be able to assign the "result" of a void function to anything, not even _
package main

func f() {
}

var _ = f()
