# Expression Annotations
The most flexible form of annotation. These are given an name and a number of expressions to run or store at compilation time to customize the expression behavior.

Expression annotations start with the `@` symbol immediately followed by an identifier. 

Below is an annotation to which namespaces to look in for all identifiers in the expression.
```
@Namespace "common::const"
```
`Namespace` is the name of the annotation and `"common::const"` is the expression the annotation will use.

Next, an annotation that takes two expressions.
```
$ + 4

@Test 4
8
```
The first expression `4` is used as the test input and the second `8` is used as the expected test result.