
## base types

```rust
#[token("Type")]
Type,
#[token("Uuid")]
Uuid,
#[token("String")]
StringType,
#[token("Int")]
Int,
#[token("Float")]
Float,
#[token("Bool")]
Bool,
#[token("Date")]
Date,
#[token("Timestamp")]
Timestamp,
#[token("Money")]
Money,
```

## Concrete types
```
Concrete<T>
```
ex : Concrete<String> (ilk) -> "hello" (kli)


## Struct types

format : {cardinality <T>}

```ilk
{1 Type}
{2 Type}
{* Type}
```

```kli
{hello Int}

{
    hello Int
    goodbye String
}

{
    hello Int
    goodbye String
    other String
    another Bool
}
```

## block (user defined types)
start with capital letter, ex: User, Product, Order, etc.

```ilk
Command {
    fields {* Type}
}
```

```kli
Command {
    fields {
        name String
        age Int
        email String
    }
}
```

## intersection types
```ilk
Event {* Type} & {timestamp Int}
```

```kli
Event {
    name String
    other Bool
    timestamp Int
}
```

## union types
```ilk
Response {success Bool} | {error String}
```

```kli
Response {
    success Bool
}
```

```kli
Response {
    error String
}
```


