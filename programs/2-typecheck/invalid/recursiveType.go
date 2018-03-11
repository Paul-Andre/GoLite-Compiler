// We cannot have a struct reference itself
package main
type T int
func main() {
  type T {
    a T
  }
}
