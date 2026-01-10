# Date & Time Data

Generate dates, times, and timestamps.

## Timestamps

### datetime / iso8601_datetime

Generates an ISO 8601 formatted datetime:

```json
{"fake": ["datetime"]}
{"fake": ["iso8601_datetime"]}
```

<div class="playground-widget" data-config='{"schema":{"timestamp":{"fake":["datetime"]}}}'>
</div>

## Date

### date

Generates a date with optional format:

```json
{"fake": ["date"]}
{"fake": ["date", format]}
```

Default format is `%Y-%m-%d`.

<div class="playground-widget" data-config='{"schema":{"defaultDate":{"fake":["date"]},"usFormat":{"fake":["date","%m/%d/%Y"]},"europeanFormat":{"fake":["date","%d.%m.%Y"]}}}'>
</div>

### Common Date Formats

| Format | Example |
|--------|---------|
| `%Y-%m-%d` | 2024-01-15 |
| `%m/%d/%Y` | 01/15/2024 |
| `%d.%m.%Y` | 15.01.2024 |
| `%B %d, %Y` | January 15, 2024 |
| `%Y%m%d` | 20240115 |

## Time

### time

Generates a time in HH:MM:SS format:

```json
{"fake": ["time"]}
```

<div class="playground-widget" data-config='{"schema":{"time":{"fake":["time"]}}}'>
</div>

## Month

### month_name

Generates a random month name:

```json
{"fake": ["month_name"]}
```

<div class="playground-widget" data-config='{"schema":{"month":{"fake":["month_name"]}}}'>
</div>

## Complete Examples

### Event

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"title":{"fake":["sentence",3,6]},"date":{"fake":["date","%B %d, %Y"]},"time":{"fake":["time"]},"createdAt":{"fake":["datetime"]}}}'>
</div>

### Audit Log

<div class="playground-widget" data-config='{"schema":{"action":"user.login","userId":{"fake":["uuid"]},"timestamp":{"fake":["datetime"]},"metadata":{"ip":{"fake":["ipv4"]},"userAgent":{"fake":["user_agent"]}}}}'>
</div>

### Schedule

<div class="playground-widget" data-config='{"schema":{"event":{"fake":["catch_phrase"]},"month":{"fake":["month_name"]},"startTime":{"fake":["time"]},"endTime":{"fake":["time"]}}}'>
</div>
