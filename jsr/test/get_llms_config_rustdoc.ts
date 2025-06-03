import { get_llms_standard_config_by_rustdoc_all_features } from "../mod.ts";

Deno.test("get_llms_config_rustdoc", async () => {
  const config = await get_llms_standard_config_by_rustdoc_all_features("stable", "Cargo.toml");
  console.log(config);
});
