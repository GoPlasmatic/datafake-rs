# Other Data Types

Additional fake data types including booleans, files, barcodes, and custom choices.

## Boolean

### bool / boolean

Generates a random boolean:

```json
{"fake": ["bool"]}
{"fake": ["boolean"]}
```

<div class="playground-widget" data-config='{"schema":{"isActive":{"fake":["bool"]},"isVerified":{"fake":["boolean"]},"hasSubscription":{"fake":["bool"]}}}'>
</div>

## Custom Choices

### enum / pick / choice

Pick a random value from provided options:

```json
{"fake": ["enum", "option1", "option2", "option3"]}
{"fake": ["pick", "option1", "option2", "option3"]}
{"fake": ["choice", "option1", "option2", "option3"]}
```

<div class="playground-widget" data-config='{"schema":{"status":{"fake":["enum","pending","active","completed","cancelled"]},"priority":{"fake":["pick","low","medium","high","critical"]},"size":{"fake":["choice","S","M","L","XL"]}}}'>
</div>

### regex

Generate from simple alternation patterns:

```json
{"fake": ["regex", "(A|B|C)"]}
```

> **Note:** Only supports simple alternation patterns like `(A|B|C)`. Complex regex patterns are not supported.

<div class="playground-widget" data-config='{"schema":{"grade":{"fake":["regex","(A|B|C|D|F)"]},"direction":{"fake":["regex","(North|South|East|West)"]}}}'>
</div>

## File System

### file_name

Generates a file name with extension:

```json
{"fake": ["file_name"]}
```

### file_extension

Generates a file extension:

```json
{"fake": ["file_extension"]}
```

### file_path

Generates a file path:

```json
{"fake": ["file_path"]}
```

### dir_path

Generates a directory path:

```json
{"fake": ["dir_path"]}
```

<div class="playground-widget" data-config='{"schema":{"fileName":{"fake":["file_name"]},"extension":{"fake":["file_extension"]},"filePath":{"fake":["file_path"]},"dirPath":{"fake":["dir_path"]}}}'>
</div>

## Barcodes

### isbn10

Generates an ISBN-10:

```json
{"fake": ["isbn10"]}
```

### isbn13

Generates an ISBN-13:

```json
{"fake": ["isbn13"]}
```

<div class="playground-widget" data-config='{"schema":{"isbn10":{"fake":["isbn10"]},"isbn13":{"fake":["isbn13"]}}}'>
</div>

## Complete Examples

### Feature Flags

<div class="playground-widget" data-config='{"schema":{"userId":{"fake":["uuid"]},"features":{"darkMode":{"fake":["bool"]},"betaAccess":{"fake":["bool"]},"notifications":{"fake":["bool"]},"tier":{"fake":["enum","free","basic","premium","enterprise"]}}}}'>
</div>

### E-commerce Product

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"name":{"fake":["words",3]},"price":{"fake":["f64",9.99,499.99]},"inStock":{"fake":["bool"]},"category":{"fake":["enum","Electronics","Clothing","Books","Home","Sports"]},"size":{"fake":["choice","XS","S","M","L","XL"]},"rating":{"fake":["f32",1.0,5.0]}}}'>
</div>

### Library Book

<div class="playground-widget" data-config='{"schema":{"isbn":{"fake":["isbn13"]},"title":{"fake":["sentence",2,5]},"author":{"fake":["name"]},"available":{"fake":["bool"]},"category":{"fake":["enum","Fiction","Non-Fiction","Science","History","Biography"]}}}'>
</div>
