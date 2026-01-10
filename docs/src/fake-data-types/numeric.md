# Numeric Types

Generate random numbers with optional range constraints.

## Integers

### Unsigned Integers

#### u8 (0 to 255)

```json
{"fake": ["u8"]}
{"fake": ["u8", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"random":{"fake":["u8"]},"age":{"fake":["u8",18,65]},"score":{"fake":["u8",0,100]}}}'>
</div>

#### u16 (0 to 65,535)

```json
{"fake": ["u16"]}
{"fake": ["u16", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"port":{"fake":["u16",1024,65535]},"year":{"fake":["u16",1900,2024]}}}'>
</div>

#### u32 (0 to 4,294,967,295)

```json
{"fake": ["u32"]}
{"fake": ["u32", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["u32"]},"quantity":{"fake":["u32",1,1000]}}}'>
</div>

#### u64 (0 to 18,446,744,073,709,551,615)

```json
{"fake": ["u64"]}
{"fake": ["u64", min, max]}
```

### Signed Integers

#### i8 (-128 to 127)

```json
{"fake": ["i8"]}
{"fake": ["i8", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"temperature":{"fake":["i8",-40,50]},"offset":{"fake":["i8",-10,10]}}}'>
</div>

#### i16 (-32,768 to 32,767)

```json
{"fake": ["i16"]}
{"fake": ["i16", min, max]}
```

#### i32 (-2,147,483,648 to 2,147,483,647)

```json
{"fake": ["i32"]}
{"fake": ["i32", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"balance":{"fake":["i32",-10000,100000]},"delta":{"fake":["i32",-100,100]}}}'>
</div>

#### i64

```json
{"fake": ["i64"]}
{"fake": ["i64", min, max]}
```

## Floating Point

### f32 (32-bit float)

```json
{"fake": ["f32"]}
{"fake": ["f32", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"rating":{"fake":["f32",0.0,5.0]},"percentage":{"fake":["f32",0.0,100.0]}}}'>
</div>

### f64 (64-bit float)

```json
{"fake": ["f64"]}
{"fake": ["f64", min, max]}
```

<div class="playground-widget" data-config='{"schema":{"price":{"fake":["f64",9.99,999.99]},"latitude":{"fake":["f64",-90.0,90.0]},"longitude":{"fake":["f64",-180.0,180.0]}}}'>
</div>

## Usage Notes

- When using ranges, both `min` and `max` are inclusive
- If no range is specified, the full range of the type is used
- Float types may produce very large or very small numbers without constraints
