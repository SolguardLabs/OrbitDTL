import test from "node:test";
import assert from "node:assert/strict";
import { eventTypes, loadDemoReport } from "../helpers/orbit-cli.js";

test("el flujo del demo registra altas, cola, cancelacion y liquidaciones", () => {
  const report = loadDemoReport();
  const types = eventTypes(report);

  assert.deepEqual(types.slice(0, 4), [
    "asset_registered",
    "asset_registered",
    "vault_created",
    "vault_created",
  ]);
  assert.equal(types.filter((type) => type === "intent_queued").length, 3);
  assert.equal(types.filter((type) => type === "intent_settled").length, 2);
  assert.equal(types.filter((type) => type === "intent_cancelled").length, 1);
  assert.equal(types.at(-1), "intent_settled");
});

test("los eventos de liquidacion conservan importes netos y comisiones", () => {
  const report = loadDemoReport();
  const settled = report.events.filter((event) => event.type === "intent_settled");

  assert.equal(settled.length, 2);
  assert.equal(settled[0].gross_out, 11111111111);
  assert.equal(settled[0].net_out, 11088888889);
  assert.equal(settled[0].fee, 22222222);
  assert.equal(settled[1].gross_out, 9259259259);
  assert.equal(settled[1].net_out, 9240740741);
  assert.equal(settled[1].fee, 18518518);
});
