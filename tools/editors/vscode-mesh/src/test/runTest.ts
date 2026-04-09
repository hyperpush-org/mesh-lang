import * as fs from "node:fs";
import * as path from "node:path";

import { runTests } from "@vscode/test-electron";

function log(message: string, logFile: string) {
  const line = `[runner] ${message}`;
  fs.appendFileSync(logFile, `${line}\n`);
  console.error(line);
}

function fail(phase: string, message: string): never {
  throw new Error(`[${phase}] ${message}`);
}

function resetDir(dirPath: string) {
  fs.rmSync(dirPath, { recursive: true, force: true });
  fs.mkdirSync(dirPath, { recursive: true });
}

function suiteStarted(logFile: string): boolean {
  if (!fs.existsSync(logFile)) {
    return false;
  }

  return fs
    .readFileSync(logFile, "utf8")
    .includes("[suite] Starting VS Code smoke suite");
}

function ensureExecutable(filePath: string, phase: string) {
  if (!fs.existsSync(filePath)) {
    fail(
      phase,
      `Missing repo-local meshc at ${filePath}. Build meshc or set the smoke override before rerunning.`
    );
  }

  if (process.platform !== "win32") {
    try {
      fs.accessSync(filePath, fs.constants.X_OK);
    } catch {
      fail(
        phase,
        `Repo-local meshc exists but is not executable at ${filePath}. Rebuild or chmod it before rerunning.`
      );
    }
  }
}

async function main() {
  const extensionDevelopmentPath = path.resolve(__dirname, "../..");
  const repoRoot = path.resolve(extensionDevelopmentPath, "../../..");
  const artifactDir = path.join(repoRoot, ".tmp", "m036-s03", "vscode-smoke");
  const workspaceDir = path.join(artifactDir, "workspace");
  const userDataDir = path.join(artifactDir, "user-data");
  const extensionsDir = path.join(artifactDir, "extensions");
  const logFile = path.join(artifactDir, "smoke.log");
  const workspaceFile = path.join(workspaceDir, "mesh-smoke.code-workspace");
  const meshcPath = path.join(repoRoot, "target", "debug", "meshc");
  const extensionTestsPath = path.resolve(__dirname, "./suite/index.js");

  fs.mkdirSync(artifactDir, { recursive: true });
  fs.mkdirSync(workspaceDir, { recursive: true });
  resetDir(userDataDir);
  resetDir(extensionsDir);
  fs.writeFileSync(logFile, "", "utf8");

  log(`artifactDir=${artifactDir}`, logFile);
  log(`extensionDevelopmentPath=${extensionDevelopmentPath}`, logFile);
  log(`extensionTestsPath=${extensionTestsPath}`, logFile);

  ensureExecutable(meshcPath, "setup/meshc");
  log(`meshcPath=${meshcPath}`, logFile);

  const workspaceConfig = {
    folders: [{ path: repoRoot }],
    settings: {
      "mesh.lsp.path": meshcPath,
    },
  };

  fs.writeFileSync(
    workspaceFile,
    `${JSON.stringify(workspaceConfig, null, 2)}\n`,
    "utf8"
  );

  const writtenWorkspace = JSON.parse(fs.readFileSync(workspaceFile, "utf8"));
  if (writtenWorkspace.settings?.["mesh.lsp.path"] !== meshcPath) {
    fail(
      "setup/workspace",
      `Workspace settings drifted after write. Expected mesh.lsp.path=${meshcPath}, got ${writtenWorkspace.settings?.["mesh.lsp.path"] ?? "<missing>"}.`
    );
  }

  fs.writeFileSync(
    path.join(artifactDir, "context.json"),
    `${JSON.stringify(
      {
        artifactDir,
        workspaceFile,
        meshcPath,
        extensionDevelopmentPath,
        extensionTestsPath,
      },
      null,
      2
    )}\n`,
    "utf8"
  );
  log(`workspaceFile=${workspaceFile}`, logFile);

  const launchOnce = async (attempt: number) => {
    log(`Launching Extension Development Host (attempt ${attempt})`, logFile);
    const exitCode = await runTests({
      extensionDevelopmentPath,
      extensionTestsPath,
      reuseMachineInstall: false,
      extensionTestsEnv: {
        MESH_VSCODE_SMOKE_ARTIFACT_DIR: artifactDir,
        MESH_VSCODE_SMOKE_WORKSPACE_FILE: workspaceFile,
        MESH_VSCODE_SMOKE_MESHC_PATH: meshcPath,
        MESH_VSCODE_SMOKE_REPO_ROOT: repoRoot,
        MESH_VSCODE_SMOKE_LOG_FILE: logFile,
      },
      launchArgs: [
        workspaceFile,
        "--disable-workspace-trust",
        "--skip-welcome",
        "--skip-release-notes",
        "--user-data-dir",
        userDataDir,
        "--extensions-dir",
        extensionsDir,
      ],
    });

    if (exitCode !== 0) {
      fail(
        "extension-host/exit",
        `Extension Development Host exited with code ${exitCode}. Inspect ${logFile} and ${artifactDir}.`
      );
    }
  };

  try {
    let lastError: unknown;
    for (let attempt = 1; attempt <= 2; attempt += 1) {
      if (attempt > 1) {
        log(
          "Retrying Extension Development Host after a pre-suite exit with clean VS Code state",
          logFile
        );
        resetDir(userDataDir);
        resetDir(extensionsDir);
      }

      try {
        await launchOnce(attempt);
        log("Extension Development Host smoke passed", logFile);
        return;
      } catch (error) {
        lastError = error;
        const detail = error instanceof Error ? error.stack ?? error.message : String(error);
        const sawSuiteStart = suiteStarted(logFile);

        if (attempt < 2 && !sawSuiteStart) {
          log(
            `Attempt ${attempt} failed before the suite started; retrying once. ${detail}`,
            logFile
          );
          continue;
        }

        throw error;
      }
    }

    throw lastError;
  } catch (error) {
    const detail = error instanceof Error ? error.stack ?? error.message : String(error);
    log(`Smoke failed: ${detail}`, logFile);
    throw error;
  }
}

main().catch((error) => {
  const detail = error instanceof Error ? error.stack ?? error.message : String(error);
  console.error(detail);
  process.exit(1);
});
