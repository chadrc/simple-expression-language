# Simple Expression Language

## Literals
```
// Numbers
4
29
3.14

// String
"Hello"
'SEL'
`The quick brown fox jumped over the lazy dog.`

// Boolean
true
false

// Ranges
5..10 // Exclusive, contains 5, 6, 7, 8, and 9
5...10 // Inclusive, contains 5, 6, 7, 8, 9 and 10
```

## Math operations
```
// Addition
10 + 5

// Subtraction
10 - 5

// Multiplication
10 * 5

// Division
10 / 5

// Remainder
10 % 5

// Exponential
2^10

// Negation
-5
```

## String operations
```
// Concatination
"Hello, " + "World"
```

## Logical operations
```
// Equality
5 = 4

// Inequality
5 != 4

// Less than
5 < 4

// Less than or equal
5 <= 4

// Greater than
5 > 4

// Greater than or equal
5 >= 4

// Logical AND
5 = 4 && 5 < 10

// Logical OR
5 = 4 || 5 < 10

// Logical NOT
!true
```

## Exposed References
If runtime exposes a variable or function it can be referenced just by its name.

Ex. exposed variable `name` = "SEL"
```
"Hello, " + name
// result "Hello, SEL"
```

Ex. exposed function `rand` that takes no parameters
```
rand()
// result is a randomly generated number
```

### Associative Array
A single collection type is provided being a combination of associative array and a regular array.

The types of the values do not have to be the same.

To use like a map. Provide a list of key-value pairs where key is specified first followed by a semi-colon and then the value.

```
[
    first_name: "James",
    last_name: "Smith",
    age: 36
]
```

If you want to include an exposed variable inside a map using the variable identifier as the key and variable value as the value. You may omit the value, keeping the tailing semi-colon.
```
[
    first_name:,
    last_name:,
    age:
]
```

To use like an array. Provide only the values, no semi-colon.
```
[10, 20, 30, 40, 50]

// Don't have to be the same type
["cat", 45, false, 3..12]
```

To use as both. Provide a combination of above items.
```
[
    10, // index 0
    20, // index 1
    first_name: "James",
    last_name: "Smith",
    30, // index 2
    age: 36
]
```

#### Indexing
Collections my be index with either an number (index to an array) or a string (key to a map).

Arrays are 0 indexed.

Exposed variables:
* `numbers` = [1, 2, 3, 4, 5]
* `user` = [first_name: "James", last_name: "Smith", age: 36]

```
numbers[3]
// 4

user["first_name"]
// "James"

// Note that indexing with a number string will not return a value from an array
numbers["1"] // Error: doesn't exist
```

## Functions
Exposed functions can be invoke in two ways:
1. Specify function name followed by arguments in a space separated list
2. Specify function name followed by arguments in a comma separated list surrounded by parenthesis

Ex. exposed functions
* `rand` that takes no parameters
* `rand_range` that takes two parameters
* `init_range` that takes three parameters
```
// 1
rand // no arguments, so no argument list necessary
rand_range 10 20
init_range 10 20 2

// 2
rand()
rand_range(10, 20)
```

## Functional operations
Two operators to help make passing values in function chains easier to read.

### Right passing
Exposed variables:
* `min` = 10
* `max` = 20

Exposed functions:
* `rand_range` - takes a minimum and maximum and returns a random value between them
* `init_array` - takes a number and creates an array with that many slots
```
rand_range(min, max) -> init_array

// equivalent to 
init_array(rand_range(min, max))

// may also split out the arguments to rand_range
min, max -> rand_range -> init_array
```
`min` and `max` are evaluated and passed into `rand_range` and then the result of `rand_range` is passed into `init_array`.

### Left passing
Exposed variables:
* `min` = 10
* `max` = 20

Exposed functions:
* `rand_range` - takes a minimum and maximum and returns a random value between them
* `named_array` - takes a string and returns a function that takes a number and creates an array with that many slots
```
named_array("my_array") <- rand_range(min, max)

// split out args, right passing resolve before left passing
"my_array" -> named_array <- min, max -> rand_range

// equivalent to 
named_array("my_array")(rand_range(min, max))

// as well as
rand_range(min, max) -> named_array("my_array")
```
Left chains are evaluated after right chains.

`named_array` is evaluated with argument "my_array" and returns a function that creates an array.

Then `rand_range` is called with `min` and `max`.
 
That result is finally passed into the function that was created by the `named_array` call.

*Since equivalent expressions can be made with both left and right passing, when to use either will be more of a stylistic and readability choice*

## Sub Expressions
Sub expressions are denoted by a `#` followed by an identifier and then curly braces `{}`.

To reference a sub expression later, use the `#` followed by the identifier.
```
// Main expression
#plus_rand {
    $ + rand()
}

// curly brace may also be on next line
#is_even 
{
    $ % 2 = 0
}

init_array(10).map(#plus_rand)
init_array(5).map(#plus_rand).filter(#is_even)

// may also be called directly the same way as functions
#plus_rand 5
#plus_rand(5)
5 -> #plus_rand
``` 

### Input
Input to an expression or sub expression is referenced by the `$` character.

Input may also be an array, in which it can be index to obtain other values.

Input = "Hello"
```
$ + ", SEL!"
// "Hello, SEL!"
```

Input = [1, 2, 3, 4, 5]
```
$[3]
// 4
```

### Dot operator
There is no concept of objects but a dot operator is available for convenience when calling functions.

Exposed functions:
* `map` - takes an array as its first argument and a sub expression or function as its second
* `squared` - takes a number and squares it
```
[1, 2, 3, 4, 5].map(squared)

// Equalivalent to 
map [1, 2, 3, 4, 5] squared
map([1, 2, 3, 4, 5], squared)
[1, 2, 3, 4, 5], squared -> map
```
The dot operator can be thought of as taking the result of the left side and injecting it as the first parameter to the right side function

### External Expressions
Expressions may also be referenced from other sources.

Can reference files directly if entire file as a single expression.

Following example uses external expressions in separate files, but external expressions may be resolved other ways depending on the runtime.

filter.sel
```
$ % 2 = 0
```

Or reference sub-expressions within a file.
 
map.sel
```
#plus_random {
    $ + rand()
}

#squared {
    $^2
}
```

main.sel
```
init_array(10).map(#squared)
init_array(5).map(#plus_random).filter(#filter)
```

May also rename an external expression.

main.sel
```
#is_even <-> #filter

init_array(10).filter(#is_even)
```