package main
// This programs procedurally generates a cave

// First thing first, a (pretty bad) random number generator
var rngState int =  2134123413

func randomBit() bool {
    rngState *= 79
    rngState >>= 1
    rngState &= 0x8fffffff
    return rngState % 2 == 0
}

func randomFloat() float64 {
    var acc = 0
    var div = 0
    for i:=0; i<31; i++ {
        if randomBit() {
            acc |= 1
        }
        div |= 1
        acc <<= 1
        div <<= 1
    }
    return float64(acc) / float64(div)
}

var width = 60
var height = 60
var cave [60][60]bool
var tmp [60][60]bool

var directNeighborDirections = 4
var dnx [4]int
var dny [4]int

var indirectNeighborDirections = 4
var inx [4]int
var iny [4]int

func printCave() {
    for j:=0; j<height; j++ {
        for i := 0; i<width; i++ {
            if cave[i][j] {
                print("##")
            } else {
                print("  ")
            }
        }
        print("\n")
    }
}

func main() {

    dnx[0], dny[0] = 0, 1
    dnx[1], dny[1] = 1, 0
    dnx[2], dny[2] = 0, -1
    dnx[3], dny[3] = -1, 0

    inx[0], iny[0] = 1, 1
    inx[1], iny[1] = 1, -1
    inx[2], iny[2] = -1, 1
    inx[3], iny[3] = -1, -1

    for j:=0; j<height; j++ {
        for i:=0; i<width; i++ {
            cave[i][j] = randomFloat() > 0.51
        }
    }

    for times:=0; times<5; times++ {
        for j:=0; j<height; j++ {
            for i:=0; i<width; i++ {
                tmp[i][j] = cave[i][j]
            }
        }

        for j:=0; j<height; j++ {
            for i:=0; i<width; i++ {
                var neighborCount = 0.
                for k:=0; k<directNeighborDirections; k++ {
                    var x = i + dnx[k]
                    var y = j + dny[k]
                    if x < 0 || x >= width || y < 0 || y >= height || tmp[x][y] {
                        neighborCount+=1.
                    }
                }
                for k:=0; k<indirectNeighborDirections; k++ {
                    var x = i + inx[k]
                    var y = j + iny[k]
                    if x < 0 || x >= width || y < 0 || y >= height || tmp[x][y] {
                        neighborCount+=0.8
                    }
                }
                if (tmp[i][j]) {
                    cave[i][j] = neighborCount > 3.
                } else {
                    cave[i][j] = neighborCount > 4.
                }

            }
        }
    }

    printCave()

}

