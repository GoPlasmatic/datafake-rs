# Basic Concepts

## Configuration Structure

A datafake configuration is a JSON object with three optional sections:

```json
{
    "metadata": { ... },
    "variables": { ... },
    "schema": { ... }
}
```

### Metadata

Optional information about the configuration:

```json
{
    "metadata": {
        "name": "User Generator",
        "version": "1.0.0",
        "description": "Generates fake user data"
    }
}
```

### Variables

Pre-generate values that can be reused across the schema:

```json
{
    "variables": {
        "userId": {"fake": ["uuid"]},
        "createdAt": {"fake": ["datetime"]}
    },
    "schema": {
        "id": {"var": "userId"},
        "audit": {
            "createdBy": {"var": "userId"},
            "createdAt": {"var": "createdAt"}
        }
    }
}
```

<div class="playground-widget" data-config='{"variables":{"userId":{"fake":["uuid"]},"timestamp":{"fake":["datetime"]}},"schema":{"id":{"var":"userId"},"audit":{"createdBy":{"var":"userId"},"createdAt":{"var":"timestamp"},"updatedAt":{"var":"timestamp"}}}}'>
</div>

### Schema

The schema defines the structure of the generated output. It uses JSONLogic operators, with the `fake` operator being the most important.

## The `fake` Operator

The `fake` operator generates fake data of a specified type:

```json
{"fake": ["type"]}
{"fake": ["type", arg1, arg2]}
```

### Simple Types

```json
{"fake": ["uuid"]}
{"fake": ["name"]}
{"fake": ["email"]}
```

### Types with Arguments

Some types accept arguments for customization:

```json
{"fake": ["u8", 18, 65]}     // Integer between 18 and 65
{"fake": ["password", 8, 16]} // Password with 8-16 characters
{"fake": ["words", 5]}        // 5 random words
```

## JSONLogic Operators

datafake-rs supports all standard JSONLogic operators. Here are some commonly used ones:

### Variable Access

Reference variables with `var`:

```json
{"var": "userId"}
{"var": "user.name"}
```

### String Concatenation

Combine strings with `cat`:

<div class="playground-widget" data-config='{"schema":{"fullName":{"cat":[{"fake":["first_name"]}," ",{"fake":["last_name"]}]},"email":{"cat":[{"fake":["username"]},"@example.com"]}}}'>
</div>

### Conditional Logic

Use `if` for conditional generation:

<div class="playground-widget" data-config='{"schema":{"age":{"fake":["u8",18,65]},"status":{"if":[{">=":[{"fake":["u8",0,100]},50]},"premium","basic"]}}}'>
</div>

### Arrays

Generate arrays with `map`:

```json
{
    "schema": {
        "tags": {"map": [
            [1, 2, 3],
            {"fake": ["word"]}
        ]}
    }
}
```

## Nested Objects

Create complex nested structures:

<div class="playground-widget" data-config='{"schema":{"user":{"id":{"fake":["uuid"]},"profile":{"name":{"fake":["name"]},"email":{"fake":["email"]}},"address":{"street":{"fake":["street_address"]},"city":{"fake":["city_name"]},"country":{"fake":["country_name"]}}}}}'>
</div>

## Error Handling

Invalid configurations will result in clear error messages:

- **ConfigParse** - Invalid JSON syntax
- **InvalidConfig** - Missing required fields
- **FakeOperatorError** - Unknown fake type or invalid arguments
- **VariableNotFound** - Referenced variable doesn't exist
