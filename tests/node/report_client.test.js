import test from "node:test";
import assert from "node:assert/strict";
import { summarizeReport } from "../../client/report-client.mjs";
import { loadDemoReport } from "../helpers/orbit-cli.js";

test("el cliente agrega vaults y settlements por activo", () => {
  const summary = summarizeReport(loadDemoReport());
  assert.equal(summary.sessions, 1);
  assert.equal(summary.settlementsByAsset["2"].settlements, 2);
  assert.equal(summary.settlementsByAsset["2"].gross, "20370370370");
  assert.equal(summary.settlementsByAsset["2"].fees, "40740740");
  assert.equal(summary.vaultsByAsset["2"].available, "504629629630");
});

test("el cliente rechaza importes inseguros y pagos no conciliados", () => {
  const fixture = {
    vaults: { 1: { asset: 2, reserve: 1_000, locked: 0 } },
    sessions: {},
    events: [{ type: "intent_settled", target_asset: 2, gross_out: 20, net_out: 18, fee: 2 }],
  };
  assert.equal(summarizeReport(fixture).settlementsByAsset["2"].net, "18");
  fixture.events[0].fee = 3;
  assert.throws(() => summarizeReport(fixture), /does not reconcile/);
  fixture.events[0].fee = 2;
  fixture.vaults[1].reserve = Number.MAX_SAFE_INTEGER + 1;
  assert.throws(() => summarizeReport(fixture), /safe integer/);
});
