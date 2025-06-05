import { defineConfig } from "tsdown";
import path from "node:path";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm", "cjs"],
  target: "es2022",
  minify: true,
  dts: { isolatedDeclarations: true },
});
