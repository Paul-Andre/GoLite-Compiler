// Identifier Type, No Initialization
package main;

var i int
var ii int;

var U,V,W float64
var UU,VV,WW float64;

var (
   k int
   a,b float64
)
var (
   kk int;
   aa,bb float64;
)

var i []int

var j [][123]int

type T struct {
	int k
}

var j T

var k struct {
	x int
	y, z int
	k []struct {
		b [123]float32
		
	}
	l [](struct {
		b []T
	})


}
