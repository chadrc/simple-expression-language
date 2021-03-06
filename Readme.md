# Simple Expression Language

Still a work in progress, but some examples have been written.

If you don't have Rust installed, follow instructions at [https://rustup.rs](https://rustup.rs) to do so.

Run following to execute example project.
```bash
git clone https://github.com/chadrc/simple-expression-language
cd simple-expression-language

cargo run -p test_bed
```

# Specification
The following spec is less of a living spec and more of a place to organize my thoughts. When the features and grammar are more established, I plan on making a Book detailing everything.

## Comments and Documentation

```
@ Comments start with '@' followed by a space and continue to end of line

@@ 
Document comment blocks, start with two '@' symbols,
and contiue until another two '@' symbols.
@@

@@ Comment blocks don't have to be multi line either @@
```

## Literals

```

@ Numbers
4
-8
29
3.14

@ Strings
'Basic string'
"Double quoted"

'Can also be on
more than one line,
but newlines are stripped'

@ Boolean
true
false

@ Ranges
5..10 @ Exclusive, contains 5, 6, 7, 8, and 9
5...10 @ Inclusive, contains 5, 6, 7, 8, 9 and 10

@ Symbol
:my_symbol

// may have up to 256 prime symbols following symbol identifier
:my_symbol'

@ Pairs
value = 123
```

### Non-existence

There is no null value but the unit value `()` is provided to represent something that is uninitialized or empty.

## Math operations

```
@ Addition
10 + 5

@ Subtraction
10 - 5

@ Multiplication
10 * 5

@ Float Division
10 / 3
@ 3.333334

@ Integer Division
10 // 3
@ 3

@ Remainder
10 % 5

@ Exponential
2 ** 10

@ Negation
-5
```

## Bitwise operations
```
@ or
4 | 5

@ and
4 & 5

@ xor
4 ^ 5

@ not
!5

@ shift
5 << 2
2 >> 5
```

## String operations

