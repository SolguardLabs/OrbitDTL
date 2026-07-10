import { readdirSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const projectRoot = resolve(fileURLToPath(new URL("..", import.meta.url)));
const checkedDirs = ["tests/helpers", "tests/node"];

for (const directory of checkedDirs) {
  const absoluteDirectory = resolve(projectRoot, directory);

  for (const entry of readdirSync(absoluteDirectory, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".js") || entry.name.endsWith(".test.js")) {
      continue;
    }

    const file = resolve(absoluteDirectory, entry.name);
    const result = spawnSync(process.execPath, ["--check", file], {
      cwd: projectRoot,
      encoding: "utf8",
      stdio: "pipe",
    });

    if (result.status !== 0) {
      process.stderr.write(result.stderr);
      process.exit(result.status ?? 1);
    }
  }
}

console.log("JavaScript syntax check completed");
