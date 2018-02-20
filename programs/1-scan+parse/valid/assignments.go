package main
func main() {
  // Some random examples I took from the spec removing what we don't have
  x = 1
  p = f()
  a[i] = 23
  (k) = ch  // same as: k = ch

  a[i] <<= 2
  i &^= 1<<n

  _ = x       // evaluate x but ignore it
  x, _ = f(), 0


  a, b = b, a  // exchange a and b

  var x []int
  x = append(x,1)
  x = append(x,2)
  x = append(x,3)

  i := 0
  i, x[i] = 1, 2  // set i = 1, x[0] = 2

  i = 0
  x[i], i = 2, 1  // set x[0] = 2, i = 1

  x[0], x[0] = 1, 2  // set x[0] = 1, then x[0] = 2 (so x[0] == 2 at end)

  x[1], x[3] = 4, 5  // set x[1] = 4, then panic setting x[3] = 5.

  type Point struct { x, y int; }

  var p Point
  x[2], p.x = 6, 7

  i = 2
}
