import { build } from "jsr:@deno/dnt@^0.42.1";
import denoJson from "./deno.json" with { type: "json" };

if (import.meta.main) {
  await build({
    entryPoints: ["./mod.ts"],
    outDir: ".npm",
    shims: { deno: true },
    declaration: "separate",
    package: {
      name: "crates-llms-txt",
      version: denoJson.version,
      description: "A repository for generating content for llms.txt and llms-full.txt files used by Rust libraries.",
      license: "MIT",
      author: "Kingsword kingsword09 <kingsword09@gmail.com>",
      homepage: "https://github.com/kingsword09/crates_llms_txt#readme",
      bugs: { url: "https://github.com/kingsword09/crates_llms_txt/issues" },
      repository: { type: "git", url: "git+https://github.com/kingsword09/crates_llms_txt.git" },
    },
    postBuild() {
      Deno.copyFileSync("../LICENSE", ".npm/LICENSE");
      Deno.copyFileSync("../jsr/README.md", ".npm/README.md");
    },
  });
}
