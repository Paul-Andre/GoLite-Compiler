// Invalid recursive type
package ting
type t int
func main() {
    type t t
}
