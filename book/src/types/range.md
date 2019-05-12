# Ranges
Represent a range of integer values. All ranges are stored as inclusive ranges, but there is an operator to create exclusive or inclusive ranges.
```
1..10
1...10
```
The first range represents the range [1, 9].

The second range represents the range [1, 10].

## Open Ranges
Open ranges allow range creation with either the min, max or both to be determined based on where they are used. 

Used allow they will extend to the smallest or largest values.

With interpreted access with lists, min will be set to 0 and max will be set to the size of the list.
```
0..
0...
..0
...0
..
...
```
The first range represents the range [0, INT_MAX - 1].

The second range represents the range [0, INT_MAX].

The third range represents the range [INT_MIN, -1].

The fourth range represents the range [INT_MIN, 0].

The fifth range represents the range [INT_MIN, INT_MAX - 1].

The sixth range represents the range [INT_MIN, INT_MAX].
