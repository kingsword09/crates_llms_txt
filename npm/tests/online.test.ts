import { test, describe } from "node:test";
import { strictEqual } from "node:assert";
import { getLlmsConfigOnlineByCratesName } from "../src/index.ts";

describe("online", () => {
  test("online_success", async () => {
    const config = await getLlmsConfigOnlineByCratesName("clap", "4.5.39");
    strictEqual(config?.libName, "clap");
    strictEqual(config?.version, "4.5.39");
    strictEqual(config?.sessions.length > 0, true);
    strictEqual(config?.fullSessions.length > 0, true);
  });

  // test("online_snapshot", async (t) => {
  //   const config = await getLlmsConfigOnline("clap", "4.5.39");
  //   t.assert.fileSnapshot(config, "./tests/__snapshots__/online.test.ts.snap");
  // })

  test("online_failure", async () => {
    const config = await getLlmsConfigOnlineByCratesName("xxxxxx", "4.5.39");
    strictEqual(config, null);
  });
});
