# Address Data

Generate realistic address components and geographic data.

## Street Address

### street_address

Generates a full street address:

```json
{"fake": ["street_address"]}
```

<div class="playground-widget" data-config='{"schema":{"address":{"fake":["street_address"]}}}'>
</div>

### street_name

Generates just the street name:

```json
{"fake": ["street_name"]}
```

### street_suffix

Generates a street suffix (Street, Avenue, Lane, etc.):

```json
{"fake": ["street_suffix"]}
```

<div class="playground-widget" data-config='{"schema":{"streetName":{"fake":["street_name"]},"suffix":{"fake":["street_suffix"]}}}'>
</div>

## City and State

### city / city_name

Generates a city name:

```json
{"fake": ["city"]}
{"fake": ["city_name"]}
```

<div class="playground-widget" data-config='{"schema":{"city":{"fake":["city_name"]}}}'>
</div>

### state_name

Generates a US state name:

```json
{"fake": ["state_name"]}
```

### state_abbr

Generates a US state abbreviation:

```json
{"fake": ["state_abbr"]}
```

<div class="playground-widget" data-config='{"schema":{"state":{"fake":["state_name"]},"stateCode":{"fake":["state_abbr"]}}}'>
</div>

## Country

### country_name

Generates a country name:

```json
{"fake": ["country_name"]}
```

### country_code

Generates a country code:

```json
{"fake": ["country_code"]}
```

<div class="playground-widget" data-config='{"schema":{"country":{"fake":["country_name"]},"countryCode":{"fake":["country_code"]}}}'>
</div>

## Postal Codes

### zip_code / zip

Generates a US ZIP code:

```json
{"fake": ["zip_code"]}
{"fake": ["zip"]}
```

### post_code / postcode / postal_code

Generates a postal code:

```json
{"fake": ["post_code"]}
{"fake": ["postcode"]}
{"fake": ["postal_code"]}
```

<div class="playground-widget" data-config='{"schema":{"zipCode":{"fake":["zip_code"]},"postalCode":{"fake":["post_code"]}}}'>
</div>

## Geographic Coordinates

### latitude

Generates a latitude value (-90 to 90):

```json
{"fake": ["latitude"]}
```

### longitude

Generates a longitude value (-180 to 180):

```json
{"fake": ["longitude"]}
```

<div class="playground-widget" data-config='{"schema":{"lat":{"fake":["latitude"]},"lng":{"fake":["longitude"]}}}'>
</div>

## Complete Address

<div class="playground-widget" data-config='{"schema":{"street":{"fake":["street_address"]},"city":{"fake":["city_name"]},"state":{"fake":["state_abbr"]},"zipCode":{"fake":["zip_code"]},"country":{"fake":["country_name"]},"coordinates":{"lat":{"fake":["latitude"]},"lng":{"fake":["longitude"]}}}}'>
</div>
