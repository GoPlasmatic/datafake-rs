# Variable System

Variables allow you to pre-generate values and reuse them across your schema. This is useful when you need the same value in multiple places or want to create relationships between fields.

## Defining Variables

Variables are defined in the `variables` section of your configuration:

```json
{
    "variables": {
        "userId": {"fake": ["uuid"]},
        "createdAt": {"fake": ["datetime"]}
    },
    "schema": {
        ...
    }
}
```

Each variable is evaluated once when generation begins, and the result is cached for reuse.

## Referencing Variables

Use the `var` operator to reference a variable:

```json
{"var": "variableName"}
```

### Basic Example

<div class="playground-widget" data-config='{"variables":{"userId":{"fake":["uuid"]},"timestamp":{"fake":["datetime"]}},"schema":{"id":{"var":"userId"},"audit":{"createdBy":{"var":"userId"},"createdAt":{"var":"timestamp"},"updatedAt":{"var":"timestamp"}}}}'>
</div>

In this example, `userId` appears in both `id` and `audit.createdBy` with the same value, and `timestamp` is used for both `createdAt` and `updatedAt`.

## Use Cases

### Consistent IDs Across Related Objects

<div class="playground-widget" data-config='{"variables":{"orderId":{"fake":["uuid"]},"customerId":{"fake":["uuid"]}},"schema":{"order":{"id":{"var":"orderId"},"customerId":{"var":"customerId"}},"lineItems":[{"orderId":{"var":"orderId"},"productId":{"fake":["uuid"]},"quantity":{"fake":["u8",1,10]}}],"payment":{"orderId":{"var":"orderId"},"amount":{"fake":["f64",10.0,1000.0]}}}}'>
</div>

### Generating Related Data

<div class="playground-widget" data-config='{"variables":{"firstName":{"fake":["first_name"]},"lastName":{"fake":["last_name"]},"domain":{"fake":["domain_name"]}},"schema":{"name":{"cat":[{"var":"firstName"}," ",{"var":"lastName"}]},"email":{"cat":[{"var":"firstName"},".",{"var":"lastName"},"@",{"var":"domain"}]},"username":{"cat":[{"var":"firstName"},{"fake":["u8",1,99]}]}}}'>
</div>

### Timestamps for Audit Trails

<div class="playground-widget" data-config='{"variables":{"created":{"fake":["datetime"]},"userId":{"fake":["uuid"]}},"schema":{"id":{"fake":["uuid"]},"data":{"fake":["sentence",5,10]},"metadata":{"createdAt":{"var":"created"},"createdBy":{"var":"userId"},"updatedAt":{"var":"created"},"updatedBy":{"var":"userId"},"version":1}}}'>
</div>

## Variable Scope

Variables are evaluated in the order they are defined, and each variable can only reference variables defined before it.

```json
{
    "variables": {
        "firstName": {"fake": ["first_name"]},
        "lastName": {"fake": ["last_name"]},
        "fullName": {"cat": [{"var": "firstName"}, " ", {"var": "lastName"}]}
    },
    "schema": {
        "name": {"var": "fullName"},
        "firstName": {"var": "firstName"},
        "lastName": {"var": "lastName"}
    }
}
```

<div class="playground-widget" data-config='{"variables":{"firstName":{"fake":["first_name"]},"lastName":{"fake":["last_name"]},"fullName":{"cat":[{"var":"firstName"}," ",{"var":"lastName"}]}},"schema":{"fullName":{"var":"fullName"},"firstName":{"var":"firstName"},"lastName":{"var":"lastName"}}}'>
</div>

## Variables vs Schema Values

| Feature | Variables | Schema Values |
|---------|-----------|---------------|
| Evaluated | Once at start | For each generation |
| Reusable | Yes, via `var` | No |
| Order-dependent | Yes | No |
| Best for | Shared values, relationships | Unique values per field |

## Tips

1. **Use variables for shared IDs** - When multiple fields need the same identifier
2. **Use variables for timestamps** - When created/updated times should match
3. **Use variables for names** - When you need to derive email from name
4. **Keep variables simple** - Complex expressions are better in the schema
