# smartcardfyi

[![crates.io](https://img.shields.io/crates/v/smartcardfyi)](https://crates.io/crates/smartcardfyi)
[![docs.rs](https://docs.rs/smartcardfyi/badge.svg)](https://docs.rs/smartcardfyi)

Async Rust client for the [SmartCardFYI](https://smartcardfyi.com) API. Look up smart card platforms (Java Card, MULTOS), EMV standards, form factors, certifications, and manufacturer specifications.

## Install

```toml
[dependencies]
smartcardfyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use smartcardfyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("emv").await?;
    println!("Found {} results", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search cards, platforms, and glossary |
| `card(slug)` | Get smart card details |
| `platform(slug)` | Get platform details |
| `standard(slug)` | Get standard details |
| `manufacturer(slug)` | Get manufacturer details |
| `application(slug)` | Get application details |
| `form_factor(slug)` | Get form factor details |
| `certification(slug)` | Get certification details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two smart cards |
| `random()` | Get a random smart card |

All methods are async and return `Result<T, SmartCardFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [smartcardfyi](https://pypi.org/project/smartcardfyi/) | `pip install smartcardfyi` |
| TypeScript | [smartcardfyi](https://www.npmjs.com/package/smartcardfyi) | `npm install smartcardfyi` |
| Go | [smartcardfyi-go](https://pkg.go.dev/github.com/fyipedia/smartcardfyi-go) | `go get github.com/fyipedia/smartcardfyi-go` |
| Rust | [smartcardfyi](https://crates.io/crates/smartcardfyi) | `cargo add smartcardfyi` |
| Ruby | [smartcardfyi](https://rubygems.org/gems/smartcardfyi) | `gem install smartcardfyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
