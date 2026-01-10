# Company Data

Generate business-related fake data.

## Company Names

### company_name

Generates a company name:

```json
{"fake": ["company_name"]}
```

<div class="playground-widget" data-config='{"schema":{"company":{"fake":["company_name"]}}}'>
</div>

### company_suffix

Generates a company suffix (Inc., LLC, Corp., etc.):

```json
{"fake": ["company_suffix"]}
```

<div class="playground-widget" data-config='{"schema":{"suffix":{"fake":["company_suffix"]}}}'>
</div>

## Industry and Profession

### industry

Generates an industry name:

```json
{"fake": ["industry"]}
```

### profession

Generates a profession/job title:

```json
{"fake": ["profession"]}
```

<div class="playground-widget" data-config='{"schema":{"industry":{"fake":["industry"]},"profession":{"fake":["profession"]}}}'>
</div>

## Marketing Buzzwords

### catch_phrase

Generates a company catch phrase:

```json
{"fake": ["catch_phrase"]}
```

<div class="playground-widget" data-config='{"schema":{"catchPhrase":{"fake":["catch_phrase"]}}}'>
</div>

### bs (Business Speak)

Generates business jargon:

```json
{"fake": ["bs"]}
```

### bs_adj

Generates a business adjective:

```json
{"fake": ["bs_adj"]}
```

### bs_noun

Generates a business noun:

```json
{"fake": ["bs_noun"]}
```

### bs_verb

Generates a business verb:

```json
{"fake": ["bs_verb"]}
```

<div class="playground-widget" data-config='{"schema":{"buzzword":{"fake":["bs"]},"adjective":{"fake":["bs_adj"]},"noun":{"fake":["bs_noun"]},"verb":{"fake":["bs_verb"]}}}'>
</div>

## Complete Company Profile

<div class="playground-widget" data-config='{"schema":{"name":{"fake":["company_name"]},"industry":{"fake":["industry"]},"catchPhrase":{"fake":["catch_phrase"]},"description":{"fake":["bs"]},"website":{"fake":["domain_name"]},"email":{"cat":["info@",{"fake":["domain_name"]}]}}}'>
</div>
