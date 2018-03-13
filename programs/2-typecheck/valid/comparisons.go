// Tests a bunch of cases in which comparing two values is legal
package main

func main() {
  {
    var a, b int
    _ = a == b
    _ = a < b
  }
  {
    var a, b rune
    _ = a == b
    _ = a < b
  }
  {
    var a, b float64
    _ = a == b
    _ = a < b
  }
  {
    var a, b string
    _ = a == b
    _ = a < b
  }
  {
    var a, b struct {
      a int
      b rune
      c float64
      d string
    }
    _ = a == b
  }
  {
    var a, b [123]int
    _ = a == b
  }
  {
    type K int
    type T struct {
      a int
      b rune
      c float64
      d string
      e K
      f [123]float64
    }
    var a, b [123]T
    _ = a == b
  }
}
