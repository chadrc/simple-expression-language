# Text

## Characters
Represent any single character code point with by surrounding that character in single quotes.

```
'a'
'b'
'\n
'\0'
```

## Strings
Strings are a collection of characters and are represented by surrounding any number of characters with double quotes.

```
"Hello, World"

"The quick brown fox,\n\tjumped over the lazy dog.\n"
```

Any formatting from the source code is retained in the string value. If the formatting is not needed, then those characters may be escaped.

```
"The quick brown fox,
    jumped over the lazy dog.
"

"The quick brown fox,\
\    jumped over the lazy dog.\
"
```

Formatting is supported by using braces within a string. Escaping the opening brace will allow usage of literal braces.

```
"5 + 4 = {5 + 4}"

"5 + 4 = \{5 + 4}"
```
The first line will output: "5 + 4 = 9"

The second line will output: "5 + 4 = {5 + 4}"
