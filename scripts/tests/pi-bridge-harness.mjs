// Drives the pi extension bridge (framework/bootstrap/pi/ductus-bridge.ts)
// through a mock pi ExtensionAPI against a real ductus runtime, one fresh
// project directory and module instance per case.
//
// What it covers: the bridge's transport and lifecycle — tool registration
// from the live tools/list, a round-tripped call, a multi-byte character split
// across two chunks, the missing-binary load path, respawn after the
// runtime exits, recovery after a call fails, and retiring a runtime that
// stops answering — and the `{project}-status` and `{project}-help` commands
// it registers under the namespace `render-help` gives at load, each showing
// its answer, or a failure, as a displayed message that triggers no turn.
// What it does not: pi's own extension loader (jiti), how pi renders a
// result or a message, and whether a message reaches the agent's context —
// a real `pi` run is still the only test of those (AGENTS.md, the Pi bridge
// entry).
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
// case rather than leaving it waiting out two minutes. `hostProject` writes
// the `[host] project` the commands are named by. `helpCopy` installs
// help.md as /ductus installs it for Pi, `{project}` substituted, under the
// `.pi` config dir the session file records.
function project({ binary = runtime, timeoutMs, hostProject, helpCopy = false } = {}) {
  const dir = mkdtempSync(join(tmpdir(), "pi-bridge-"));
  dirs.push(dir);
  mkdirSync(join(dir, ".ductus", "bin"), { recursive: true });
  if (hostProject) writeFileSync(join(dir, ".ductus", "config.toml"), `[host]\nproject = "${hostProject}"\n`);
  if (helpCopy) {
    writeFileSync(join(dir, ".ductus", "session.toml"), `cli-config-dir = ".pi"\n`);
    mkdirSync(join(dir, ".pi", "prompts"), { recursive: true });
    const help = readFileSync(join(repo, "framework/commands/help.md"), "utf8").replaceAll("{project}", hostProject);
    writeFileSync(join(dir, ".pi", "prompts", `${hostProject}-help.md`), help);
  }
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

// A stand-in runtime that answers the handshake and lists the tools `names`,
// `probe` alone by default, and answers tools/call with `onCall` — the body
// of a function given `msg`, `reply(result)`, `raw(bytes)` and the stub's own
// `dir`. CommonJS, because the pointer it is reached through has no extension.
function stubRuntime(onCall, names = ["probe"]) {
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
    else if (msg.method === "tools/list") reply({ tools: ${JSON.stringify(names)}.map((name) => ({ name, inputSchema: { type: "object" } })) });
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
  const commands = new Map();
  const messages = [];
  const handlers = {};
  const notices = [];
  const api = {
    on: (event, handler) => { handlers[event] = handler; },
    registerTool: (tool) => tools.set(tool.name, tool),
    registerCommand: (name, options) => commands.set(name, options),
    sendMessage: (message, options) => messages.push({ message, options }),
  };
  const ctx = { ui: { notify: (message, level) => notices.push(`${level}: ${message}`) } };
  const url = pathToFileURL(join(dir, ".pi", "extensions", "ductus.ts")).href;
  const mod = await import(`${url}?case=${++loaded}`);
  mod.default(api);
  await sleep(100); // pi fires session_start right after loading extensions
  handlers.session_start?.({}, ctx);
  return { tools, notices, commands, messages };
}

// The one message a registered command sent: displayed, and triggering no turn.
function shownWithoutATurn(messages) {
  if (messages.length !== 1) throw new Error(`sent ${messages.length} messages`);
  const [{ message, options }] = messages;
  if (message.display !== true || options?.triggerTurn !== false)
    throw new Error(`not displayed without a turn: display ${message.display}, options ${JSON.stringify(options)}`);
  return message.content;
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

await check("registers {project}-status, whose handler shows the view and triggers no turn", async () => {
  const { tools, commands, messages } = await load(project({ hostProject: "harness", helpCopy: true }));
  await until(5000, () => commands.size > 1);
  if (commands.size !== 2 || !commands.has("harness-status") || !commands.has("harness-help"))
    throw new Error(`registered ${[...commands.keys()].join(", ")}`);
  if (!tools.has("ductus__render-status")) throw new Error("render-status was not registered as a tool");
  await within(5000, commands.get("harness-status").handler("", {}));
  const view = shownWithoutATurn(messages);
  if (!view.includes("| Feature | Status |")) throw new Error(`no view: ${view.slice(0, 200)}`);
});

await check("{project}-status spells each command hint as Pi names the command", async () => {
  const { commands, messages } = await load(project({ hostProject: "harness", helpCopy: true }));
  await until(5000, () => commands.size > 0);
  await within(5000, commands.get("harness-status").handler("", {}));
  const view = shownWithoutATurn(messages);
  const colon = view.match(/\/harness:[a-z]+/g) ?? [];
  if (colon.length) throw new Error(`hints Pi cannot run: ${[...new Set(colon)].join(", ")}`);
  if (!view.includes("/harness-target")) throw new Error(`no /harness-target hint: ${view.slice(-300)}`);
});

await check("a status command that cannot reach the runtime shows why the same way", async () => {
  const dir = project({ hostProject: "harness", helpCopy: true });
  const { commands, messages } = await load(dir);
  await until(5000, () => commands.size > 0);
  rmSync(join(dir, ".ductus/bin/ductus"));
  killRuntime(dir);
  await sleep(200);
  await within(5000, commands.get("harness-status").handler("", {}));
  const shown = shownWithoutATurn(messages);
  if (!shown.includes("could not show the view") || !shown.includes("run /ductus"))
    throw new Error(`the failure does not say why: ${shown}`);
});

await check("registers {project}-help, whose handler shows the guide as Pi spells it and triggers no turn", async () => {
  const { tools, commands, messages } = await load(project({ hostProject: "harness", helpCopy: true }));
  await until(5000, () => commands.size > 1);
  if (!tools.has("ductus__render-help")) throw new Error("render-help was not registered as a tool");
  await within(5000, commands.get("harness-help").handler("", {}));
  const guide = shownWithoutATurn(messages);
  if (!guide.startsWith("## harness — Spec-Driven Development Pipeline")) throw new Error(`no guide: ${guide.slice(0, 200)}`);
  if (!guide.includes("| `/harness-specify` |")) throw new Error(`no /harness-specify row: ${guide.slice(0, 600)}`);
  const colon = guide.match(/\/harness:[a-z]+/g) ?? [];
  if (colon.length) throw new Error(`commands Pi cannot run: ${[...new Set(colon)].join(", ")}`);
  if (guide.includes("<!--")) throw new Error("the guide carries the generator's markers");
});

await check("a help command whose help copy has gone shows why the same way", async () => {
  const dir = project({ hostProject: "harness", helpCopy: true });
  const { commands, messages } = await load(dir);
  await until(5000, () => commands.size > 1);
  rmSync(join(dir, ".pi", "prompts", "harness-help.md"));
  await within(5000, commands.get("harness-help").handler("", {}));
  const shown = shownWithoutATurn(messages);
  if (!shown.includes("could not show the guide") || !shown.includes("no installed copy of the help command"))
    throw new Error(`the failure does not say why: ${shown}`);
});

await check("a view that cannot be rendered at load still registers both commands, and status shows why", async () => {
  const dir = project({ hostProject: "harness", helpCopy: true });
  mkdirSync(join(dir, "specs", "001-broken"), { recursive: true });
  writeFileSync(join(dir, "specs", "001-broken", "spec.md"), "---\nstatus: [\n---\n");
  const { commands, messages, notices } = await load(dir);
  await until(5000, () => commands.size > 1);
  if (!commands.has("harness-status") || !commands.has("harness-help"))
    throw new Error(`registered ${[...commands.keys()].join(", ")}`);
  await sleep(200);
  if (notices.length) throw new Error(`a notice for a load that registered: ${notices.join("; ")}`);
  await within(5000, commands.get("harness-status").handler("", {}));
  const shown = shownWithoutATurn(messages);
  if (!shown.includes("could not show the view") || !shown.includes("001-broken"))
    throw new Error(`the failure does not name the spec: ${shown}`);
});

await check("with no help copy at load, no command registers, and the notice names render-help's error", async () => {
  const { tools, commands, notices } = await load(project({ hostProject: "harness" }));
  await until(5000, () => notices.length > 0);
  if (commands.size !== 0) throw new Error(`registered ${[...commands.keys()].join(", ")}`);
  if (!tools.has("ductus__render-help")) throw new Error("the tools were not registered");
  const [notice] = notices;
  if (!notice.includes("not registered, since render-help failed at load") || !notice.includes("no installed copy of the help command"))
    throw new Error(`the notice does not name the error: ${notice}`);
  if (notice.includes("missing or unusable")) throw new Error(`the notice blames the pointer: ${notice}`);
});

await check("a runtime with render-status but no render-help registers only the status command", async () => {
  const ANSWERS_AS_A_VIEW = `reply({ content: [{ type: "text", text: JSON.stringify({ "rendered-markdown": "x", project: "stub" }) }], isError: false });`;
  const { tools, commands } = await load(project({ binary: stubRuntime(ANSWERS_AS_A_VIEW, ["render-status"]) }));
  await until(5000, () => commands.size > 0);
  await sleep(200);
  if (tools.size !== 1 || commands.size !== 1 || !commands.has("stub-status"))
    throw new Error(`registered ${[...commands.keys()].join(", ")}`);
});

await check("a runtime without render-status registers no status command", async () => {
  // The stub would answer render-status, so only the tool list keeps it out.
  const ANSWERS_AS_A_VIEW = `reply({ content: [{ type: "text", text: JSON.stringify({ "rendered-markdown": "x", project: "stub" }) }], isError: false });`;
  const { tools, commands } = await load(project({ binary: stubRuntime(ANSWERS_AS_A_VIEW) }));
  await until(5000, () => tools.size > 0);
  await sleep(200);
  if (commands.size !== 0) throw new Error(`registered ${[...commands.keys()].join(", ")}`);
});

console.log(results.join("\n"));
process.chdir(repo);
for (const dir of dirs) rmSync(dir, { recursive: true, force: true });
process.exit(results.every((r) => r.startsWith("PASS")) ? 0 : 1);
