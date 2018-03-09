// Does a basic DFS to find some node with some value
package main
var tree [7][]int
var values [7]int

func dfs(r int, k int) int {
    if values[r] == k {
        return 1
    }
    for i := 0; i<len(tree[r]); i++ {
        if 1 == dfs(tree[r][i], k) {
            return 1
        }
    }
    return 0
}

func main() {

    tree[0] = append(tree[0], 1)
    tree[1] = append(tree[1], 2)
    tree[1] = append(tree[1], 3)
    tree[2] = append(tree[2], 4)
    tree[3] = append(tree[3], 5)
    tree[4] = append(tree[4], 6)
    values[5] = 23

    if(dfs(0, 23) == 1) {
        println("Value was found :)")
    }
}
