// There must be a variable that is currently not in scope in shortVarDecl
package main
func f() {
  a, b, c := 1,2,3

  _,a,b,_ := 4,5,6,7
}


