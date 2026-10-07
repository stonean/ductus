// Drives the pi extension bridge (framework/bootstrap/pi/ductus-bridge.ts)
// through a mock pi ExtensionAPI against a real ductus runtime, one fresh
// project directory and module instance per case.
//
// What it covers: the bridge's transport and lifecycle — tool registration
// from the live tools/list, a round-tripped call, a multi-byte character split
// across two chunks, the missing-binary load path, respawn after the
// runtime exits, recovery after a call fails, and retiring a runtime that
// stops answering.
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
import { mkdtempSync, mkdirSync, symlinkSync, rmSync, readFileSync, writeFileSync, chmodSync } from "node:fs";
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
const TIMEOUT_DECL = "const REQUEST_TIMEOUT_MS = 120_000;";
// `binary` is the pointer's target — the runtime build, a stub, or none.
// `timeoutMs` rewrites the bridge copy's request cap, so a timeout case runs
// in milliseconds; the rewrite is asserted, so a renamed constant fails the
// case rather than leaving it waiting out two minutes.
function project({ binary = runtime, timeoutMs } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "pi-bridge-"));
  dirs.push(dir);
  mkdirSync(join(dir, ".ductus", "bin"), { recursive: true });
  if (binary) symlinkSync(binary, join(dir, ".ductus", "bin", "ductus"));
  mkdirSync(join(dir, ".pi", "extensions"), { recursive: true });
  let bridge = readFileSync(bridgeSrc, "utf8");
  if (timeoutMs !== undefined) {
    if (!bridge.includes(TIMEOUT_DECL)) throw new Error(`bridge no longer declares ${TIMEOUT_DECL}`);
    bridge = bridge.replace(TIMEOUT_DECL, `const REQUEST_TIMEOUT_MS = ${timeoutMs};`);
  }
  writeFileSync(join(dir, ".pi", "extensions", "ductus.ts"), bridge);
  return dir;
}

// A stand-in runtime that answers the handshake and lists one tool, `probe`,
// and answers tools/call with `onCall` — the body of a function given
// `msg`, `reply(result)`, `raw(bytes)` and the stub's own `dir`. CommonJS,
// because the pointer it is reached through has no extension.
function stubRuntime(onCall) {
  const dir = mkdtempSync(join(tmpdir(), "pi-bridge-stub-"));
  dirs.push(dir);
  const stub = join(dir, "stub");
  writeFileSync(stub, `#!/usr/bin/env node
const fs = require("node:fs");
const dir = ${JSON.stringify(dir)};
const state = { wedged: false };
const onCall = (msg, reply, raw) => { ${onCall} };
let buf = "";
process.stdin.setEncoding("utf8");
process.stdin.on("data", (data) => {
  buf += data;
  let nl;
  while ((nl = buf.indexOf("\\n")) >= 0) {
    const line = buf.slice(0, nl).trim();
    buf = buf.slice(nl + 1);
    if (!line || state.wedged) continue;
    const msg = JSON.parse(line);
    const reply = (result) =>
      process.stdout.write(JSON.stringify({ jsonrpc: "2.0", id: msg.id, result }) + "\\n");
    const raw = (bytes) => process.stdout.write(bytes);
    if (msg.method === "initialize") reply({ protocolVersion: "2025-03-26", capabilities: {}, serverInfo: { name: "stub", version: "0" } });
    else if (msg.method === "tools/list") reply({ tools: [{ name: "probe", inputSchema: { type: "object" } }] });
    else if (msg.method === "tools/call") onCall(msg, reply, raw);
  }
});
`);
  chmodSync(stub, 0o755);
  return stub;
}

// The first process to receive a tools/call goes silent for good, leaving a
// marker; every later process answers.
const WEDGE_ONCE = `
  const marker = dir + "/wedged-once";
  if (fs.existsSync(marker)) reply({ content: [{ type: "text", text: "answered" }], isError: false });
  else { fs.writeFileSync(marker, ""); state.wedged = true; }`;

// The response line is written in two writes cut inside an em-dash, 50 ms
// apart, so the bridge receives the character split across two chunks every
// time rather than whenever a pipe boundary happens to land in one.
const SPLIT_INSIDE_A_DASH = `
  const line = Buffer.from(JSON.stringify({ jsonrpc: "2.0", id: msg.id, result: { content: [{ type: "text", text: "a\\u2014b" }], isError: false } }) + "\\n");
  const cut = line.indexOf(Buffer.from("\\u2014")) + 1;
  raw(line.subarray(0, cut));
  setTimeout(() => raw(line.subarray(cut)), 50);`;

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
  const { tools } = await load(project());
  await until(5000, () => tools.size > 0);
  if (tools.size !== manifestTools) throw new Error(`registered ${tools.size}, runtime-tools.txt lists ${manifestTools}`);
  const res = await within(5000, tools.get("ductus__check-artifact-size").execute("t1", ARGS));
  if (!res.content?.[0]?.text) throw new Error(`empty result ${JSON.stringify(res)}`);
});

await check("a character split across two chunks arrives intact", async () => {
  const { tools } = await load(project({ binary: stubRuntime(SPLIT_INSIDE_A_DASH) }));
  await until(5000, () => tools.size > 0);
  const res = await within(3000, tools.get("ductus__probe").execute("t1", {}));
  if (res.isError || res.content[0].text !== "a—b")
    throw new Error(`the split character was corrupted: ${JSON.stringify(res)}`);
});

await check("missing binary at load: no host crash, no tools, a notice naming the pointer and /ductus", async () => {
  const { tools, notices } = await load(project({ binary: null }));
  await until(3000, () => notices.length > 0);
  if (tools.size !== 0) throw new Error(`registered ${tools.size} tools with no runtime`);
  if (!notices[0].includes(".ductus/bin/ductus") || !notices[0].includes("run /ductus"))
    throw new Error(`notice does not name the pointer and /ductus: ${notices[0]}`);
});

await check("runtime exits mid-session: the next call respawns it", async () => {
  const dir = project();
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
  const dir = project();
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

await check("a runtime that stops answering times out once, then is replaced", async () => {
  const { tools } = await load(project({ binary: stubRuntime(WEDGE_ONCE), timeoutMs: 300 }));
  await until(5000, () => tools.size > 0);
  const tool = tools.get("ductus__probe");
  const hung = await within(3000, tool.execute("t1", {}));
  if (!hung.isError || !hung.content[0].text.includes("timed out"))
    throw new Error(`no timeout envelope: ${JSON.stringify(hung)}`);
  // Kept as the connection, the wedged child would time this call out too.
  const next = await within(3000, tool.execute("t2", {}));
  if (next.isError || next.content[0].text !== "answered")
    throw new Error(`the wedged runtime was not replaced: ${JSON.stringify(next)}`);
});

console.log(results.join("\n"));
process.chdir(repo);
for (const dir of dirs) rmSync(dir, { recursive: true, force: true });
process.exit(results.every((r) => r.startsWith("PASS")) ? 0 : 1);
