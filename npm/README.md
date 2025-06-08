# [crates-llms-txt](https://www.npmjs.com/package/crates-llms-txt)

[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![License][license-src]][license-href]

This is a higher-level JavaScript/TypeScript wrapper over the `crates-llms-txt-napi` package. It provides a more user-friendly interface by automatically parsing the JSON strings for `sessions` and `fullSessions` into structured JavaScript objects.

## **Installation**

```bash
npm install crates-llms-txt
# or
yarn add crates-llms-txt
# or
pnpm add crates-llms-txt
```

## **Usage Example**

```typescript
import { getLlmsConfigOnlineByCratesName } from "crates-llms-txt";

async function main() {
  const config = await getLlmsConfigOnlineByCratesName("tokio", "1"); // Example: Get latest 1.x for tokio
  if (config) {
    console.log(`Fetched config for ${config.libName} v${config.version}`);
    config.sessions.forEach(session => {
      console.log(`  Session: ${session.title} - ${session.link}`);
    });
    // config.fullSessions.forEach(session => {
    //   console.log(`  Full Session Content (first 50 chars): ${session.content.substring(0, 50)}...`);
    // });
  } else {
    console.log("Failed to fetch LLMs config.");
  }
}

main();
```

## API

All functions return `null` if the underlying NAPI call fails or if the JSON parsing of session data is unsuccessful.

##### `getLlmsConfigOnlineByCratesName(libName: string, version?: string): Promise<LLMsStandardConfig | null>`

Get the LLMs config from the online API by crates name. `sessions` and `fullSessions` are parsed into objects.

-   `libName: string`: The name of the library.
-   `version?: string`: The version of the library. (Optional, defaults to latest if not specified).
-   **Returns:** `Promise<LLMsStandardConfig | null>` - The LLMs config with parsed sessions, or `null` on error.

##### `getLlmsConfigOnlineByUrl(url: string): Promise<LLMsStandardConfig | null>`

Get the LLMs config from the online API by URL. `sessions` and `fullSessions` are parsed into objects.

-   `url: string`: The direct URL to the crate's JSON documentation index.
-   **Returns:** `Promise<LLMsStandardConfig | null>` - The LLMs config with parsed sessions, or `null` on error.

##### `getLlmsConfigByRustdocAllFeatures(toolchain: string, manifestPath: string): LLMsStandardConfig | null`

Get the LLMs config by generating documentation locally using `rustdoc` with all features enabled. `sessions` and `fullSessions` are parsed into objects. Requires a local Rust toolchain.

-   `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
-   `manifestPath: string`: Path to the `Cargo.toml` file of the crate.
-   **Returns:** `LLMsStandardConfig | null` - The LLMs config with parsed sessions, or `null` on error.

##### `getLlmsConfigByRustdocFeatures(toolchain: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[]): LLMsStandardConfig | null`

Get the LLMs config by generating documentation locally using `rustdoc` with specified features. `sessions` and `fullSessions` are parsed into objects. Requires a local Rust toolchain.

-   `toolchain: string`: The Rust toolchain to use (e.g., "stable", "nightly").
-   `manifestPath: string`: Path to the `Cargo.toml` file of the crate.
-   `noDefaultFeatures: boolean`: Whether to disable the default features.
-   `features?: string[]`: Optional list of features to enable.
-   **Returns:** `LLMsStandardConfig | null` - The LLMs config with parsed sessions, or `null` on error.

#### Types

These are the primary types returned by the API functions.

```typescript
/**
 * SessionItem is the session item.
 */
export type SessionItem = {
  title: string;
  description: string;
  link: string;
};

/**
 * FullSessionItem is the full session item.
 */
export type FullSessionItem = {
  content: string;
  link: string;
};

/**
 * LLMsStandardConfig is the standard config for LLMs.
 * It contains the library name, version, and parsed session data.
 */
export type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};
```

<!-- Badges -->

[npm-version-src]: https://img.shields.io/npm/v/crates-llms-txt?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/crates-llms-txt
[npm-downloads-src]: https://img.shields.io/npm/dm/crates-llms-txt?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/crates-llms-txt
[license-src]: https://img.shields.io/github/license/kingsword09/crates_llms_txt.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/kingsword09/crates_llms_txt/blob/main/LICENSE
