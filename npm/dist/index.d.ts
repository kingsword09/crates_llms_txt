//#region src/types.d.ts
/**
* SessionItem is the session item.
* @property title - The title of the session.
* @property description - The description of the session.
* @property link - The link of the session.
*/
type SessionItem = {
  title: string;
  description: string;
  link: string;
};
/**
* FullSessionItem is the full session item.
* @property content - The content of the session.
* @property link - The link of the session.
*/
type FullSessionItem = {
  content: string;
  link: string;
};
/**
* LLMsStandardConfig is the standard config for LLMs.
* @property libName - The name of the library.
* @property version - The version of the library.
* @property sessions - The sessions of the library.
* @property fullSessions - The full sessions of the library.
*/
type LLMsStandardConfig = {
  libName: string;
  version: string;
  sessions: SessionItem[];
  fullSessions: FullSessionItem[];
};
//#endregion
//#region src/index.d.ts
/**
* Get the LLMs config from the online API.
*
* @param libName The name of the library.
* @param version The version of the library.
* @returns The LLMs config.
*/
declare const getLlmsConfigOnline: (libName: string, version?: string) => Promise<LLMsStandardConfig | null>;
/**
* Get the LLMs config by rustdoc all features.
*
* @param toolchain The toolchain to use.
* @param manifestPath The path to the Cargo.toml file.
* @returns The LLMs config.
*/
declare const getLlmsConfigByRustdocAllFeatures: (toolchain: string, manifestPath: string) => LLMsStandardConfig | null;
/**
* Get the LLMs config by rustdoc features.
*
* @param toolchain The toolchain to use.
* @param manifestPath The path to the Cargo.toml file.
* @param noDefaultFeatures Whether to include default features.
* @param features The features to include.
* @returns The LLMs config.
*/
declare const getLlmsConfigByRustdocFeatures: (toolchain: string, manifestPath: string, noDefaultFeatures: boolean, features?: string[]) => LLMsStandardConfig | null;
//#endregion
export { FullSessionItem, LLMsStandardConfig, SessionItem, getLlmsConfigByRustdocAllFeatures, getLlmsConfigByRustdocFeatures, getLlmsConfigOnline };