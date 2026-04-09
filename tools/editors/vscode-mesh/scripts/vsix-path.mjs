import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const extensionRoot = path.resolve(scriptDir, "..");
const packageJsonPath = path.join(extensionRoot, "package.json");
const distDir = path.join(extensionRoot, "dist");

function readPackageJson() {
  return JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
}

export function getVsixFilename() {
  const pkg = readPackageJson();
  return `${pkg.name}-${pkg.version}.vsix`;
}

export function getVsixPath(options = {}) {
  const relativePath = path.posix.join("dist", getVsixFilename());
  if (options.absolute) {
    return path.join(extensionRoot, relativePath);
  }
  return relativePath;
}

function logPhase(message) {
  console.error(`[vsix-path] ${message}`);
}

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: extensionRoot,
    stdio: "inherit",
  });

  if (result.error) {
    throw result.error;
  }

  if (typeof result.status === "number" && result.status !== 0) {
    process.exit(result.status);
  }
}

function cleanDistArtifacts() {
  fs.mkdirSync(distDir, { recursive: true });
  for (const entry of fs.readdirSync(distDir, { withFileTypes: true })) {
    if (entry.isFile() && entry.name.endsWith(".vsix")) {
      fs.rmSync(path.join(distDir, entry.name), { force: true });
      logPhase(`removed stale dist/${entry.name}`);
    }
  }
}

function packageVsix() {
  const vsixPath = getVsixPath();
  cleanDistArtifacts();
  logPhase(`packaging ${vsixPath}`);
  run(resolveLocalBin("vsce"), ["package", "--out", getVsixPath({ absolute: true })]);
}

function installLocal() {
  const vsixPath = getVsixPath();
  packageVsix();
  logPhase(`installing ${vsixPath}`);
  run("code", ["--install-extension", getVsixPath({ absolute: true })]);
}

function resolveLocalBin(name) {
  const suffix = process.platform === "win32" ? ".cmd" : "";
  return path.join(extensionRoot, "node_modules", ".bin", `${name}${suffix}`);
}

function main() {
  const args = process.argv.slice(2);
  const absolute = args.includes("--absolute");
  const command = args.find((arg) => !arg.startsWith("-")) ?? "path";

  if (command === "path") {
    process.stdout.write(getVsixPath({ absolute }));
    return;
  }

  if (command === "package") {
    packageVsix();
    return;
  }

  if (command === "install-local") {
    installLocal();
    return;
  }

  throw new Error(`Unknown command: ${command}`);
}

const isDirectRun =
  process.argv[1] !== undefined &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isDirectRun) {
  try {
    main();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error(`[vsix-path] ${message}`);
    process.exit(1);
  }
}
