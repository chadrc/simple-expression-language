# Simple Expression Language

## Literals
```
`Comments are enclosed in backticks`

`
Comments can
also be
multi-line
`

`Numbers`
4
29
3.14

`Strings`
'Basic string'

'Can also be
more than one line,
but newlines are stripped'

"
Formatted strings
Newlines are kept
    And so are tabs
"

`Boolean`
true
false

`Ranges`
5..10 `Exclusive, contains 5, 6, 7, 8, and 9`
5...10 `Inclusive, contains 5, 6, 7, 8, 9 and 10`
```

## Math operations
```
`Addition`
10 + 5

`Subtraction`
10 - 5

`Multiplication`
10 * 5

`Division`
10 / 5

`Remainder`
10 % 5

`Exponential`
2^10

`Negation`
-5
```

## String operations
```
`Concatenation`
"Hello, " + "World"
`"Hello, World"`

`Numbers and Booleans are cast into Strings when concated to a String`
"High " + 5
`"High 5"`

"Is " + true
`"Is true"`
```

## Logical operations
```
`Equality`
5 = 4
'Hello' = 'World'

`Inequality`
5 != 4
'Hello' != 'World'

`Less than`
5 < 4
'Hello' < 'World'

`Less than or equal`
5 <= 4
'Hello' <= 'World'

`Greater than`
5 > 4
'Hello' > 'World'

`Greater than or equal`
5 >= 4
'Hello' >= 'World'

`Logical AND`
5 = 4 && 5 < 10
'Hello' = 'World' && 'Hello' < 'World'

`Logical OR`
5 = 4 || 5 < 10
'Hello' = 'World' || 'Hello' < 'World'

`Logical NOT`
!true
```

## Exposed References
If runtime exposes a variable or function it can be referenced just by its name.

Ex. exposed variable `name` = "SEL"
```
"Hello, " + name
`result "Hello, SEL"`
```

Ex. exposed function `rand` that takes no parameters
```
rand()
`result is a randomly generated number`
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

`Don't have to be the same type`
["cat", 45, false, 3..12]
```

To use as both. Provide a combination of above items.
```
[
    10, `index 0`
    20, `index 1`
    first_name: "James",
    last_name: "Smith",
    30, `index 2`
    age: 36
]
```

#### Accessing
Collections may be index with either an number (index to an array) or a string (key to a map).

Map values may also be accessed with the dot operator.

Arrays are 0 indexed.

Exposed variables:
* `numbers` = [1, 2, 3, 4, 5]
* `user` = [first_name: "James", last_name: "Smith", age: 36]

```
numbers[3]
`4`

user["first_name"]
`"James"`

user.last_name
`"Smith"`

`Note that indexing with a number string will not return a value from an array`
numbers["1"] `Error: doesn't exist`
```

## Input
Input to an expression is referenced by the `$` character.

Input may also be an associative array, in which it can be index to obtain other values.

Input = "Hello"
```
$ + ", SEL!"
`"Hello, SEL!"`
```

Input = [1, 2, 3, 4, 5]
```
$[3]
`4`
```

Input = [first_name: "James", last_name: "Smith", age: 36]
```
$["last_name"]
`"Smith"`
```

## Results
Each expression outputs a result. 

The runtime will have access to each result after execution
```
4 + 5
`9`
```

The last result may be referenced with the `?` symbol
```
4 + 5
`9`

? + 11
`? = 9`
`20`

? * 5
`? = 20`
`100`

`may also be used in function calls`
rand_range(0, ?)
`random number between 0, 100`
```

The input is set as the first result of an expression
Input = 10
```
? + $ `10 / 10`
`1`
```

## Expression Blocks
Group a set of expressions together to output a single value

```
`Enclose expressions in a braces`
{
    8 * 5
    ? / 4
}
`10`

? * 10
`100`
```

Blocks receive the last evaluated expression as input
```
5^2 `25`

{
    $ * 3 `75`
    ? / 5 `15`
}

