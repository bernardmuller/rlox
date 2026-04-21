# Lox

This repository is for educatinoal purposes and is my implementation of the [Lox](https://github.com/munificent/craftinginterpreters) scripting language.
Lox is a tiny scripting language described in [Bob Nystrom](https://stuffwithstuff.com/)'s book [Crafting Interpreters](https://craftinginterpreters.com/).

## Lox Language

A few example programs from [Crafting Interpreters](https://craftinginterpreters.com/the-lox-language.html):

```lox
print "Hello, world!";
```

If statements:

```lox
if (condition) {
  print "yes";
} else {
  print "no";
}
```

Loops:

```lox
var a = 1;
while (a < 10) {
  print a;
  a = a + 1;
}

for (var a = 1; a < 10; a = a + 1) {
  print a;
}
```

Functions and closures:

```lox
fun printSum(a, b) {
  print a + b;
}
printSum(1, 2);

fun returnFunction() {
  var outside = "outside";

  fun inner() {
    print outside;
  }

  return inner;
}

var fn = returnFunction();
fn();
```

Classes:

```lox
class Breakfast {
  init(meat, bread) {
    this.meat = meat;
    this.bread = bread;
  }
}

class Brunch < Breakfast {
  init(meat, bread, drink) {
    super.init(meat, bread);
    this.drink = drink;
  }
}
```
