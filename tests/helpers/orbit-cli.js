import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const currentDir = dirname(fileURLToPath(import.meta.url));
export const projectRoot = resolve(currentDir, "../..");

export function runOrbit(args = []) {
  return execFileSync("cargo", ["run", "--quiet", "--", ...args], {
    cwd: projectRoot,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}

export function loadDemoReport() {
  const stdout = runOrbit(["demo", "--json"]);
  const report = JSON.parse(stdout);

  assert.equal(typeof report, "object");
  assert.ok(report);
  return report;
}

export function eventTypes(report) {
  return report.events.map((event) => event.type);
}
