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


function print_float(a) {
    var out = a.toExponential(6);
    if (out[0] != '-') {
        out = "+"+out;
    }
    var beforeExponent = "";
    var exponent = "0";
    for (var i=1; i<out.length; i++) {
        if (out[i] == '-' || out[i] == '+') {
            exponent = out.substring(i+1);
            beforeExponent = out.substring(0,i+1);
            break;
        }
    }
    while (exponent.length < 3) {
        exponent = "0" + exponent;
    }
    process.stdout.write(beforeExponent + exponent);
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
        for (field in a) {
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
        capacity: slice.capactity, // I "simulate" the capacity
        contents: slice.contents,
    };
    if (ret.length+1 > ret.capacity) {
        ret.contents = deepCopy(ret.contents);
        ret.capacity = (ret.capacity + 1) * 2;
    }
    ret.contents.push(deepCopy(object));
    ret.size++;
    return ret;
}

function check_bounds(a, length, line_number) {
    if (a < 0) {
        console.error("Error: line "+line_number+": trying to index an array or slice with negative number.");
        process.exit(1);
    }
    if (a > length) {
        console.error("Error: line "+line_number+": index out of range.");
        process.exit(1);
    }
    return a;
}

function deepCopy(a) {
    var type = typeof(a);
    if (type === 'number' || 
            type === 'string' ||
            type === 'boolean') {
        return a;
    }
    if (type === 'object') {
        if (Array.isArray(a)) {
            var b = [];
            for (var i=0; i<a.length; i++) {
                b.push(deepCopy(a[i]));
            }
            return b;
        } else {
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
}


function makeArray(length, example) {
    let ret = [];
    for (var i=0; i<length; i++) {
        ret.push(deepCopy(example));
    }
    return ret;
}

var _;


//============================== END OF HEADER ================================//



function f·2 ( a·3 ) {
}
function main (  ) {
	var ⴵ_27 = f·2(deepCopy(glob·1), );
	var ⴵ_26 = f·2(deepCopy(ⴵ_27), );
	var ⴵ_25 = f·2(deepCopy(ⴵ_26), );
	var ⴵ_24 = f·2(deepCopy(ⴵ_25), );
	var ⴵ_23 = f·2(deepCopy(ⴵ_24), );
	var ⴵ_22 = f·2(deepCopy(ⴵ_23), );
	var ⴵ_21 = f·2(deepCopy(ⴵ_22), );
	var ⴵ_20 = f·2(deepCopy(ⴵ_21), );
	var ⴵ_19 = f·2(deepCopy(ⴵ_20), );
	var ⴵ_18 = f·2(deepCopy(ⴵ_19), );
	var ⴵ_17 = f·2(deepCopy(ⴵ_18), );
	var ⴵ_16 = f·2(deepCopy(ⴵ_17), );
	var ⴵ_15 = f·2(deepCopy(ⴵ_16), );
	var ⴵ_14 = f·2(deepCopy(ⴵ_15), );
	var ⴵ_13 = f·2(deepCopy(ⴵ_14), );
	var ⴵ_12 = f·2(deepCopy(ⴵ_13), );
	var ⴵ_11 = f·2(deepCopy(ⴵ_12), );
	var ⴵ_10 = f·2(deepCopy(ⴵ_11), );
	var ⴵ_9 = f·2(deepCopy(ⴵ_10), );
	var ⴵ_8 = f·2(deepCopy(ⴵ_9), );
	var ⴵ_7 = f·2(deepCopy(ⴵ_8), );
	var ⴵ_6 = f·2(deepCopy(ⴵ_7), );
	var ⴵ_5 = f·2(deepCopy(ⴵ_6), );
	var ⴵ_4 = f·2(deepCopy(ⴵ_5), );
	var ⴵ_3 = f·2(deepCopy(ⴵ_4), );
	var ⴵ_2 = f·2(deepCopy(ⴵ_3), );
	var ⴵ_1 = f·2(deepCopy(ⴵ_2), );
	print_not_float(ⴵ_1);
	print_not_float("\n");

	print_not_float(glob·1);
	print_not_float("\n");

}
main();
