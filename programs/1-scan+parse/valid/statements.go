// a bunch of allowed statements
package main

func main() {
  a++
  a--

  ; // some random empty
  ;

  f() // expression statement
  print()
  println()

  f(1,2) // expression statement
  print(1,2)
  println(1,2)

  {

  }
  {}
  {
    f()
  } // some random blocks

  // Some random assignments
  _ = a
  _,_,_,_ = 1,1,1,1
  a,b,c = b,a,c
  (a), b.c, d[a+b.c] = 1,2,3

  // Some random short declarations
  d,_,_ := b, a, c
  f,t,g := 1,2,3

  // Some random variable declarations
  var f, g = 1, 2

  var k = append(k, 2)

  // Some random type declarations
  type k int

  type (
    k int
    t int
  )

  // some random return statements
  return
  return 1
  return 2 + 2 == 4
}

