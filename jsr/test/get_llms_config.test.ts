import { assertEquals } from "jsr:@std/assert@^1.0.13";
import { assertSnapshot } from "jsr:@std/testing@^1.0.13/snapshot";
import { get_llms_standard_config } from "../mod.ts";

Deno.test("get_llms_config", async (t) => {
  const libName = "clap";
  const version = "4.5.39";
  const llmsStandardConfig = await get_llms_standard_config(libName, version);

  assertEquals(llmsStandardConfig!.libName, "clap");
  assertEquals(llmsStandardConfig!.version, "4.5.39");

  await t.step("sessions", async (t) => {
    await assertSnapshot(t, llmsStandardConfig!.sessions);
  });

  await t.step("full_sessions", async (t) => {
    await assertSnapshot(t, llmsStandardConfig!.fullSessions);
  });
});

Deno.test("get_llms_config_failed", async (_t) => {
  const libName = "opendal";
  const version = "0.53.3";
  const llmsStandardConfig = await get_llms_standard_config(libName, version);
  assertEquals(llmsStandardConfig, null);
});
