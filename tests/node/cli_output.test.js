import test from "node:test";
import assert from "node:assert/strict";
import { runOrbit } from "../helpers/orbit-cli.js";

test("el comando demo devuelve un resumen operativo estable", () => {
  const stdout = runOrbit(["demo"]);

  assert.match(stdout, /Orbit DTL demo executed/);
  assert.match(stdout, /accounts: 3/);
  assert.match(stdout, /tracked_balances: 3/);
  assert.match(stdout, /vaults: 2/);
  assert.match(stdout, /sessions: 1/);
  assert.match(stdout, /events: 13/);
});

test("el comando help publica el subcomando demo", () => {
  const stdout = runOrbit(["--help"]);

  assert.match(stdout, /Orbit DTL settlement console/);
  assert.match(stdout, /demo/);
});
