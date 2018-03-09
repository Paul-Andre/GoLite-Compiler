// prints a shitty imitation of pascal's triangle 
package main

func main() {
    var a,n = 1,2
    var b bool
    for {
        b = a > 10000000000000000

        switch b{
            case true: println("Done...")
                                    return;
        }
        println(a)
        a = pascal(n, a)
        //println(a)
        n++
    }
}

func pascal(n int, a int) int {
    if n == 1 {
        return 11;
    } else {
    }
    var k = 1
    var r int
    for k<=n {
        r += pow(10,n-k)*coeff(n,k,a)
        k++
    }
    //r += 1
    return r;
}

func coeff(n int, k int, a int) int {
    /*
    print(n)
    print(" ")
    print(k)
    print(" ")
    print(a)
    print(" ")
    */
    if k == 1 {
        return 1
    }
    if n < k {
        println("Error computing binomial coefficient");
        return 0;
    }
    r := ((a/pow(10,k-1))%10) + ((a/pow(10,k)%10))
    //print(r)
    return r
}

// why care about being fast?
func pow(x int, y int) int {
    if y == 0 {
        return 1
    }
    return x*pow(x,y-1)
}
