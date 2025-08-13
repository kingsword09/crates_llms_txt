# crates_llms_txt

[![Crates.io][crates-src]][crates-href]
[![crates downloads][crates-download-src]][crates-download-href]
[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

A repository for generating content for `llms.txt` and `llms-full.txt` files used by Rust libraries.

## Package Distribution

This repository provides two main distribution formats/packages:

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
crates_llms_txt = "0.0.8" # Replace with the latest version
```

Then, you can use its functions in your Rust code. For example, to fetch documentation (conceptual):

```rust
use crates_llms_txt::get_llms_config_online;

async fn main() {
    match get_llms_config_online("clap", Some("4.5.39".to_string())).await {
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

#### API

This section details the methods available on the `LLMsStandardConfig` struct. To use these, you would typically first obtain or create an `LLMsStandardConfig` instance if needed, or call static/async methods directly if applicable. The primary public interface also includes top-level functions like `get_llms_config_online` (shown in the example above) which often wrap these methods.

##### `LLMsStandardConfig::new(lib_name: &str, version: &str) -> Self`

Creates a new `LLMsStandardConfig` instance. This function is used internally to initialize a configuration object.

- `lib_name: &str`: The name of the crate.
- `version: &str`: The version of the crate.
- **Returns:** `Self` - A new `LLMsStandardConfig` instance.

##### `async LLMsStandardConfig::get_llms_config_online(lib_name: &str, version: Option<String>) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Get the LLM config for a given crate and version by fetching documentation from online sources (docs.rs).

- `lib_name: &str`: The name of the crate.
- `version: Option<String>`: The version of the crate. If `None`, the latest version will be used.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The LLM config for the crate, with sessions and full_sessions serialized as JSON strings.

##### `async LLMsStandardConfig::get_llms_config_online_by_url(url: &str) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Get the LLM config for a given crate by providing a direct URL to the `cratename/latest/json` documentation endpoint.

- `url: &str`: The direct URL to the crate's JSON documentation index (e.g., "https://docs.rs/crate/clap/latest/json").
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The LLM config for the crate, with sessions and full_sessions serialized as JSON strings.

##### `LLMsStandardConfig::get_llms_config_offline_with_all_features(toolchain: &str, manifest_path: PathBuf) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Generate documentation for a crate using offline mode with all features enabled. This function invokes `cargo doc` locally.

- `toolchain: &str`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifest_path: PathBuf`: Path to the `Cargo.toml` file of the crate.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config, with sessions and full_sessions serialized as JSON strings.

_Note: This function is only available when the `rustdoc` feature is enabled._

##### `LLMsStandardConfig::get_llms_config_offline_with_features(toolchain: &str, manifest_path: PathBuf, no_default_features: bool, features: Option<Vec<String>>) -> Result<LLMsStandardStringConfig, Box<dyn Error>>`

Generate documentation for a crate using offline mode with specified features enabled. This function invokes `cargo doc` locally.

- `toolchain: &str`: The Rust toolchain to use (e.g., "stable", "nightly").
- `manifest_path: PathBuf`: Path to the `Cargo.toml` file of the crate.
- `no_default_features: bool`: Whether to disable the default features.
- `features: Option<Vec<String>>`: List of features to enable.
- **Returns:** `Result<LLMsStandardStringConfig, Box<dyn Error>>` - The generated documentation config, with sessions and full_sessions serialized as JSON strings.

_Note: This function is only available when the `rustdoc` feature is enabled._

For detailed API usage, including the top-level helper functions, please refer to the documentation within the `rs-lib` directory or on [crates.io](https://crates.io/crates/crates_llms_txt).

### NAPI Package: `crates-llms-txt-napi`

The `crates-llms-txt-napi` library is the Node.js/TypeScript implementation, distributed via npm.

- **NPM Package:** [crates-llms-txt-napi](https://www.npmjs.com/package/crates-llms-txt-napi)
- **Source:** `napi/`

**Description:** This library provides a standard interface to fetch and parse Rust crate documentation and session data for use with LLMs (Large Language Models).

**Features:**

- Generate metadata for `llms.txt` and `llms-full.txt` files
- Parse Rust crate documentation into standardized formats
- Support both online (docs.rs) and local crate documentation
- Enable custom feature selection for local documentation generation
- Cross-platform support with prebuilt binaries

**Installation:**

```bash
npm install crates-llms-txt-napi
```

**Usage Examples:**

```typescript
import { fromCrateName, fromLocal, fromLocalWithFeatures } from 'crates-llms-txt-napi'

// Fetch latest version from docs.rs
const config = await fromCrateName('clap')

// Fetch specific version
const specificConfig = await fromCrateName('clap', '4.5.39')

// Generate local documentation with all features
const localConfig = fromLocal('./Cargo.toml', 'stable')

// Generate with specific features
const customConfig = fromLocalWithFeatures(
  './Cargo.toml',
  true, // no default features
  ['async', 'serde'], // specific features
  'nightly' // toolchain
)
```

**API Reference:**

#### Online Documentation Functions

##### `fromCrateName(libName: string, version?: string): Promise<LLMsConfig | null>`

Fetches Rust crate documentation from docs.rs by crate name and version.

- `libName: string`: The name of the crate as it appears on crates.io
- `version?: string`: Optional version string. If not provided, the latest version will be fetched
- **Returns:** `Promise<LLMsConfig | null>` - Documentation configuration or null if failed

##### `fromUrl(url: string): Promise<LLMsConfig | null>`

Fetches documentation from a direct URL to the JSON documentation.

- `url: string`: Direct URL to the crate's JSON documentation
- **Returns:** `Promise<LLMsConfig | null>` - Documentation configuration or null if failed

##### `fromOnline(params: LLMsConfigByCrate | LLMsConfigByUrl): Promise<LLMsConfig | null>`

Unified function for fetching documentation from online sources.

- `params`: Either `{ libName: string, version?: string }` or `{ url: string }`
- **Returns:** `Promise<LLMsConfig | null>` - Documentation configuration or null if failed

#### Local Documentation Functions

##### `fromLocal(manifestPath: string, toolchain?: string): LLMsConfig | null`

Generates documentation for a local crate with all features enabled.

- `manifestPath: string`: Path to the Cargo.toml file
- `toolchain?: string`: Optional Rust toolchain (e.g., "stable", "nightly")
- **Returns:** `LLMsConfig | null` - Documentation configuration or null if failed

##### `fromLocalWithFeatures(manifestPath: string, noDefaultFeatures: boolean, features?: string[], toolchain?: string): LLMsConfig | null`

Generates documentation with fine-grained feature control.

- `manifestPath: string`: Path to the Cargo.toml file
- `noDefaultFeatures: boolean`: Whether to disable default features
- `features?: string[]`: Optional array of features to enable
- `toolchain?: string`: Optional Rust toolchain
- **Returns:** `LLMsConfig | null` - Documentation configuration or null if failed

##### `fromLocalByRustdoc(params: LLMsConfigRustdocByAllFeatures | LLMsConfigRustdocByFeatures): LLMsConfig | null`

Unified function for local documentation generation with flexible configuration.

**TypeScript Types:**

```typescript
interface SessionItem {
  title: string
  description: string
  link: string
}

interface FullSessionItem {
  content: string
  link: string
}

interface LLMsConfig {
  libName: string
  version: string
  sessions: SessionItem[]
  fullSessions: FullSessionItem[]
}

interface LLMsConfigByCrate {
  libName: string
  version?: string
}

interface LLMsConfigByUrl {
  url: string
}

interface LLMsConfigRustdocByAllFeatures {
  toolchain?: string
  manifestPath: string
}

interface LLMsConfigRustdocByFeatures {
  toolchain?: string
  manifestPath: string
  noDefaultFeatures: boolean
  features?: string[]
}
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
[crates-download-src]: https://img.shields.io/crates/d/crates_llms_txt?label=crates-download
[crates-download-href]: https://crates.io/crates/crates_llms_txt
