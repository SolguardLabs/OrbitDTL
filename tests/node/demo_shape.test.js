import test from "node:test";
import assert from "node:assert/strict";
import { loadDemoReport } from "../helpers/orbit-cli.js";

test("el reporte json contiene las colecciones principales", () => {
  const report = loadDemoReport();

  assert.equal(Object.keys(report.accounts).length, 3);
  assert.equal(Object.keys(report.vaults).length, 2);
  assert.equal(Object.keys(report.sessions).length, 1);
  assert.equal(report.events.length, 13);
});

test("las cuentas del demo reflejan beneficiario y operador", () => {
  const report = loadDemoReport();

  assert.equal(report.accounts["1"].label, "orbit-operator");
  assert.equal(report.accounts["2"].label, "market-maker-a");
  assert.equal(report.accounts["3"].label, "settlement-recipient-b");
  assert.equal(report.accounts["3"].balances["2"], 20329629630);
  assert.equal(report.accounts["1"].balances["2"], 40740740);
});
