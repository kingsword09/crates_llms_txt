import {
  getLlmsConfigOnlineByCratesName as getOnlineByCratesName,
  getLlmsConfigOnlineByUrl as getOnlineByUrl,
  getLlmsConfigByRustdocAllFeatures as getByAllFeatures,
  getLlmsConfigByRustdocFeatures as getByFeatures,
} from "crates-llms-txt-napi";

import type { LLMsStandardConfig } from "./types.ts";
import { processLlmsConfig } from "./utils.ts";

/**
 * Get the LLMs config from the online API by crates name.
 *
 * @param libName The name of the library.
 * @param version The version of the library.
 * @returns The LLMs config.
 */
export const getLlmsConfigOnlineByCratesName = async (
  libName: string,
  version?: string
): Promise<LLMsStandardConfig | null> => {
  const config = await getOnlineByCratesName(libName, version);

  return processLlmsConfig(config);
};

/**
 * Get the LLMs config from the online API by url.
 *
 * @param url The url of the library.
 * @returns The LLMs config.
 */
export const getLlmsConfigOnlineByUrl = async (
  url: string
): Promise<LLMsStandardConfig | null> => {
  const config = await getOnlineByUrl(url);

  return processLlmsConfig(config);
};

/**
 * Get the LLMs config by rustdoc all features.
 *
 * @param toolchain The toolchain to use.
 * @param manifestPath The path to the Cargo.toml file.
 * @returns The LLMs config.
 */
export const getLlmsConfigByRustdocAllFeatures = (
  toolchain: string,
  manifestPath: string
): LLMsStandardConfig | null => {
  const config = getByAllFeatures(toolchain, manifestPath);

  return processLlmsConfig(config);
};

/**
 * Get the LLMs config by rustdoc features.
 *
 * @param toolchain The toolchain to use.
 * @param manifestPath The path to the Cargo.toml file.
 * @param noDefaultFeatures Whether to include default features.
 * @param features The features to include.
 * @returns The LLMs config.
 */
export const getLlmsConfigByRustdocFeatures = (
  toolchain: string,
  manifestPath: string,
  noDefaultFeatures: boolean,
  features?: string[]
): LLMsStandardConfig | null => {
  const config = getByFeatures(
    toolchain,
    manifestPath,
    noDefaultFeatures,
    features
  );

  return processLlmsConfig(config);
};

export type { LLMsStandardConfig, SessionItem, FullSessionItem } from "./types";
