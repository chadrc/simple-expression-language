# Language Grammar

## Values

### Characters
`'a'`

`'\n'`

### Strings
`"Hello World"`

### Integers
`0`

`1234567890`

### Decimals
`3.14`

`.3334`

### Booleans
`true`

`false`

### Ranges
`1..10`

`..10`

`1..`

`..`

`1...10`

`...10`

`1...`

`...`

### Identifiers
`my_value`

`common::const::my_value`

### Symbols
`:my_symbol`

`:my_symbol'`

### Unit
`()`

### Pairs
`my_value = 100`

### Lists
`10, 20, 30`

### Associative Lists
`[:first = 100, :second = 200, :third = 300]`

### Expressions
`{ $ + 5 }`

### Groupings
`(5 + 4)`

`(3 / 10 * 5)`

## Operators

### Binary

#### Addition `+`
`5 + 4`

#### Subtraction `-`
`5 - 4`

#### Multiplication `*`
`5 * 4`

#### Division `/`
`5 / 4`

#### Integer Division `//`
`5 // 4`

#### Modulo `%`
`5 % 4`

#### Exponential `**`
`5 ** 4`

#### Concatenation `++`
`"Hello, " ++ "World"`

#### Exclusive Range `..`
`1..10`

#### Inclusive Range `...`
`1...10`

#### Pairing `=`
`100 = "one hundered"`

#### List `,`
`10, 20`

`,20`

`10,`

#### Equality `==`
`100 == 200`

#### Inequality `!=`
`100 !== 200`

#### Greater Than `>`
`100 > 200`

#### Greater Than or Equal `>=`
`100 >= 200`

#### Less Than `<`
`100 < 200`

#### Less Than or Equal `<=`
`100 <= 200`

#### Keys Equal `:=`
`some_data := some_other_data`

#### Keys Not Equal `:!=`
`some_data :!= some_other_data`

#### Values Equal `$=`
`some_data $= some_other_data`

#### Values Not Equal `$!=`
`some_data $!= some_other_data`

#### Contains `~=`
`some_data ~= some_value`

#### Does not Contain `~!=`
`some_data ~!= some_value`

#### Logical Or `||`
`true || false`

#### Logical And `&&`
`true && false`

#### Logical Xor `^^`
`true ^^ false`

#### Bitwise Or `|`
`1 | 2`

#### Bitwise And `&`
`1 & 2`

#### Bitwise Xor `^`
`1 ^ 2`

#### Bitshift Right `>>`
`1 >> 2`

#### Bitshift Left `<<`
`1 << 2`

#### Dot `.`
`my_data.my_value`

##### Partial Application `~`
`10, 20, 30 ~ my_expression`

#### Pipe First Right `->`
`10 -> my_expression`

#### Pipe First Left `<-`
`10 <- my_expression`

#### Pipe Last Right `|>`
`10 |> my_expression`

#### Pipe Last Left `<|`
`10 <| my_expression`

#### Match True `=>`
`true => my_expression`

#### Match False `!=>`
`true !=> my_expression`

#### Stream `>>>`
`some_data >>> my_expression`

#### Combine Streams `<>`
`stream_one <> stream_two`

#### Collect `-<`
`some_stream -< my_expression`

#### Initialize Collect `>-`
`initial_value >- my_expression`

#### Infix Expression
``my_value `my_expression` my_other_value``

### Unary

#### Prefix

##### Negation `-`
`-5`

`-(4 * 4)`

##### Symbol `:`
`:my_value`

##### Not `!`
`!true`

`!4`

#### Suffix

##### Call `()`
`my_expression(10, 20)`

##### Interpreted Access `[]`
`some_data[some_interpreted_key]`