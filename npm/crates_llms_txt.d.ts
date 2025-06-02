// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

/**
 * Get the LLM config for a given crate and version.
 *
 * # Arguments
 *
 * * `lib_name` - The name of the crate.
 * * `version` - The version of the crate. If None, the latest version will be used.
 *
 * # Returns
 *
 * * `Result<LLMsStandardConfig, Box<dyn std::error::Error>>` - The LLM config for the crate.
 *
 * # Examples
 *
 * ```
 * let config = LLMsStandardConfig::get_llms_config("clap", Some("4.5.39")).await.unwrap();
 * ```
 */
export function get_llms_config(
  lib_name: string,
  version?: string | null,
): Promise<any>;
