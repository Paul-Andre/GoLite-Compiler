// It should be illegal to do anything with the "return value" of void
// functions, not even compare it to another void
package main

func f() {
}

var _ = f() == f()
