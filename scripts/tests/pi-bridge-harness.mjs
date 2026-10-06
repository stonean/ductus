// Drives the pi extension bridge (framework/bootstrap/pi/ductus-bridge.ts)
// through a mock pi ExtensionAPI against a real ductus runtime, one fresh
// project directory and module instance per case.
//
// What it covers: the bridge's transport and lifecycle — tool registration
// from the live tools/list, a round-tripped call, the missing-binary load
// path, respawn after the runtime exits, and recovery after a call fails.
// What it does not: pi's own extension loader (jiti) and how pi renders a
// result — a real `pi` run is still the only test of those (AGENTS.md, the
// Pi bridge entry).
//
// Not wired into CI: it loads the bridge's TypeScript directly, which needs
// Node's built-in type stripping (Node 22.18+ / 23.6+), and CI pins Node 20.
// Run it by hand after changing the bridge:
//
//   (cd runtime && cargo build --release --locked)
//   node scripts/tests/pi-bridge-harness.mjs [bridge.ts] [runtime-binary]
import { mkdtempSync, mkdirSync, symlinkSync, copyFileSync, rmSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

const repo = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");
const bridgeSrc = resolve(process.argv[2] ?? join(repo, "framework/bootstrap/pi/ductus-bridge.ts"));
const runtime = resolve(process.argv[3] ?? join(repo, "runtime/target/release/ductus"));
const manifestTools = readFileSync(join(repo, "framework/runtime-tools.txt"), "utf8")
  .split("\n")
  .filter((line) => /^[a-z]/.test(line)).length;

const uncaught = [];
process.on("uncaughtException", (e) => uncaught.push(e.code || e.message));
process.on("unhandledRejection", (e) => uncaught.push(`unhandledRejection: ${e?.message ?? e}`));

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const within = (ms, p) =>
  Promise.race([p, sleep(ms).then(() => { throw new Error(`no answer within ${ms}ms`); })]);
const until = (ms, cond) => within(ms, (async () => { while (!cond()) await sleep(20); })());

const dirs = [];
function project({ withBinary }) {
  const dir = mkdtempSync(join(tmpdir(), "pi-bridge-"));
  dirs.push(dir);
  mkdirSync(join(dir, ".ductus", "bin"), { recursive: true });
  if (withBinary) symlinkSync(runtime, join(dir, ".ductus", "bin", "ductus"));
  mkdirSync(join(dir, ".pi", "extensions"), { recursive: true });
  copyFileSync(bridgeSrc, join(dir, ".pi", "extensions", "ductus.ts"));
  return dir;
}

function killRuntime(dir) {
  execFileSync("pkill", ["-f", `${join(dir, ".ductus/bin/ductus")} mcp`]);
}

let loaded = 0;
async function load(dir) {
  process.chdir(dir);
  const tools = new Map();
  const handlers = {};
  const notices = [];
  const api = {
    on: (event, handler) => { handlers[event] = handler; },
    registerTool: (tool) => tools.set(tool.name, tool),
  };
  const ctx = { ui: { notify: (message, level) => notices.push(`${level}: ${message}`) } };
  const url = pathToFileURL(join(dir, ".pi", "extensions", "ductus.ts")).href;
  const mod = await import(`${url}?case=${++loaded}`);
  mod.default(api);
  await sleep(100); // pi fires session_start right after loading extensions
  handlers.session_start?.({}, ctx);
  return { tools, notices };
}

const ARGS = { feature: "no-such-feature" };
const results = [];
async function check(name, fn) {
  const before = uncaught.length;
  try {
    await fn();
    if (uncaught.length > before) throw new Error(`uncaught in the host: ${uncaught.slice(before).join("; ")}`);
    results.push(`PASS ${name}`);
  } catch (e) {
    results.push(`FAIL ${name}: ${e.message}`);
  }
}

await check("registers every runtime tool and round-trips a call", async () => {
  const { tools } = await load(project({ withBinary: true }));
  await until(5000, () => tools.size > 0);
  if (tools.size !== manifestTools) throw new Error(`registered ${tools.size}, runtime-tools.txt lists ${manifestTools}`);
  const res = await within(5000, tools.get("ductus__check-artifact-size").execute("t1", ARGS));
  if (!res.content?.[0]?.text) throw new Error(`empty result ${JSON.stringify(res)}`);
});

await check("missing binary at load: no host crash, no tools, a notice naming the pointer and /ductus", async () => {
  const { tools, notices } = await load(project({ withBinary: false }));
  await until(3000, () => notices.length > 0);
  if (tools.size !== 0) throw new Error(`registered ${tools.size} tools with no runtime`);
  if (!notices[0].includes(".ductus/bin/ductus") || !notices[0].includes("run /ductus"))
    throw new Error(`notice does not name the pointer and /ductus: ${notices[0]}`);
});

await check("runtime exits mid-session: the next call respawns it", async () => {
  const dir = project({ withBinary: true });
  const { tools } = await load(dir);
  await until(5000, () => tools.size > 0);
  const tool = tools.get("ductus__check-artifact-size");
  await within(5000, tool.execute("t1", ARGS));
  killRuntime(dir);
  await sleep(200);
  const res = await within(5000, tool.execute("t2", ARGS));
  if (res.content?.[0]?.text?.includes("exited")) throw new Error(`no respawn: ${res.content[0].text}`);
});

await check("binary removed mid-session: an envelope naming /ductus, then recovery once it is back", async () => {
  const dir = project({ withBinary: true });
  const { tools } = await load(dir);
  await until(5000, () => tools.size > 0);
  const tool = tools.get("ductus__check-artifact-size");
  await within(5000, tool.execute("t1", ARGS));
  const pointer = join(dir, ".ductus/bin/ductus");
  rmSync(pointer);
  killRuntime(dir);
  await sleep(200);
  const gone = await within(5000, tool.execute("t2", ARGS));
  if (!gone.isError || !gone.content[0].text.includes("run /ductus"))
    throw new Error(`no envelope: ${JSON.stringify(gone)}`);
  symlinkSync(runtime, pointer);
  const back = await within(5000, tool.execute("t3", ARGS));
  if (back.content[0].text.includes("run /ductus")) throw new Error(`calls did not recover: ${back.content[0].text}`);
});

console.log(results.join("\n"));
process.chdir(repo);
for (const dir of dirs) rmSync(dir, { recursive: true, force: true });
process.exit(results.every((r) => r.startsWith("PASS")) ? 0 : 1);
