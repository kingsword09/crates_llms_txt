import { describe, test } from "node:test";
import { strictEqual } from "node:assert";
import path from "node:path";
import process from "node:process";
import { getLlmsConfigByRustdocAllFeatures } from "../src/index.ts";

describe("all_features", () => {
  test("all_features_success", () => {
    const config = getLlmsConfigByRustdocAllFeatures(
      "stable",
      path.resolve(process.cwd(), "../rs-lib/Cargo.toml")
    );
    strictEqual(config?.libName, "crates_llms_txt");
    strictEqual(config?.sessions.length > 0, true);
    strictEqual(config?.fullSessions.length > 0, true);
  });
});
