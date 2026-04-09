import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

import { getVsixFilename, getVsixPath } from "./vsix-path.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const extensionRoot = path.resolve(scriptDir, "..");
const packageJson = JSON.parse(
  fs.readFileSync(path.join(extensionRoot, "package.json"), "utf8")
);

test("getVsixPath returns the deterministic dist-relative path", () => {
  assert.equal(getVsixFilename(), `${packageJson.name}-${packageJson.version}.vsix`);
  assert.equal(
    getVsixPath(),
    `dist/${packageJson.name}-${packageJson.version}.vsix`
  );
});

test("CLI path command prints the same deterministic path", () => {
  const result = spawnSync(process.execPath, ["./scripts/vsix-path.mjs"], {
    cwd: extensionRoot,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, getVsixPath());
});

test("CLI accepts --absolute without an explicit command", () => {
  const result = spawnSync(process.execPath, ["./scripts/vsix-path.mjs", "--absolute"], {
    cwd: extensionRoot,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, path.join(extensionRoot, getVsixPath()));
});
