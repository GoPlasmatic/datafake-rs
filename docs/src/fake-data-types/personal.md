# Personal Data

Generate realistic personal information.

## Names

### name / full_name

Generates a complete name:

```json
{"fake": ["name"]}
{"fake": ["full_name"]}
```

<div class="playground-widget" data-config='{"schema":{"fullName":{"fake":["name"]}}}'>
</div>

### first_name

Generates a first name:

```json
{"fake": ["first_name"]}
```

<div class="playground-widget" data-config='{"schema":{"firstName":{"fake":["first_name"]}}}'>
</div>

### last_name

Generates a last name:

```json
{"fake": ["last_name"]}
```

<div class="playground-widget" data-config='{"schema":{"lastName":{"fake":["last_name"]}}}'>
</div>

### name_with_title

Generates a name with a professional title:

```json
{"fake": ["name_with_title"]}
```

<div class="playground-widget" data-config='{"schema":{"formalName":{"fake":["name_with_title"]}}}'>
</div>

### title

Generates a title/honorific (Mr., Mrs., Dr., etc.):

```json
{"fake": ["title"]}
```

<div class="playground-widget" data-config='{"schema":{"title":{"fake":["title"]}}}'>
</div>

### suffix

Generates a name suffix (Jr., Sr., III, etc.):

```json
{"fake": ["suffix"]}
```

<div class="playground-widget" data-config='{"schema":{"suffix":{"fake":["suffix"]}}}'>
</div>

## Complete User Profile

Combine personal data types for a complete profile:

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"title":{"fake":["title"]},"firstName":{"fake":["first_name"]},"lastName":{"fake":["last_name"]},"suffix":{"fake":["suffix"]},"email":{"fake":["email"]},"phone":{"fake":["phone_number"]}}}'>
</div>

## Phone Numbers

### phone_number

Generates a phone number:

```json
{"fake": ["phone_number"]}
```

### cell_number

Generates a cell phone number:

```json
{"fake": ["cell_number"]}
```

<div class="playground-widget" data-config='{"schema":{"phone":{"fake":["phone_number"]},"mobile":{"fake":["cell_number"]}}}'>
</div>
