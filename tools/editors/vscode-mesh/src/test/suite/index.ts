import * as fs from "node:fs";
import * as path from "node:path";

function log(message: string) {
  const artifactDir = process.env.MESH_VSCODE_SMOKE_ARTIFACT_DIR;
  const line = `[suite] ${message}`;
  if (artifactDir) {
    fs.mkdirSync(artifactDir, { recursive: true });
    fs.appendFileSync(path.join(artifactDir, "smoke.log"), `${line}\n`);
  }
  console.error(line);
}

export async function run(): Promise<void> {
  try {
    log("Starting VS Code smoke suite");
    const { runSmokeSuite } = await import("./extension.test");
    await runSmokeSuite();
    log("VS Code smoke suite passed");
  } catch (error) {
    const detail = error instanceof Error ? error.stack ?? error.message : String(error);
    log(`VS Code smoke suite failed: ${detail}`);
    throw error;
  }
}
