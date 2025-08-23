//~running: a b c d e
package main

func shout(s string, b bool) bool {
  print(s)
  return b
}

func main() {
  print("running:")
  _,_,_ = shout(" a", true) && shout(" b", true), shout(" c", false) || shout(" d", true) , shout(" e", true)
  println()
}
