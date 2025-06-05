import { describe, test } from "node:test";
import { strictEqual } from "node:assert";
import path from "node:path";
import process from "node:process";
import { getLlmsConfigByRustdocFeatures } from "../src/index.ts";

describe("features", () => {
  test("features_success", () => {
    const config = getLlmsConfigByRustdocFeatures(
      "stable",
      path.resolve(process.cwd(), "../rs-lib/Cargo.toml"),
      false,
      ["rustdoc"]
    );
    strictEqual(config?.libName, "crates_llms_txt");
    strictEqual(config?.sessions.length > 0, true);
    strictEqual(config?.fullSessions.length > 0, true);
  });
});
