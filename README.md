# SSL Certificate Checker

A command-line tool to check and display SSL certificate information for any domain.

## Features

- Displays certificate subject and issuer information
- Shows validity period
- Shows public key information
- Supports JSON output format
- Easy to use command-line interface

## Installation

### Prerequisites

- Rust toolchain (rustc, cargo)
- OpenSSL development libraries

To install from source:

```bash
git clone https://github.com/yourusername/ssl-checker.git
cd ssl-checker
cargo build --release
```
## Usage

Basic usage:
```bash
ssl-checker <domain> [--json]
```

Options:
- `--json`: Output certificate information in JSON format

### Examples

Standard text output:
```bash
ssl-checker example.com
```

JSON format output:
```bash
ssl-checker example.com --json
```

The text output will display information such as:
This will display information such as:
- Certificate subject (organization, common name)
- Certificate issuer
- Validity period
- Public key details

Example output:
```
--- Certificate Information ---
Subject:
  C = US
  ST = California
  L = Los Angeles
  O = Internet Corporation for Assigned Names and Numbers
  CN = *.example.com
Issuer:
  C = US
  O = DigiCert Inc
  CN = DigiCert Global G3 TLS ECC SHA384 2020 CA1
Validity Period:
  Not Before: Jan 15 00:00:00 2025 GMT
  Not After:  Jan 15 23:59:59 2026 GMT
```

JSON format output example:
```json
{
  "subject": [
    { "key": "C", "value": "US" },
    { "key": "ST", "value": "California" },
    { "key": "L", "value": "Los Angeles" },
    { "key": "O", "value": "Internet Corporation for Assigned Names and Numbers" },
    { "key": "CN", "value": "*.example.com" }
  ],
  "issuer": [
    { "key": "C", "value": "US" },
    { "key": "O", "value": "DigiCert Inc" },
    { "key": "CN", "value": "DigiCert Global G3 TLS ECC SHA384 2020 CA1" }
  ],
  "validity": {
    "not_before": "Jan 15 00:00:00 2025 GMT",
    "not_after": "Jan 15 23:59:59 2026 GMT"
  },
  "public_key": {
    "key_type": "RSA",
    "key_size": 2048,
    "modulus": "d1a2b3c4..."
  },
  "version": 3,
  "serial_number": "1234567890abcdef",
  "signature_algorithm": "sha256WithRSAEncryption"
}
```

## License

This project is licensed under either of:
- MIT license
- Apache License, Version 2.0

at your option.