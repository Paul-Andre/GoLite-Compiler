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
    if (b == 0) {
        console.error("Error: division by zero.");
        exit(1);
    }
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


function print_float(a) {
    if (Number.isFinite(a)) {
        let out = a.toExponential(6);
        if (out[0] !== '-') {
            out = "+"+out;
        }
        let beforeExponent = "";
        let exponent = "0";
        for (let i=1; i<out.length; i++) {
            if (out[i] === '-' || out[i] === '+') {
                exponent = out.substring(i+1);
                beforeExponent = out.substring(0,i+1);
                break;
            }
        }
        while (exponent.length < 3) {
            exponent = "0" + exponent;
        }
        process.stdout.write(beforeExponent + exponent);
    } else if (Number.isNaN(a)) {
        process.stdout.write("NaN");
    } else if (a == Infinity) {
        process.stdout.write("+Inf");
    } else if (a == -Infinity) {
        process.stdout.write("-Inf");
    } else {
        console.error("Error: Trying to print out something that isn't a float as a float.");
        exit(1);
    }
}

function print_not_float(a) {
    // TODO: check if this needs further splitting
    process.stdout.write(""+a);
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
        for (let field in a) {
            if (!deepEq(a[field], b[field])) {
                return false;
            }
        }
        return true;
    }

    console.error("Error: doing comparison of things that aren't number/string/boolean/object");
    process.exit(1);
}

//------------------------------------------------
// Expressions

function append(slice, object) {
    let ret = {
        length: slice.length,
        capacity: slice.capacity, // I "simulate" the capacity
        contents: slice.contents,
    };
    if (ret.length+1 > ret.capacity) {
        ret.contents = deepCopy(ret.contents);
        ret.capacity = ret.capacity? ret.capacity * 2: 1;
        while (ret.contents.length < ret.capacity) {
            ret.contents.push(undefined);
        }
    }
    ret.contents[ret.length] = deepCopy(object);
    ret.length++;
    return ret;
}

function check_bounds(a, length, line_number) {
    if (a < 0) {
        console.error("Error: line "+line_number+": trying to index an array or slice with negative number.");
        process.exit(1);
    }
    if (a >= length) {
        console.error("Error: line "+line_number+": index " + a +" out of range. Should be in range of "+length);
        process.exit(1);
    }
    return a;
}

function deepCopy(a) {
    let b;
    const type = typeof(a);
    if (type === 'number' || 
            type === 'string' ||
            type === 'boolean') {
        return a;
    }
    if (type === 'object') {
        if (Array.isArray(a)) {
            b = [];
            for (let i=0; i<a.length; i++) {
                b.push(deepCopy(a[i]));
            }
            return b;
        } else {
            b = {};
            for (let field in a) {
                if (field === "contents") {
                    b.contents = a.contents
                } else {
                    b[field] = deepCopy(a[field])
                }
            }
            return b;
        }
    }
}


function makeArray(length, example) {
    let ret = [];
    for (let i=0; i<length; i++) {
        ret.push(deepCopy(example));
    }
    return ret;
}

var _;


//============================== END OF HEADER ================================//


