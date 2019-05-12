# Pairs
Pair two values together with the `=` operator.
```
"first" = 100
```
The pair operator is evaluated from right to left. This allows creating a pair with a nested pair value on the right side. Parenthesis may also be used to be explicit. The following two lines are equivalent.
```
"first" = "second" = 100
"first" = ("second" = 100)
```
In order to place a pair on the left side of another pair parenthesis are required.
```
("first" = 100) = "second"
```