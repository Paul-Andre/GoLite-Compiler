// We cannot use a function as a value in Golite
package main
func f() {
}
func main() {
  _ = f
}
