# Identifiers and Symbols
An identifiers are constrained to the following rules.

1. Contains only alphabetic letters, numbers and underscores.
1. Starts with an alphabetic letter or underscore.
1. My end with a number of prime `'` symbols.
```
value
_value
_value1
_my_value
value_prime'
value_double_prime''
```

### Namespaces
Identifiers may be placed under a namespace to resolve different values.

In this example, `common::const` is the namespace and `value` is the identifier.
```
common::const::value
```
Namespaces follow the same rules as identifiers.

## Symbols
Symbols are a special type of identifier that is stored as a value instead of being resolved to a value by the runtime. Symbols are used to access value in an associative list or as flags indicating certain behavior.

Symbols are created by adding a colon immediately before any identifier.
```
:my_symbol
:prime_symbol'
```
Symbols cannot use namespaces.