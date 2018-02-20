// Tests a bunch of for loops
package main
func main() {
  for  {
  }
  for true {
  }
  for a(); b(); c() {
  }
  for i:=0; i<10; i++ {
  }
  for i:=0; i<10; {
  }
  for ; i<10; i++ {
  }
  for ; i<10; {
  }


  // And as from the specs:

  if x > max {
    x = max
  }


  if x := f(); x < y {
    return x
  } else if x > z {
    return z
  } else {
    return y
  }

}
