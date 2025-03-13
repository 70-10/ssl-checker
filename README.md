# SSL Certificate Checker

A command-line tool to check and display SSL certificate information for any domain.

## Features

- Displays certificate subject and issuer information
- Shows validity period
- Shows public key information
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

```bash
ssl-checker <domain>
```

### Example

```bash
ssl-checker example.com
```

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

## License

This project is licensed under either of:
- MIT license
- Apache License, Version 2.0

at your option.