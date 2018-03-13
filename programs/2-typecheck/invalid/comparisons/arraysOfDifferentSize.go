// If two arrays are of different size, they are considered to not be of identical type, so cannot be compared
package main

var a [10]int
var b [9]int
var _ = a == b
