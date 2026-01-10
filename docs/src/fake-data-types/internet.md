# Internet Data

Generate internet-related fake data like emails, usernames, IPs, and more.

## Email Addresses

### email / safe_email

Generates a safe email address (uses example.com domains):

```json
{"fake": ["email"]}
{"fake": ["safe_email"]}
```

<div class="playground-widget" data-config='{"schema":{"email":{"fake":["email"]}}}'>
</div>

### free_email

Generates an email from free email providers:

```json
{"fake": ["free_email"]}
```

<div class="playground-widget" data-config='{"schema":{"freeEmail":{"fake":["free_email"]}}}'>
</div>

## Usernames and Passwords

### username

Generates a username:

```json
{"fake": ["username"]}
```

<div class="playground-widget" data-config='{"schema":{"username":{"fake":["username"]}}}'>
</div>

### password

Generates a password with configurable length:

```json
{"fake": ["password"]}
{"fake": ["password", min_length, max_length]}
```

<div class="playground-widget" data-config='{"schema":{"defaultPassword":{"fake":["password"]},"strongPassword":{"fake":["password",16,24]}}}'>
</div>

## Domains

### domain_name

Generates a domain name:

```json
{"fake": ["domain_name"]}
```

### domain_suffix

Generates a domain suffix (com, org, net, etc.):

```json
{"fake": ["domain_suffix"]}
```

<div class="playground-widget" data-config='{"schema":{"domain":{"fake":["domain_name"]},"tld":{"fake":["domain_suffix"]}}}'>
</div>

## IP Addresses

### ipv4

Generates an IPv4 address:

```json
{"fake": ["ipv4"]}
```

### ipv6

Generates an IPv6 address:

```json
{"fake": ["ipv6"]}
```

<div class="playground-widget" data-config='{"schema":{"ipv4":{"fake":["ipv4"]},"ipv6":{"fake":["ipv6"]}}}'>
</div>

## Network

### mac_address

Generates a MAC address:

```json
{"fake": ["mac_address"]}
```

### user_agent

Generates a browser user agent string:

```json
{"fake": ["user_agent"]}
```

<div class="playground-widget" data-config='{"schema":{"macAddress":{"fake":["mac_address"]},"userAgent":{"fake":["user_agent"]}}}'>
</div>

## Complete Network Profile

<div class="playground-widget" data-config='{"schema":{"hostname":{"fake":["domain_name"]},"ipv4":{"fake":["ipv4"]},"ipv6":{"fake":["ipv6"]},"macAddress":{"fake":["mac_address"]},"credentials":{"username":{"fake":["username"]},"password":{"fake":["password",12,20]}}}}'>
</div>
