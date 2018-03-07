package ting

// Basic
func f() int {
    return 1
}

// return inside block
func g() int {
    {
        return 1
    }
}

// Infinite loop
func h() int {
    for ;; {
        print("hi");
    }
}

// Simple if
func i() int {
    if true {
        print("hi");
        return 2
    } else {
        return 2
    }
}

// Complex if
func j() int {
    if 1 == 4 {
        print("hi");
        return 2
    } else if 2 == 4 {
        print("ho");
        return 2
    } else if 3 == 4 {
        {
            print("ha");
            return 2
        }
    } else {
        print("hu");
        {
            return 2
        }
    }
}

// Simple Switch
func k() int {
    switch 5 {
        case 4 :
            print("hi");
            return 2;
        default:
            return 2;
    }
}

// complex Switch
func l() int {
    switch 5 {
        case 4 :
            print("hi");
            {
                return 2;
            }
        case 2 :
            {
                print("hi");
                return 2;
            }
        case 3 :
            if true {
                print("hi");
                for ; ; {
                    break
                }
                return 2
            } else {
                return 2
            }
        case 1 :
            print("hi");
            return 2;
        default:
            return 2;
    }
}
