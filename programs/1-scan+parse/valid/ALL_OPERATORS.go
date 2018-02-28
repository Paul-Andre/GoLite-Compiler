package ting

func f() {
    var x,y int
    x = x+y
    x = x-y
    x = x*y
    x = x/y
    x = x%y
    x = x&y
    x = x|y
    x = x^y
    x = x<<y
    x = x>>y
    x = x&^y

    x += y
    x -= x +y
    x *= x +y
    x /= x +y
    x %= x +y

    x &= x +y
    x |= x +y
    x ^= x +y
    x <<= x +y
    x >>= x +y
    x &^= x +y

    x = x&&y
    x = x||y
    x++
    x--

    x = x==y
    x = x<y
    x = x>y
    x = !y
    x = x!=y
    x = x <= y
    x = x >= y
    x := x +y
    x = (x+y)*x
}
