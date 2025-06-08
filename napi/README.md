# [crates-llms-txt-napi](https://www.npmjs.com/package/crates-llms-txt-napi)

[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

## **Features**

- Generate metadata for `llms.txt`、 `llms-full.txt` files
- Parse Rust crate documentation into standardized formats
- Support both online (docs.rs) and local crate documentation
- Enable custom feature selection for local documentation generation
- Cross-platform support with prebuilt binaries

## **Installation**

```bash
npm install crates-llms-txt-napi
```

## **Usage Example**

```ts
import { getLlmsConfigOnlineByCratesName } from 'crates-llms-txt-napi'

async function main() {
  const config = await getLlmsConfigOnlineByCratesName('clap', '4.5.39')
}

main()
```

## **API**

#### `getLlmsConfigOnlineByCratesName(libName: string, version?: string): Promise<LlMsConfig | null>`

Fetches the standard configuration for a given Rust crate and version from online sources (docs.rs).

- `libName: string`: The name of the crate (e.g., "clap").
- `version?: string`: The version string (optional, if not provided or `undefined`, the latest version of the crate will be attempted).
- **Returns:** `Promise<LlMsConfig | null>` - A promise that resolves to the `LlMsConfig` object or `null` if an error occurs (e.g., crate not found, network issue).

#### `getLlmsConfigOnlineByUrl(url: string): Promise<LlMsConfig | null>`

Fetches the standard configuration for a Rust crate by providing a direct URL to its `docs.rs` JSON documentation file.

- `url: string`: The direct URL to the crate's JSON documentation index (e.g., "https://docs.rs/crate/clap/latest/json").
- **Returns:** `Promise<LlMsConfig | null>` - A promise that resolves to the `LlMsConfig` object or `null` if an error occurs (e.g., URL not reachable, invalid JSON).

#### `getLlmsConfigByRustdocAllFeatures(toolchain: string, manifestPath: string): LlMsConfig | null`

Generates the LLM configuration for a local Rust crate by invoking `cargo doc` with all features enabled. This function requires a local Rust toolchain.

- `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifestPath: string`: Absolute or relative path to the `Cargo.toml` file of the local crate.
- **Returns:** `LlMsConfig | null` - The `LlMsConfig` object or `null` if an error occurs during documentation generation.

#### `getLlmsConfigByRustdocFeatures(toolchain: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[]): LlMsConfig | null`

Generates the LLM configuration for a local Rust crate by invoking `cargo doc` with specified features. This function requires a local Rust toolchain.

- `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifestPath: string`: Absolute or relative path to the `Cargo.toml` file of the local crate.
- `noDefaultFeatures: boolean`: If `true`, disables the default features of the crate.
- `features?: string[]`: An optional list of features to enable.
- **Returns:** `LlMsConfig | null` - The `LlMsConfig` object or `null` if an error occurs during documentation generation.

**`LlMsConfig` Type:**

```typescript
type SessionItem = { title: string; description: string; link: string }
type FullSessionItem = { content: string; link: string }

export type LlMsConfig = {
  libName: string
  version: string
  sessions: string /*SessionItem[]*/
  fullSessions: string /*FullSessionItem[]*/
}
```

## Supported Architectures

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

<!-- Badges -->

[npm-version-src]: https://img.shields.io/npm/v/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/crates-llms-txt-napi
[npm-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt-napi?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/crates-llms-txt-napi
[license-src]: https://img.shields.io/github/license/kingsword09/crates_llms_txt.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/kingsword09/crates_llms_txt/blob/main/LICENSE
