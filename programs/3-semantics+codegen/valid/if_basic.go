//~1
//~2
//~3
//~4
//~7
//~8
//~13
//~14
//~15
//~16
//~17
//~18
//~a prior 0
//~a first if 10
//~a second else 20
package main

func main() {
  println(1);
  println(2);
  if true {
    println(3);
    println(4);
  }
  if false {
    println(5);
    println(6);
  }

  if true {
    println(7);
    println(8);
  } else {
    println(9);
    println(10);
  }

  if false {
    println(11);
    println(12);
  } else {
    println(13);
    println(14);
  }

  println(15);
  println(16);

  {
    println(17);
    println(18);
  }

  var a int
  println("a prior", a)

  if a=10; true {
    println("a first if", a);
  } else {
    println("a first else", a);
  }

  if a=20; false {
    println("a second if", a);
  } else {
    println("a second else", a);
  }

}
