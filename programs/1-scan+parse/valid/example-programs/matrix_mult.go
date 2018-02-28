package main


func main() {

	// Matrices sizes
	///////////////////////////////////
	var Xa = 2

	var Ya = 2
	var Yb = 3

	///////////////////////////////////

	var xRowOne []float32
	var xRowTwo []float32

	xRowOne = append(xRowOne, 1.0)
	xRowOne = append(xRowOne, 2.0)
	xRowOne = append(xRowOne, 3.0)

	xRowTwo = append(xRowTwo, 4.0)
	xRowTwo = append(xRowTwo, 5.0)
	xRowTwo = append(xRowTwo, 6.0)

	var X [][]float32
	X = append(X, xRowOne)
	X = append(X, xRowTwo)

	var yRowOne []float32
	var yRowTwo []float32

	yRowOne = append(yRowOne, 1.1)
	yRowOne = append(yRowOne, 2.2)
	yRowOne = append(yRowOne, 3.3)

	yRowTwo = append(yRowTwo, 4.4)
	yRowTwo = append(yRowTwo, 5.5)
	yRowTwo = append(yRowTwo, 6.6)

	var Y [][]float32
	Y = append(Y, yRowOne)
	Y = append(Y, yRowTwo)


	// Change depending on matrix size!!!
	var transY [3][]float32

	for i := 0; i < Ya; i += 1 {
		for j := 0; j < Yb; j += 1 {
			transY[j] = append(transY[j], Y[i][j])
		}
	}

	// Change depending on matrix size!!!
	var result [2][2]float32

	for i := 0; i < Xa; i += 1 {
		for j := 0; j < Ya; j += 1 {
			if i == 0 {
				var temp [2]float32
				result[i] = temp
			}
			result[i][j] += X[i][j] * transY[j][i]
		}
	}
}