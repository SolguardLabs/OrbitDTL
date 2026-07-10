import test from "node:test";
import assert from "node:assert/strict";
import { loadDemoReport } from "../helpers/orbit-cli.js";

test("la sesion queda cerrada con dos intents incluidos", () => {
  const report = loadDemoReport();
  const session = report.sessions["1"];

  assert.equal(session.closed, true);
  assert.deepEqual(session.included_intents, [1, 2]);
  assert.equal(session.counterflow["1"], 10500000000);
  assert.equal(session.accounted_counterflow["1"], 19759259259);
});

test("los vaults mantienen reservas y bloqueos esperados tras el demo", () => {
  const report = loadDemoReport();
  const sourceVault = report.vaults["1"];
  const targetVault = report.vaults["2"];

  assert.equal(sourceVault.reserve, 578000000000);
  assert.equal(sourceVault.locked, 0);
  assert.equal(sourceVault.paid, 22000000000);
  assert.equal(targetVault.reserve, 504629629630);
  assert.equal(targetVault.locked, 0);
  assert.equal(targetVault.paid, 20370370370);
});
