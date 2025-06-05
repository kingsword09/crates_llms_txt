# crates_llms_txt

[![Crates.io][crates-src]][crates-href]
[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

A repository for generating content for `llms.txt` and `llms-full.txt` files used by Rust libraries.

## Package Distribution

This repository provides two distribution formats:

### Rust Library: `crates_llms_txt`

The `crates_llms_txt` library is the native Rust implementation.

- **Crates.io:** [crates_llms_txt](https://crates.io/crates/crates_llms_txt)
- **Source:** `rs-lib/`

**Description:** An asynchronous Rust library for retrieving Rust crates documentation structure and LLM configurations.

**Features:**

- Asynchronously fetch crates documentation index.
- Parse documentation structure into Rust structs.
- Generate standard LLM configurations, including sessions and full_sessions.

**Usage:**

To use `crates_llms_txt` in your Rust project, add it as a dependency to your `Cargo.toml`:

```toml
[dependencies]
crates_llms_txt = "0.1.0" # Replace with the latest version
```

Then, you can use its functions in your Rust code. For example, to fetch documentation (conceptual):

```rust
use crates_llms_txt::get_llms_config_online;

async fn main() {
    match get_llms_config_online("clap", Some("4.5.39")).await {
        Ok(data) => {
            // Process data
            println!("Successfully fetched data.");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

For detailed API usage, please refer to the documentation within the `rs-lib` directory or on [crates.io](https://crates.io/crates/crates_llms_txt).

### NAPI Package: `crates-llms-txt-napi`

The `crates-llms-txt-napi` library is the JS/TS implementation, distributed via npm.

- **NPM Package:** [crates-llms-txt-napi](https://www.npmjs.com/package/crates-llms-txt-napi)
- **Source:** `napi/`

**Description:** This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

**Features:**

- Fetches documentation and session data for any Rust crate by name and version.
- Returns a unified configuration object for LLM consumption.
- TypeScript type definitions included.

**Installation:**

```bash
npm install crates-llms-txt-napi
```

**Usage Example:**

```typescript
import { getLlmsConfigOnline } from "crates-llms-txt-napi";

async function main() {
  const config = await getLlmsConfigOnline("clap", "4.5.39");
}

main();
```

**API:**

#### `getLlmsConfigOnline(libName: string, version?: string): Promise<LlMsConfig>`

Fetches the standard configuration for a given Rust crate and version.

- `libName: string`: The name of the crate (e.g., "clap").
- `version?: string`: The version string (optional, defaults to the latest version of the crate).
- **Returns:** `Promise<LlMsConfig>` - A promise that resolves to the LlMsConfig object.

**`LlMsConfig` Type:**

```typescript
type SessionItem = { title: string; description: string; link: string };
type FullSessionItem = { content: string; link: string };

export type LlMsConfig = {
  libName: string;
  version: string;
  sessions: string /*SessionItem[]*/;
  fullSessions: string /*FullSessionItem[]*/;
};
```

#### Supported Architectures

The `crates-llms-txt-napi` package provides prebuilt binaries for the following target architectures:

| Target Triple                   |
| ------------------------------- |
| `x86_64-apple-darwin`           |
| `aarch64-apple-darwin`          |
| `x86_64-pc-windows-msvc`        |
| `aarch64-pc-windows-msvc`       |
| `i686-pc-windows-msvc`          |
| `x86_64-unknown-linux-gnu`      |
| `x86_64-unknown-linux-musl`     |
| `aarch64-unknown-linux-gnu`     |
| `aarch64-unknown-linux-musl`    |
| `armv7-unknown-linux-gnueabihf` |

## License

MIT License

<!-- Badges -->

[npm-version-src]: https://img.shields.io/npm/v/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/crates-llms-txt-napi
[npm-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/crates-llms-txt-napi
[license-src]: https://img.shields.io/github/license/kingsword09/crates_llms_txt.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/kingsword09/crates_llms_txt/blob/main/LICENSE
[crates-src]: https://img.shields.io/crates/v/crates_llms_txt
[crates-href]: https://crates.io/crates/crates_llms_txt
