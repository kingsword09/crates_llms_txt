import * as path from "jsr:@std/path@^1.1.0";
import { get_llms_standard_config_by_rustdoc_all_features } from "../mod.ts";

Deno.test("get_llms_config_rustdoc", async () => {
  console.log("Deno.cwd();");
  const config = await get_llms_standard_config_by_rustdoc_all_features(
    "stable",
    path.join(Deno.cwd(), "rs-lib/Cargo.toml"),
  );
  console.log(config);
});