? * 4 `60`
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
`1`
rand `no arguments, so no argument list necessary`
rand_range 10 20
init_range 10 20 2

`2`
rand()
rand_range(10, 20)
```

### Functional operations
Operators for piping arguments into functions/named expressions.

Exposed for all examples

Variables:
* `min` = 10
* `max` = 20
* `numbers` = [1, 2, 3, 4, 5]

Functions:
* `rand_range(min: int, max: int)`
* `map(ary: array, mapExpr: expression)`
* `is_even(num: int)`
* `clamp(num: int, min: int, max: int)`

#### Right Piping
Pass result of a expression (on left) into the first variable position of the another expression (on right).
```
rand_range(min, max) -> is_even

`equivalent to` 
is_even(rand_range(min, max))

`may also split out the arguments to rand_range`
min, max -> rand_range -> is_even
```
`min` and `max` are evaluated and piped into `rand_range` and then the result of `rand_range` is piped into `init_array`.

#### Left Piping
Pass result of a expression (on right) into the first variable position of the another expression (on left).
```
clamp(11, 15) <- rand_range(min, max)

`split out args, right piping resolve before left piping`
clamp(11, 15) <- min, max -> rand_range

`equivalent to` 
clamp(rand_range(min, max), 11, 15)

`as well as`
rand_range(min, max) -> clamp(11, 15)
```

*Since equivalent expressions can be made with both left and right piping, when to use either will be more of a stylistic and readability choice*

#### Pipe Last
Also provided is the ability to pipe into a function starting from the end of the argument list.
```
max |> rand_range(min)

`equivalent to` 
rand_range(min, max)

`Can also left pipe last`
map(numbers) <| is_even

`equivalent to`
map(numbers, is_even)
numbers -> map(is_even)

`Piping last passes the arguments in the same order given.`
min, max |> clamp(34)

`equivalent to`
34, min, max |> clamp
max |> clamp(34, min)
clamp(34, min, max)
```

## Named Expressions
Named expressions are denoted by a `#` followed by an identifier and then curly braces `{}`.

These expressions are not evaluated right away, but instead must be called like a function.

To reference a named expression later, use the `#` followed by the identifier.
```
`Single line`
#plus_rand $ + rand()

`
Multi line
Opening brace can be on same line 
or next line
`
#is_even {
    $ % 2 = 0
}

init_array(10) -> map(#plus_rand)
init_array(5) -> map(#plus_rand) -> filter(#is_even)

`may also be called directly the same way as functions`
#plus_rand 5
#plus_rand(5)
5 -> #plus_rand
``` 

### External Expressions
Named expressions may also be referenced from other sources.

Can reference files directly if entire file as a single expression.

Following example uses external expressions in separate files, but external expressions may be resolved other ways depending on the runtime.

filter.sel
```
$ % 2 = 0
```

Or reference named expressions within a file.
 
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
init_array(10) -> map(#squared)
init_array(5) -> map(#plus_random) -> filter(#filter)
```

## Matching
Can perform pattern matching on a value.

On input or last result

```
`Input: 5`

`last result, implicit match on input
1 => "One",
2 => "Couple",
3 => "Few",
4 => "Some",
5 => "Many",
12 => "Dozen",
_ => "Several"

`"Many"`

`explicit match on input`

$ `Effectivly making it the last result

1 => "One",
2 => "Couple",
3 => "Few",
4 => "Some",
5 => "Many",
12 => "Dozen",
_ => "Several"

`"Many"`

`Match on last result again`
10 - 8

1 => "One",
2 => "Couple",
3 => "Few",
4 => "Some",
5 => "Many",
12 => "Dozen",
_ => "Several"

`"Couple"`
```

Since there are no variants, enums or types, in order to be an exhaustive match the _ "catch all" pattern must be specified.

### Matching with Associative Arrays

### Matching with Named Expressions

## Annotations
Annotations are used to provide a runtime with metadata about expressions.

Mostly reserved for a runtime to add customization there are some standard annotations.

### @Runtime

### @Input

### @Result

### @Test