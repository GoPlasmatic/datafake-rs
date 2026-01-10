# Finance Data

Generate financial data for testing payment systems and banking applications.

## Bank Identifiers

### bic

Generates a BIC (Bank Identifier Code), randomly 8 or 11 characters:

```json
{"fake": ["bic"]}
{"fake": ["bic", 8]}   // Force 8 characters
{"fake": ["bic", 11]}  // Force 11 characters
```

<div class="playground-widget" data-config='{"schema":{"bic":{"fake":["bic"]}}}'>
</div>

### bic8

Generates an 8-character BIC:

```json
{"fake": ["bic8"]}
```

### bic11

Generates an 11-character BIC (with branch code):

```json
{"fake": ["bic11"]}
```

<div class="playground-widget" data-config='{"schema":{"bic8":{"fake":["bic8"]},"bic11":{"fake":["bic11"]}}}'>
</div>

### iban

Generates an IBAN with optional country code:

```json
{"fake": ["iban"]}
{"fake": ["iban", "DE"]}  // German IBAN
{"fake": ["iban", "FR"]}  // French IBAN
```

<div class="playground-widget" data-config='{"schema":{"ibanDE":{"fake":["iban","DE"]},"ibanFR":{"fake":["iban","FR"]},"ibanDefault":{"fake":["iban"]}}}'>
</div>

### lei

Generates a Legal Entity Identifier:

```json
{"fake": ["lei"]}
```

<div class="playground-widget" data-config='{"schema":{"lei":{"fake":["lei"]}}}'>
</div>

## Credit Cards

### credit_card_number

Generates a credit card number:

```json
{"fake": ["credit_card_number"]}
```

<div class="playground-widget" data-config='{"schema":{"cardNumber":{"fake":["credit_card_number"]}}}'>
</div>

## Currency

### currency_code

Generates a currency code (USD, EUR, etc.):

```json
{"fake": ["currency_code"]}
```

### currency_name

Generates a currency name:

```json
{"fake": ["currency_name"]}
```

### currency_symbol

Generates a currency symbol:

```json
{"fake": ["currency_symbol"]}
```

<div class="playground-widget" data-config='{"schema":{"code":{"fake":["currency_code"]},"name":{"fake":["currency_name"]},"symbol":{"fake":["currency_symbol"]}}}'>
</div>

## Alphanumeric

### alphanumeric

Generates an alphanumeric string (useful for reference numbers):

```json
{"fake": ["alphanumeric", length]}
{"fake": ["alphanumeric", min_length, max_length]}
```

<div class="playground-widget" data-config='{"schema":{"referenceNumber":{"fake":["alphanumeric",12]},"trackingCode":{"fake":["alphanumeric",8,16]}}}'>
</div>

## Complete Transaction

<div class="playground-widget" data-config='{"schema":{"transactionId":{"fake":["uuid"]},"amount":{"fake":["f64",10.0,10000.0]},"currency":{"fake":["currency_code"]},"fromAccount":{"iban":{"fake":["iban","DE"]},"bic":{"fake":["bic8"]}},"toAccount":{"iban":{"fake":["iban","FR"]},"bic":{"fake":["bic11"]}},"reference":{"fake":["alphanumeric",16]},"timestamp":{"fake":["datetime"]}}}'>
</div>
