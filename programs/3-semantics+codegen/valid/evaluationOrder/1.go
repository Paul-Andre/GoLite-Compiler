//~c c a
package main

var a []string;
var b []string;

func f() []int {

  a = b 
  var x []int
  x = append(x,0)
  return x

}


func main() {
  a = append(a, "a")
  b = append(b, "b")

  var prevA = a

  a[0], f()[0] = "c", 0
  println(a[0],b[0],prevA[0])
}

