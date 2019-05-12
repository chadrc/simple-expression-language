# Streams
To iterate over collections or process streams of data the stream operator is used to created a stream object.
```
some_list >>> $ + 4

some_list >>> some_processor
```
Mentioned in [expressions](./expressions.md) the stream operator will interpret the right side as an implicit expression block unless a single identifier is provided. In which case that identifier is assumed to represent an expression or function that will be invoked.

The right side expression is called once for each element of the collection or stream and receives that element as input.