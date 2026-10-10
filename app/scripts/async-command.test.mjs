import assert from "node:assert/strict";
import test from "node:test";
import { invokeWithTimeout } from "../src/ui/asyncCommand.ts";

test("reports which local command stopped responding instead of leaving study busy forever", async () => {
  const stalled = new Promise(() => {});

  await assert.rejects(
    invokeWithTimeout("read_local_document", () => stalled, 5),
    /read_local_document.*não respondeu/i,
  );
});