```
@ Concatenation
"Hello, " + "World"
@ "Hello, World"

@ Numbers and Booleans are cast into Strings when concatenated to a String
"High " + 5
@ "High 5"

"Is " + true
@ "Is true"

@ Length
"Hello, World!"`l
@ 13
```

## Symbol operations
```
@ Get prime count
:my_symbol'''.prime

@ 3
```

## Range operations

```
@ Length
5..10`l
@ 5

5...10`l
@ 6
```

## Logical operations

```
@ Equality
5 == 4
'Hello' == 'World'

@ Inequality
5 != 4
'Hello' != 'World'

@ Less than
5 < 4
'Hello' < 'World'

@ Less than or equal
5 <= 4
'Hello' <= 'World'

@ Greater than
5 > 4
'Hello' > 'World'

@ Greater than or equal
5 >= 4
'Hello' >= 'World'

@ Logical AND
5 == 4 && 5 < 10
'Hello' == 'World' && 'Hello' < 'World'

@ Logical OR
5 == 4 || 5 < 10
'Hello' == 'World' || 'Hello' < 'World'

@ Logical XOR
5 == 4 ^^ 5 < 10
'Hello' == 'World' ^^ 'Hello' < 'World'

@ Logical NOT
!true
```

## Exposed References

If runtime exposes a variable or function it can be referenced just by its name.

Ex. exposed variable `name` = "SEL"

```
"Hello, " + name
@ result "Hello, SEL"
```

Ex. exposed function `rand` that takes no parameters

```
rand()
@ result is a randomly generated number
```

### Associative Array

A single collection type is provided being a combination of associative array and a regular array.

The types of the values do not have to be the same.

To use like a map. Provide a list of pair objects where key is specified first followed by a semi-colon and then the value.

```
[
    :first_name = "James", @ use a symbol key
    "last_name" = "Smith", @ a string key
    email = "james@example.com", @ unbound identifier automatically treated as a symbol
    1 = 36, @ or an integer key
]
```

If you want to include an exposed variable inside a map using the variable identifier as the key and variable value as the value. You may omit the value, keeping the tailing semi-colon.
_Won't work right now, rethink design_
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

@ Don't have to be the same type
["cat", 45, false, 3..12]
```

To use as both. Provide a combination of above items.

```
[
    10, @ index 0
    20, @ index 1
    first_name = "James",
    last_name = "Smith",
    30, @ index 2
    age = 36
]
```

#### Concatenation

Can use piping operators on associative arrays to concat them together or with other primitives

Prepend

```
0 -> [1, 2, 3, 4, 5]
@ [0, 1, 2, 3, 4, 5]

[1, 2, 3, 4, 5] <- 0
@ [0, 1, 2, 3, 4, 5]

[-1, 0] -> [1, 2, 3, 4, 5]
@ [-1, 0, 1, 2, 3, 4, 5]

[1, 2, 3, 4, 5] <- [-1, 0]
@ [-1, 0, 1, 2, 3, 4, 5]
```

Append

```
6 |> [1, 2, 3, 4, 5]
@ [1, 2, 3, 4, 5, 6]

[1, 2, 3, 4, 5] <| 6
@ [1, 2, 3, 4, 5, 6]

[6, 7] |> [1, 2, 3, 4, 5]
@ [1, 2, 3, 4, 5, 6, 7]

[1, 2, 3, 4, 5] <| [6, 7]
@ [1, 2, 3, 4, 5, 6, 7]
```

_Note: that these operators produce a new associative array. The original arrays are unmodified._

##### With key-values

When using concatenation operations with key-value pairs, 'prepend' and 'append' are equivalent since key-value pairs are unordered. Same is true for the direction of the operator.

```
[email = "john@example.com"] -> [first_name = "John", last_name = "Smith"]
[email = "john@example.com"] <- [first_name = "John", last_name = "Smith"]
[email = "john@example.com"] |> [first_name = "John", last_name = "Smith"]
[email = "john@example.com"] <| [first_name = "John", last_name = "Smith"]

@ All above have same result
@ [first_name = "John", last_name = "Smith", email = "john@example.com"]
```

#### Comparison

Associative arrays can be compared by values, key-value pairs, and keys.

By value operators - `$=` and `$!=`

By key operators - `:=` and `:!=`

By key and value operators - `==` and `!=`. Same as equality operators above.

Values - Checks to see if the two arrays contain the same values

```
[1, 2, 3, 4, 5] $= [1, 2, 3, 4, 5]
@ true

[1, 2, 3, 4, 5] $= [4, 3, 5, 2, 1]
@ true

[1, 2, 3, 4, 5] $= [4, 3, 5, 2, 6]
@ false

[1, 2, 3, 4, 5] $= [1, 2, 3, 4]
@ false

[first_name = "John", last_name = "Smith"] $= [first_name = "John", last_name = "Smith"]
@ true

[first_name = "John", last_name = "Smith"] $= [first_name = "John", last_name = "Anderson"]
@ false

[first_name = "John", last_name = "Smith"] $= [first_name = "John", last_name = "Smith", admin: true]
@ false

[] $= []
@ true
```

Key-value Pairs - Check to see if the two arrays contain all the same keys with their matching values.

```
[1, 2, 3, 4, 5] == [1, 2, 3, 4, 5]
@ true

[1, 2, 3, 4, 5] == [4, 3, 5, 2, 1]
@ false

[first_name = "John", last_name = "Smith"] == [first_name = "John", last_name = "Smith"]
@ true

[first_name = "John", last_name = "Smith"] == [first_name = "John", last_name = "Anderson"]
@ false

[first_name = "John", last_name = "Smith"] == [first_name = "John", last_name = "Smith", admin = true]
@ false

[] == []
@ true
```

Keys - Checks to see if the two arrays contain all the same keys.

```
[1, 2, 3, 4, 5] := [1, 2, 3, 4, 5]
@ true

[1, 2, 3, 4, 5] := [4, 3, 5, 2, 1]
@ true

[1, 2, 3, 4, 5] := [4, 3, 5, 2]
@ false

[1, 2, 3, 4, 5] := [4, 3, 5, 2, 6]
@ true

[first_name = "John", last_name = "Smith"] := [first_name = "John", last_name = "Smith"]
@ true

[first_name = "John", last_name = "Smith"] := [first_name = "John", last_name = "Anderson"]
@ true

[first_name = "John", last_name = "Smith"] := [first_name = "John", last_name = "Smith", admin = true]
@ false

[] := []
@ true
```

#### Contains

Can check if an associative array contains a specific value, key-value, or key with the `~=` operator. Also check if the collections doesn't contain a value with the `~!=` operator.

Contains Value

```
[1, 2, 3, 4, 5] ~= 1
@ true

[1, 2, 3, 4, 5] ~!= 1 @ Doesn't contain
@ false

[1, 2, 3, 4, 5] ~= 6
@ false

[first_name = "John", last_name = "Smith"] ~= "John"
@ true

[first_name = "John", last_name = "Smith"] ~= "Anderson"
@ false
```

Contains Key-Value pair
```
[1, 2, 3, 4, 5] ~= :0 = 1
@ true

[1, 2, 3, 4, 5] ~= :0 = 2
@ false

[first_name = "John", last_name = "Smith"] ~= first_name = "John"
@ true

[first_name = "John", last_name = "Smith"] ~= last_name = "Anderson"
@ false
```

Contains Key

```
[1, 2, 3, 4, 5] ~= :1
@ true

[1, 2, 3, 4, 5] ~= :6
@ false

[first_name = "John", last_name = "Smith"] ~= :last_name
@ true

[first_name = "John", last_name = "Smith"] ~= :admin
@ false
```

Can also specify multiple and a mix of the checks

```
@ Contains the value 2, the pair [2: 3], and the key 4
[1, 2, 3, 4, 5] ~= [2, 2 = 3, :4]
@ true
```

#### Accessing

Collections may be index with either an number (index to an array) or a string (key to a map).

Map values may also be accessed with the dot operator.

Arrays are 0 indexed.

Exposed variables:

- `numbers` = [1, 2, 3, 4, 5]
- `user` = [first_name: "James", last_name: "Smith", age: 36]

```
numbers[3]
@ 4

user["first_name"]
user[:first_name]
@ "James"

user.last_name
@ "Smith"

@ Note that indexing with a number string will not return a value from an array
numbers["1"] @ doesn't exist
@ ()
```

#### Length

Can use the length operator on associative arrays but note that it will return total number of values even if there is a mix of index keys and string keys.

```
[1, 2, 3, 4, 5]`l
@ 5

[first_name = "James", last_name = "Smith", age = 36]`l
@ 3

[1, 2, 3, 4, 5, first_name = "James", last_name = "Smith", age = 36]`l
@ 8
```

## Input

Input to an expression is referenced by the `$` character.

Input may also be an associative array, in which it can be index to obtain other values.

Input = "Hello"

```
$ + ", SEL!"
@ "Hello, SEL!"
```

Input = [1, 2, 3, 4, 5]

```
$[3]
@ 4
```

Input = [first_name: "James", last_name: "Smith", age: 36]

```
$["last_name"]
@ "Smith"
```

## Results

Each expression outputs a result.

The runtime will have access to each result after execution

```
4 + 5
@ 9
```

The last result may be referenced with the `?` symbol

```
4 + 5
@ 9

? + 11
@ ? == 9
@ 20

? * 5
@ ? == 20
@ 100

@ may also be used in function calls
rand_range(0, ?)
@ random number between 0, 100
```

The input is set as the first result of an expression
Input = 10

```
? + $ @ 10 / 10
@ 1
```

## Expression Blocks

Group a set of expressions together to output a single value

```
@ Enclose expressions in a braces, passing in input
$ -> {
    8 * 5
    ? / 4
}
@ 10

? * 10
@ 100
```

If you don't pass in any value to the block, it does nothing. These blocks are used to pass as parameters to other expressions and functions.

```
{
    $ * 3
}
@ resulting expression

@ ? is the above expression
?(25)
@ 75
```

## Functions

Exposed functions can be invoke in two ways:

1. Specify function name followed by arguments in a space separated list
2. Specify function name followed by arguments in a comma separated list surrounded by parenthesis

Ex. exposed functions

- `rand()`
- `rand_range(min: int, max: int)`
- `clamp(num: int, min: int, max: int)`

```
@ 1
rand @ no arguments, so no argument list necessary
rand_range 10 20
clamp 10 20 2

@ 2
rand()
rand_range(10, 20)
```

### Pass by parameter name

Can also pass arguments by parameter name for clarity.

```
@ 1
rand_range min=10, max=20

@ 2
clamp(num=3, min=10, max=20)
```

### Functional operations

Operators for piping arguments into functions/named expressions.

Exposed for all examples

Variables:

- `min` = 10
- `max` = 20
- `numbers` = [1, 2, 3, 4, 5]

Functions:

- `rand_range(min: int, max: int)`
- `map(ary: array, mapExpr: expression)`
- `is_even(num: int)`
- `clamp(num: int, min: int, max: int)`

#### Right Piping

Pass result of a expression (on left) into the first variable position of the another expression (on right).

```
rand_range(min, max) -> is_even

@ equivalent to
is_even(rand_range(min, max))

@ may also split out the arguments to rand_range
min, max -> rand_range -> is_even
```

`min` and `max` are evaluated and piped into `rand_range` and then the result of `rand_range` is piped into `init_array`.

#### Left Piping

Pass result of a expression (on right) into the first variable position of the another expression (on left).

```
clamp(11, 15) <- rand_range(min, max)

@ split out args, right piping resolve before left piping
clamp(11, 15) <- min, max -> rand_range

@ equivalent to
clamp(rand_range(min, max), 11, 15)

@ as well as
rand_range(min, max) -> clamp(11, 15)
```

_Since equivalent expressions can be made with both left and right piping, when to use either will be more of a stylistic and readability choice_

#### Pipe Last

Also provided is the ability to pipe into a function starting from the end of the argument list.

```
max |> rand_range(min)

@ equivalent to
rand_range(min, max)

@ Can also left pipe last
map(numbers) <| is_even

@ equivalent to
map(numbers, is_even)
numbers -> map(is_even)

@ Piping last passes the arguments in the same order given.
min, max |> clamp(34)

@ equivalent to
34, min, max |> clamp
max |> clamp(34, min)
clamp(34, min, max)
```

### Infix Notation
Infix notation for a function allows you to place an argument to the function before the function name. The infix function will receive both the value on the left and the value on the right as its arguments.

```
10 `max` 15

@ 15 
```

## Named Expressions

Named expressions are denoted by a `#` followed by an identifier and then curly braces `{}`.

These expressions are not evaluated right away, but instead must be called like a function.

To reference a named expression later, use the `#` followed by the identifier.

```
@ Single line
#plus_rand $ + rand()

@ Multi line
@ Opening brace can be on same line
@ or next line

#is_even {
    $ % 2 == 0
}

init_array(10) -> map(#plus_rand)
init_array(5) -> map(#plus_rand) -> filter(#is_even)

@ pass inline expression with a block
init_array(5) -> map(#plus_rand) -> filter({ $ % 2 == 1 }) @ is odd for filter

@ may also be called directly the same way as functions
#plus_rand 5
#plus_rand(5)
5 -> #plus_rand
```

### Multiple inputs

An expression technically will always only have one input represented by the `$` symbol.

But you may pass multiple values and the `$` will be converted to an array with those values in the same order.

```
#sum_divisible_by_3 {
    $[0] + $[1] % 3 == 0
}

#sum_divisible_by_3(7, 3)
@ false
```

#### Passing by parameter name

Just like functions, you can pass parameters by name to a named expression. However this changes how they can be accessed in the named expression.

```
#sum_divisible_by_3 {
    $.num1 + $.num2 % 3 == 0
}

#sum_divisible_by_3(num1=7, num2=3)
@ false
```

The parameters may no longer be accessed from the input with their index position and must you must always pass parameters by name for this named expression.

### External Expressions

Named expressions may also be referenced from other sources.

Can reference files directly if entire file as a single expression.

Following example uses external expressions in separate files, but external expressions may be resolved other ways depending on the runtime.

filter.sel

```
$ % 2 == 0
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

## Partial Arguments

Both functions and named expressions can be curried.

Functions:

- `clamp(num: int, min: int, max: int)`

```
#clamp_10 clamp ~ (10)
@ this new expresson now has takes 2 arguments, min and max

@ can also provide multiple values
#clamp_10_min_5 clamp ~ (10, 5)
@ takes one argument, max

@ can skip parameters by omitting them
#clamp_min_5 clamp ~ (, 5)
@ takes two arguments, num and max

@ can partially apply on an already partially applied function

#clamp_5_to_15 clamp_min_5 ~ (, 15)

@ taks one argument, num
```

### Partial Arguments by name

You may also provide an argument by its name. This lets you avoid the need to ignore parameters.

Assuming clamp accesses its inputs by name (see Functions _TODO: add link_ section).

```
#clamp_min_5 ~ clamp(:min=5)

#clamp_5_to_15 ~ clamp_min_5(:max=15)

#clamp_10_to_20 ~ clamp(:min=10, :max=20)
```

## Match Operations

Match operators evaluate their left side to determine if they match the current result. If left does not match then current result is re-output

Operators:
* `=>` - Executes right if left evaluates to `true`.
* `!=>` - Executes right if left evaluates to `false`.

On input or last result

```
@ Input: 5

@ last result, implicit match on input
$ > 3 => ? + 1
@ 6
```

### Match Lists
Match expressions can be separated by a comma to make a list of matches to evaluate. They are checked in declaration order and checking stops as soon as the first match expression is evaluated.

```
@ Input: 10

? % 15 == 0 => "Divisible by 15",
? % 5 == 0 => "Divisible by 5",
? % 3 == 0 => "Divisible by 3"

@ ? == "Divisible by 5"
```

Provide a default match by using `true =>` as the last match in the list

```
@ Input: 4

? % 15 == 0 => "Divisible by 15",
? % 5 == 0 => "Divisible by 5",
? % 3 == 0 => "Divisible by 3",
true => "Not divisible by 3, 5, or 15"

@ ? == "Not divisible by 3, 5, or 15"
```

### Something or Nothing

To check if a value has not been initialized the unit `()` as the match condition with the `==>`

```
@ Unit evaluates to false, so use false match on value and a true arm as the default

() !=> "Value is uninitialized",
true => "Value has some type of value"
```

## Iteration

There are two operations for iteration of associative arrays.

### Streaming

Streaming takes in an collection passing each value to the right side expression and then outputs that expressions result one at a time

The right side is an expression with input:

[value: any, index: Integer]

```
(100, 200, 300, 400, 500) >>> $.value / 100

@ 1
@ 2
@ 3
@ 4
@ 5

```

#### Combining streams
Combine two or more streams in order to output there values together.

```
@ Combine 2 streams
(10, 20, 30) <> (40, 50, 60) >>> $.0, $.1
@ (10, 40)
@ (20, 50)
@ (30, 60)

@ Combine 3 or more
(10, 20, 30) <> (40, 50, 60) <> (70, 80, 90) >>> $.0, $.1, $.2
@ (10, 40, 70)
@ (20, 50, 80)
@ (30, 60, 90)


@ Combine 2 the combine with another or more
((10, 20, 30) <> (40, 50, 60)) <> (70, 80, 90) >>> $.0, $.1
@ ((10, 40), 70)
@ ((20, 50), 80)
@ ((30, 60), 90)
```

### Collecting

Collecting is initialized with a seed value, takes in a associative array, range or stream and passes each value into a given expression.

Operator in form `<any>` where 'any' is the seed value. Must be a value literal value.

The right side expression receives input with shape:

[result: T, value: any]

Where result is the type of the seed value (or any for case of no seed value) and value is the current value being streamed.

Result becomes the value returned from the expression

Sum numbers in an array

```
@ Input: [1, 2, 3, 4, 5]

$ >>> $ * 2 >- 0 -< $.value + $.result
@ 15
```

Map an array

```
@ Input: [1, 2, 3, 4, 5]

$ >>> $ * 2 <[]> $.value -> $.result
@ [2, 4, 6, 8, 10]
```

For convenience, if doing a simple assignment to a new associative array like the above, the right side expression may be omitted

```
@ Input: [1, 2, 3, 4, 5]

$ >>> $ * 2 <[]>
@ [2, 4, 6, 8, 10]
```

This will take the input stream and insert them into the new array in order received, for value-stream, or into the received key for key-pair stream.

May also opt out of collecting by not specifying a seed value. This will output the input to the stream.

```
@ Input: [1, 2, 3, 4, 5]

$ >>> log($) <>
@ [1, 2, 3, 4, 5]
```

Can use the first value in the stream as the input with the `?` symbol as the seed.

```
@ Input: [1, 2, 3, 4, 5]

@ multiply by 2, then take average
$ >>> $ * 2 <?> ($.result + $.value) / 2
@ 6
```

The first execution of the collection expression is performed with [result: 1, value: 2] since it was seeded with the first value of the array.

Use collection directly on an associative array, by omitting the stream all together.

```
@ Input: [1, 2, 3, 4, 5]

@ take average
$ <?> ($.value + $.result) / 2
@ 3

@ shallow clone
$ <[]>
@ [1, 2, 3, 4, 5]
```

## Annotations

Annotations are used to provide a runtime with metadata about expressions.

Annotations are defined starting with `@` symbol followed by an identifier. Then an optional set of parenthesis that may contain any number of values separated by commas. This value is not parse as SEL, instead being passed to the run-context for handling.

```
@NoValue

@OneValue(MyValue)

@ManyValues(FirstValue, SecondValue)
```

### Standard Annotations

Although, mostly reserved for a runtime to add customization, there are some standard annotations.

#### @Context

Used to specify which run-contexts an expression is expecting.

One per expression.

```
@Context(MyContext)
@Context(FirstContext, SecondContext)
```

#### @Shape

Used to describe which values and their types that my appear in an associative array.

Any number may be defined

```
@ Object shape
@Shape(MyShape, [value1: int, value2: string])

@ Array shape
@Shape(MyArray, [int...])

@ Tuple like
@Shape(MyTuple, [int, string, bool])
```

#### @Input

Describes the shape of the input to an expression.

One per expression

```
@ Single input, my omit brackets
@Input(int)

@ tuple like
@Input([int, string, bool])

@ use predefined @Shape
@Shape(MyArray, [int...])
@Input(MyArray)
```

#### @Result

Describes the shape of the output of an expression.

One per expression

```
@ Single ouput, my omit brackets
@Result(int)

@ tuple like
@Result([int, string, bool])

@ use predefined @Shape
@Shape(MyArray, [int...])
@Result(MyArray)
```

#### @Test

Flags a test expression. Test expressions may be used in two ways.

1 - Test the expression file.

```
@Input([int...])

@ take average
$ <?> ($.value + $.result) / 2

@Test([1, 2, 3, 4, 5)
$ == 3
```

Here, the [1, 2, 3, 4, 5] inside of the @Test annotation will be passed into the expression generated from the file and its output will be passed to the expression below the @Test annotation.

2 - Test a named expression within a file.

```
@Input([int...])
#sum $ <0> $.result + $.value

@Input([int...])
#avg $ <?> ($.result + $.value) / 2

@Test
#avg([1, 2, 3, 4, 5]) == 3

@Test
#sum([1, 2, 3, 4, 5]) == 15
```

Here, we call the named expressions directly inside the test expression.

Test expressions must return a boolean, either `true` or `false`.

They are also omitted from final compiled output.

#### @Mock

Input to functions and named expressions may be mocked for testing in isolation. These mocks apply to all tests that come after their declaration.

@Mock(expression_name, result, input)

- expression_name - name of the function or named expression to mock
- result - result that will be used when invoking the expression
- input - _optional_ result will only be output if input during test matches input specified here

```
@Input(string)

get_user_by_id($).first_name

@Mock(get_user_by_id, [first_name: "John", last_name: "Smith"])
@Mock(get_user_by_id, (), "unknown_id")

@Test("some id string")
$ == "John"

@Test("unknown_id")
$ == ()
```

##### MockOnce

To apply a mock to only the next test in the file use @MockOnce. Takes same parameters as @Mock.

```
@Input(string)

get_user_by_id($).first_name

@Mock(get_user_by_id, [first_name: "John", last_name: "Smith"])

@MockOnce(get_user_by_id, (), "unknown_id")
@Test("unknown_id")
$ == ()

@Test("unknown_id")
$ == "John"
```

#### @Exhaustive

For cases where a match expression is exhaustive due to how the run context defines the input and the compiler can't figure that out we can flag it as such.

```
@Input(int)

@Exhaustive
0 => "Sunday",
1 => "Monday",
2 => "Tuesday",
3 => "Wednesday",
4 => "Thursday",
5 => "Friday",
6 => "Saturday"
```
