# JSONLogic Integration

datafake-rs is built on top of [datalogic-rs](https://github.com/GoPlasmatic/datalogic-rs), a high-performance JSONLogic implementation. This means you can use all standard JSONLogic operators alongside the `fake` operator.

## Core Operators

### Variable Access

Use `var` to access variables or nested data:

```json
{"var": "variableName"}
{"var": "nested.path.to.value"}
```

### String Concatenation

Use `cat` to combine strings:

```json
{"cat": ["Hello, ", {"fake": ["name"]}, "!"]}
```

<div class="playground-widget" data-config='{"schema":{"greeting":{"cat":["Hello, ",{"fake":["first_name"]},"!"]},"email":{"cat":[{"fake":["username"]},"@example.com"]}}}'>
</div>

### Conditional Logic

Use `if` for conditional generation:

```json
{"if": [condition, then_value, else_value]}
```

<div class="playground-widget" data-config='{"schema":{"score":{"fake":["u8",0,100]},"grade":{"if":[{">=":[{"fake":["u8",0,100]},90]},"A",{"if":[{">=":[{"fake":["u8",0,100]},80]},"B",{"if":[{">=":[{"fake":["u8",0,100]},70]},"C","D"]}]}]}}}'>
</div>

## Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `{"==": [1, 1]}` |
| `!=` | Not equal | `{"!=": [1, 2]}` |
| `>` | Greater than | `{">": [5, 3]}` |
| `>=` | Greater or equal | `{">=": [5, 5]}` |
| `<` | Less than | `{"<": [3, 5]}` |
| `<=` | Less or equal | `{"<=": [5, 5]}` |

## Logical Operators

### and

All conditions must be true:

```json
{"and": [condition1, condition2, ...]}
```

### or

At least one condition must be true:

```json
{"or": [condition1, condition2, ...]}
```

### not / !

Negates a condition:

```json
{"!": [condition]}
{"not": [condition]}
```

## Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Add | `{"+": [1, 2, 3]}` → 6 |
| `-` | Subtract | `{"-": [10, 3]}` → 7 |
| `*` | Multiply | `{"*": [2, 3, 4]}` → 24 |
| `/` | Divide | `{"/": [10, 2]}` → 5 |
| `%` | Modulo | `{"%": [10, 3]}` → 1 |

<div class="playground-widget" data-config='{"schema":{"basePrice":{"fake":["f64",10.0,100.0]},"quantity":{"fake":["u8",1,10]},"subtotal":{"*":[{"fake":["f64",10.0,100.0]},{"fake":["u8",1,10]}]},"tax":{"*":[{"fake":["f64",100.0,500.0]},0.08]}}}'>
</div>

## Array Operators

### map

Transform each element:

```json
{"map": [array, transformation]}
```

<div class="playground-widget" data-config='{"schema":{"items":{"map":[[1,2,3],{"fake":["word"]}]}}}'>
</div>

### filter

Filter elements by condition:

```json
{"filter": [array, condition]}
```

### reduce

Reduce array to single value:

```json
{"reduce": [array, reducer, initial_value]}
```

### merge

Combine arrays:

```json
{"merge": [array1, array2]}
```

## String Operators

### cat

Concatenate strings:

```json
{"cat": ["string1", "string2", ...]}
```

### substr

Extract substring:

```json
{"substr": ["string", start, length]}
```

## Combining with fake

You can combine any JSONLogic operator with `fake`:

<div class="playground-widget" data-config='{"variables":{"price":{"fake":["f64",10.0,100.0]}},"schema":{"product":{"name":{"fake":["words",2]},"price":{"var":"price"},"formattedPrice":{"cat":["$",{"var":"price"}]},"discountedPrice":{"*":[{"var":"price"},0.9]},"inStock":{"fake":["bool"]}}}}'>
</div>

## Complex Example

<div class="playground-widget" data-config='{"variables":{"basePrice":{"fake":["f64",50.0,200.0]},"quantity":{"fake":["u8",1,5]}},"schema":{"orderId":{"fake":["uuid"]},"items":{"map":[[1,2,3],{"id":{"fake":["uuid"]},"name":{"fake":["words",2]},"price":{"fake":["f64",10.0,50.0]}}]},"subtotal":{"*":[{"var":"basePrice"},{"var":"quantity"}]},"tax":{"*":[{"*":[{"var":"basePrice"},{"var":"quantity"}]},0.08]},"total":{"+":[[{"*":[{"var":"basePrice"},{"var":"quantity"}]},{"*":[{"*":[{"var":"basePrice"},{"var":"quantity"}]},0.08]}]]}}}'>
</div>

## Learn More

For complete JSONLogic documentation, see:
- [JSONLogic.com](https://jsonlogic.com/)
- [datalogic-rs Documentation](https://docs.rs/datalogic-rs)
