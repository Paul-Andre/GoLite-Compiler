// init does not introduce a binding and thus cannot be called
package ting
func init() {}
func f() {
    init();
}
