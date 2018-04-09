// Might want to rename this file.


// Binary operations

function binary_Or(a,b) {

}
function binary_And(a,b) {

}
function binary_Eq(a,b) {
    return deepEq(a,b)
}
function binary_Neq(a,b) {
    return !deepEq(a,b)
}
function binary_Lt(a,b) {
    return a < b
}
function binary_Leq(a,b) {
    return a <= b
}
function binary_Gt(a,b) {
    return a > b
}
function binary_Geq(a,b) {
    return a >= b
}
function binary_Add(a,b) {
    return a+b
}
function binary_Sub(a,b) {
    return a-b
}
function binary_Mul(a,b) {
    return a*b
}
function binary_Div(a,b) {
    return a/b
}
function binary_BwOr(a,b) {
    return a | b
}
function binary_BwXor(a,b) {
    return a ^ b
}
function binary_Mod(a,b) {
    return (a%b) | 0 // Note: modulo is only performed on ints in Go
}
function binary_BwAnd(a,b) {
    return a & b
}
function binary_BwAndNot(a,b) {
    return a & (~b)
}
function binary_LShift(a,b) {
    return a << b
}
function binary_RShift(a,b) {
    return a >> b
}


function binary_Add_int(a,b) {
    return (a+b) | 0
}
function binary_Sub_int(a,b) {
    return (a-b) | 0
}
function binary_Mul_int(a,b) {
    return Math.imul(a,b)
}
function binary_Div_int(a,b) {
    return (a/b) | 0
}


// Unary Operations
function unary_Plus(a) {
    return a
}
function unary_Neg(a) {
    return -a
}
function unary_BwCompl(a) {
    return ~a
}
function unary_Not(a) {
    return !a
}


// DeepEq

function deepEq(a,b) {
    var type = typeof(a);
    if (type === 'number' || 
        type === 'string' ||
        type === 'boolean') {
        return a === b;
    }
    if (type === 'object') {
        for (field in a) {
            if (!deepEq(a[field], b[field])) {
                return false;
            }
        }
        return true;
    }

    console.log("Error: doing comparison of things that aren't number/string/boolean/object");
    process.exit(1);
}

//------------------------------------------------
// Expressions

function append(a,b) {

}

function check_bounds(a, lenght) {

}

function deepCopy(a) {
    var type = typeof(a);
    if (type === 'number' || 
        type === 'string' ||
        type === 'boolean') {
        return a;
    }
    // Horrible: not differentiating objects from arrays.
    // It will work in our case, but the copied arrays will loose their array properties and access to them might be slow
    if (type === 'object') {
        var b = {};
        for (field in a) {
            if (field === "values") {
                b.values = a.values
            } else {
                b[field] = deepCopy(a[field])
            }
        }
        return b;
    }
}


function makeArray(length, example) {

}

var _;
