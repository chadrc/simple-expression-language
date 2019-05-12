# Document Annotations
Document are similar to comments, in that they are used to provided a description of the expression. The difference is that, if enabled, document annotations will be converted to documentation that may be hosted for reference.

Document annotations start with a double `@` symbol and continue till the end of the line. Contiguous lines of these annotations are grouped together to form a document.
```
@@ Document 1 line 1
@@ Document 1 line 2

5 + 4

@@ Document 2 line 1

@@ Document 3 line 1
@@ Document 3 line 2
```