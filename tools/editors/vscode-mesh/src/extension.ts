import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { ExtensionContext, OutputChannel, window, workspace } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

export interface MeshExtensionApi {
  resolvedMeshcPath: string;
  resolutionSource: string;
}

interface MeshcResolution {
  meshcPath: string;
  source: string;
}

let client: LanguageClient | undefined;
let outputChannel: OutputChannel | undefined;

function ensureOutputChannel(context: ExtensionContext): OutputChannel {
  if (!outputChannel) {
    outputChannel = window.createOutputChannel("Mesh Language");
    context.subscriptions.push(outputChannel);
  }
  return outputChannel;
}

function log(message: string) {
  outputChannel?.appendLine(`[mesh-lang] ${message}`);
}

function canExecuteCandidate(candidate: string): boolean {
  if (!fs.existsSync(candidate)) {
    return false;
  }

  if (process.platform === "win32") {
    return true;
  }

  try {
    fs.accessSync(candidate, fs.constants.X_OK);
    return true;
  } catch {
    return false;
  }
}

function findMeshc(): MeshcResolution {
  const config = workspace.getConfiguration("mesh.lsp");
  const configured = config.get<string>("path");
  if (configured && configured !== "meshc") {
    return {
      meshcPath: configured,
      source: "configuration",
    };
  }

  const workspaceFolders = workspace.workspaceFolders;
  if (workspaceFolders) {
    for (const folder of workspaceFolders) {
      const candidates = [
        {
          candidate: path.join(folder.uri.fsPath, "target", "debug", "meshc"),
          source: `workspace:${folder.name}:target/debug`,
        },
        {
          candidate: path.join(folder.uri.fsPath, "target", "release", "meshc"),
          source: `workspace:${folder.name}:target/release`,
        },
      ];
      for (const { candidate, source } of candidates) {
        if (canExecuteCandidate(candidate)) {
          return {
            meshcPath: candidate,
            source,
          };
        }
      }
    }
  }

  const home = os.homedir();
  const wellKnown = [
    {
      candidate: path.join(home, ".mesh", "bin", "meshc"),
      source: "well-known:~/.mesh/bin",
    },
    {
      candidate: "/usr/local/bin/meshc",
      source: "well-known:/usr/local/bin",
    },
    {
      candidate: "/opt/homebrew/bin/meshc",
      source: "well-known:/opt/homebrew/bin",
    },
  ];
  for (const { candidate, source } of wellKnown) {
    if (canExecuteCandidate(candidate)) {
      return {
        meshcPath: candidate,
        source,
      };
    }
  }

  return {
    meshcPath: "meshc",
    source: "path",
  };
}

async function startClient(meshcPath: string) {
  const serverOptions: ServerOptions = {
    command: meshcPath,
    args: ["lsp"],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "mesh" }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher("**/*.mpl"),
    },
  };

  client = new LanguageClient(
    "mesh-lsp",
    "Mesh Language Server",
    serverOptions,
    clientOptions
  );

  await client.start();
}

function reportStartFailure(meshcPath: string, error: unknown) {
  const detail = error instanceof Error ? error.message : String(error);
  log(`Mesh LSP failed to start from ${meshcPath}: ${detail}`);

  void window
    .showErrorMessage(
      `Mesh LSP failed to start from '${meshcPath}'. Install Mesh or configure 'mesh.lsp.path' to a working meshc binary.`,
      "Configure Path",
      "Dismiss"
    )
    .then(async (action) => {
      if (action !== "Configure Path") {
        return;
      }

      const uris = await window.showOpenDialog({
        canSelectFiles: true,
        canSelectFolders: false,
        canSelectMany: false,
        title: "Select the meshc binary",
        openLabel: "Select meshc",
      });

      if (!uris || uris.length === 0) {
        return;
      }

      const selected = uris[0].fsPath;
      await workspace.getConfiguration("mesh.lsp").update("path", selected, true);
      window.showInformationMessage(
        `Mesh LSP path set to '${selected}'. Reload window to activate.`
      );
    });

  throw new Error(`Mesh LSP failed to start from '${meshcPath}': ${detail}`);
}

export async function activate(
  context: ExtensionContext
): Promise<MeshExtensionApi> {
  ensureOutputChannel(context);

  const resolution = findMeshc();
  log(
    `Resolved meshc from ${resolution.source}: ${resolution.meshcPath}`
  );

  try {
    await startClient(resolution.meshcPath);
    log(`Mesh LSP started successfully from ${resolution.meshcPath}`);
  } catch (error) {
    reportStartFailure(resolution.meshcPath, error);
  }

  return {
    resolvedMeshcPath: resolution.meshcPath,
    resolutionSource: resolution.source,
  };
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
