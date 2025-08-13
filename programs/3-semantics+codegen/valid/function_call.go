//~hello from foo
//~hello from bar 1337
//~hello from baz
//~baz returned 1000
package main

func foo() {
  println("hello from foo");
}

func bar(a int) {
  println("hello from bar", a);
}

func baz() int {
  println("hello from baz");
  return 1000
}

func main() {
  foo();
  bar(1337);
  var a = baz();
  println("baz returned", a);
}
