# Fake Data Types Overview

datafake-rs supports over 50 different fake data types organized into categories. Each type is invoked using the `fake` operator:

```json
{"fake": ["type_name"]}
{"fake": ["type_name", arg1, arg2]}
```

## Categories

| Category | Types | Description |
|----------|-------|-------------|
| [Numeric](numeric.md) | 10 | Integers and floating-point numbers with optional ranges |
| [Personal](personal.md) | 7 | Names, titles, and personal information |
| [Address](address.md) | 12 | Street addresses, cities, countries, coordinates |
| [Internet](internet.md) | 11 | Emails, usernames, IPs, domains |
| [Company](company.md) | 9 | Company names, industries, buzzwords |
| [Finance](finance.md) | 10 | BIC, IBAN, credit cards, currencies |
| [Content](content.md) | 5 | Words, sentences, paragraphs, UUIDs |
| [Date & Time](datetime.md) | 5 | Dates, times, timestamps |
| [Other](other.md) | 8 | Booleans, files, barcodes, custom choices |

## Quick Reference

### Most Common Types

<div class="playground-widget" data-config='{"schema":{"uuid":{"fake":["uuid"]},"name":{"fake":["name"]},"email":{"fake":["email"]},"age":{"fake":["u8",18,65]},"city":{"fake":["city_name"]},"bool":{"fake":["bool"]}}}'>
</div>

### All Types by Category

#### Numeric
`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64`

#### Personal
`name`, `full_name`, `first_name`, `last_name`, `name_with_title`, `title`, `suffix`

#### Address
`street_address`, `street_name`, `street_suffix`, `city`, `city_name`, `state_name`, `state_abbr`, `country_name`, `country_code`, `zip_code`, `post_code`, `latitude`, `longitude`

#### Internet
`email`, `safe_email`, `free_email`, `username`, `password`, `domain_name`, `domain_suffix`, `ipv4`, `ipv6`, `mac_address`, `user_agent`

#### Company
`company_name`, `company_suffix`, `industry`, `profession`, `catch_phrase`, `bs`, `bs_adj`, `bs_noun`, `bs_verb`

#### Finance
`bic`, `bic8`, `bic11`, `credit_card_number`, `currency_code`, `currency_name`, `currency_symbol`, `iban`, `lei`, `alphanumeric`

#### Content
`uuid`, `word`, `words`, `sentence`, `paragraph`

#### Date & Time
`datetime`, `iso8601_datetime`, `date`, `time`, `month_name`

#### Other
`bool`, `boolean`, `enum`, `pick`, `choice`, `regex`, `file_name`, `file_extension`, `file_path`, `dir_path`, `isbn10`, `isbn13`
