import random

binaryOperators = [
"+",
"-",
"*",
"/ ",
"% ",
"& ",
"|",
"^ ",
"&^",
]

unarayOperators = [
"+",
"-",
"^",
"",
"",
"",
]

print("package main")
print("func main() {")

for _ in range(100):
    print("\t{");
    print("\t\tprintln(");

    for _ in range(10):
        print("\t\t\t"+random.choice(unarayOperators)+str(random.randint(0,100000))+" "+random.choice(binaryOperators))

    print("\t\t\t"+random.choice(unarayOperators)+str(random.randint(0,100000)) + ")" )

    print("\t}");

print("}")
