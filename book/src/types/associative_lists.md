# Associative Lists
Similar to lists except that these allow associations to be made with values in the list so they may be accessed by symbols.

Create an associative list by surrounding a list with brackets.
```
[100, 200, 300]
```

Associations are created when a pair with a symbol on the left side is provided as an element to the list.
```
[:id = 100, :name = "Alice"]
```
This will allow dot access and interpreted access by symbol or string to the contained values.

If a list has already been defined, surrounding it in brackets will effectively convert it to an associative list. In order to create an associative list with a pre-made list add a comma before or after the list identifier.
```
[some_list]

[,some_list]

[some_list,]
```
The first list is created from the elements of `some_list`, while the other two are initialized with `some-list` as the first element in the list.