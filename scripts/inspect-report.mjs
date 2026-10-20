import { readFileSync } from "node:fs";
import { summarizeReport } from "../client/report-client.mjs";

const path = process.argv[2];
if (!path) {
  console.error("Uso: bun run inspect <reporte.json>");
  process.exitCode = 2;
} else {
  try {
    const report = JSON.parse(readFileSync(path, "utf8"));
    console.log(JSON.stringify(summarizeReport(report), null, 2));
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  }
}
