// In Golite, we cannot have a type that refers to itself in the form of slice,
// even though in Go we can
package main
type T a
func main() {
  type T struct {
    a []T
  }
}
