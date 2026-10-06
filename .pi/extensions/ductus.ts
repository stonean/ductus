/**
 * ductus — pi extension bridge to the ductus runtime.
 *
 * Pi has no built-in MCP client (by design), so this extension wraps the
 * ductus runtime's own MCP server — `.ductus/bin/ductus mcp` — over stdio
 * and re-registers every tool it lists as a pi tool under the `ductus__`
 * prefix. The runtime's MCP server stays the single source of truth for tool
 * names and schemas: this file holds no second copy of the registry, and a
 * runtime release that adds or changes a primitive reaches pi with no edit
 * here.
 *
 * Written to `.pi/extensions/ductus.ts` by /ductus (Shared Files manifest,
 * strategy `update`). Zero npm dependencies — the MCP-over-stdio transport is
 * newline-delimited JSON-RPC 2.0, which this file speaks directly. Keep it
 * dependency-free: the README promises nothing enters the adopter's
 * dependency manifest, and the live-schema property depends on the bridge
 * staying a thin transport.
 *
 * Failures are loud, never silent. When the runtime cannot start at load, no
 * `ductus__*` tool is registered — a placeholder would read as a live runtime
 * to /ductus, which must instead acquire it — and session start shows a notice
 * naming the pointer and pointing at `/ductus`. When the runtime dies or goes
 * missing mid-session, the call returns an error envelope saying the same, and
 * the next call respawns it. No markdown-only fallback — a wired host that lost
 * its binary must stop, not degrade (§runtime-boundary).
 *
 * spec 064 (pi host support). See framework/bootstrap/ductus.md §MCP
 * registration.
 */

import { spawn, type ChildProcess } from "node:child_process";
import { resolve, dirname, join } from "node:path";
import { existsSync } from "node:fs";

/** The MCP protocol version the rmcp-based runtime serves (2025-03-26). */
const MCP_PROTOCOL_VERSION = "2025-03-26";

/** How long one request may wait on the runtime before it is failed. */
const REQUEST_TIMEOUT_MS = 120_000;

type Json = unknown;
interface JsonRpcResponse {
  jsonrpc: "2.0";
  id: number;
  result?: Json;
  error?: { code: number; message: string };
}

interface McpTool {
  name: string;
  description?: string;
  inputSchema: Record<string, Json>;
}

interface ToolResult {
  content: { type: "text"; text: string }[];
  isError: boolean;
}

/** The slice of pi's `ExtensionAPI` this bridge uses, declared here so the
 *  file stays dependency-free. */
interface ExtensionAPI {
  on(
    event: "session_start",
    handler: (
      event: unknown,
      ctx: { ui: { notify(message: string, level: "error"): void } },
    ) => unknown,
  ): void;
  registerTool(tool: {
    name: string;
    label: string;
    description: string;
    parameters: Record<string, Json>;
    execute(id: string, params: Record<string, Json>): Promise<ToolResult>;
  }): void;
}

function failure(text: string): ToolResult {
  return { content: [{ type: "text", text }], isError: true };
}

function reason(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}

/** The ductus runtime child process: spawned and initialized on first use,
 *  and again on the first use after it dies. */
class DuctusServer {
  pointerPath: string;
  private child: ChildProcess | null = null;
  private connection: Promise<ChildProcess> | null = null;
  private lineBuffer = "";
  private pending = new Map<number, (response: JsonRpcResponse) => void>();
  private nextId = 1;
  /** Calls run one at a time, in the order pi issues them. */
  private callChain: Promise<unknown> = Promise.resolve();

  constructor(pointerPath: string) {
    this.pointerPath = pointerPath;
  }

  /** Forget the child and fail every in-flight request, so no call waits on
   *  a dead process and the next call respawns. */
  private drop(message: string) {
    this.child = null;
    this.connection = null;
    const dying = this.pending;
    this.pending = new Map();
    for (const [id, settle] of dying) {
      settle({ jsonrpc: "2.0", id, error: { code: -32000, message } });
    }
  }

  private spawnChild(): ChildProcess {
    /* stderr is ignored rather than piped: a pipe nobody reads fills and
     * blocks the child. */
    const proc = spawn(this.pointerPath, ["mcp"], { stdio: ["pipe", "pipe", "ignore"] });
    this.child = proc;
    this.lineBuffer = "";
    proc.stdout?.on("data", (chunk: Buffer) => this.onData(chunk));
    /* A missing or non-executable pointer arrives as an `error` event, not a
     * throw from spawn, and an unlistened `error` is an uncaught exception in
     * pi's own process. Only the current child may reset the state. */
    proc.on("error", (err: Error & { code?: string }) => {
      if (this.child === proc) this.drop(`could not be started: ${err.code ?? err.message}`);
    });
    proc.on("exit", () => {
      if (this.child === proc) this.drop("the ductus runtime exited");
    });
    /* A write racing the child's death raises EPIPE here; the exit or error
     * handler above already fails the request. */
    proc.stdin?.on("error", () => {});
    return proc;
  }

  private onData(chunk: Buffer) {
    this.lineBuffer += chunk.toString();
    let nl: number;
    while ((nl = this.lineBuffer.indexOf("\n")) >= 0) {
      const line = this.lineBuffer.slice(0, nl).trim();
      this.lineBuffer = this.lineBuffer.slice(nl + 1);
      if (!line) continue;
      let msg: JsonRpcResponse;
      try {
        msg = JSON.parse(line) as JsonRpcResponse;
      } catch {
        continue; /* not our response */
      }
      if (typeof msg.id === "number") {
        const settle = this.pending.get(msg.id);
        if (settle) {
          this.pending.delete(msg.id);
          settle(msg);
        }
      }
    }
  }

  private request(proc: ChildProcess, method: string, params: Record<string, Json>): Promise<JsonRpcResponse> {
    const id = this.nextId++;
    return new Promise((settle) => {
      /* Cap every request so a wedged child cannot freeze the session; the
       * deterministic server answers well under this. */
      const timer = setTimeout(() => {
        if (this.pending.delete(id)) {
          settle({ jsonrpc: "2.0", id, error: { code: -32001, message: "ductus call timed out" } });
        }
      }, REQUEST_TIMEOUT_MS);
      this.pending.set(id, (response) => {
        clearTimeout(timer);
        settle(response);
      });
      proc.stdin?.write(JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n");
    });
  }

  /** The initialized child — the MCP handshake runs once per child. A failed
   *  handshake is not kept, so the next call starts over. */
  private connect(): Promise<ChildProcess> {
    if (this.connection) return this.connection;
    const proc = this.spawnChild();
    const connection = this.request(proc, "initialize", {
      protocolVersion: MCP_PROTOCOL_VERSION,
      capabilities: {},
      clientInfo: { name: "ductus-pi-bridge", version: "1.0.0" },
    }).then((init) => {
      if (init.error) {
        proc.kill();
        throw new Error(init.error.message);
      }
      /* A notification: no id, and no response to wait for. */
      proc.stdin?.write(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }) + "\n");
      return proc;
    });
    this.connection = connection;
    connection.catch(() => {
      if (this.connection === connection) this.connection = null;
    });
    return connection;
  }

  async listTools(): Promise<McpTool[]> {
    const proc = await this.connect();
    const listed: McpTool[] = [];
    let cursor: string | undefined;
    do {
      const resp = await this.request(proc, "tools/list", cursor ? { cursor } : {});
      if (resp.error) throw new Error(`tools/list failed: ${resp.error.message}`);
      const result = resp.result as { tools?: McpTool[]; nextCursor?: string };
      listed.push(...(result.tools ?? []));
      cursor = result.nextCursor;
    } while (cursor);
    return listed;
  }

  callTool(name: string, args: Record<string, Json>): Promise<ToolResult> {
    const call = this.callChain.then(() => this.invoke(name, args));
    /* The chain continues past a failed call; the caller still sees it. */
    this.callChain = call.catch(() => undefined);
    return call;
  }

  private async invoke(name: string, args: Record<string, Json>): Promise<ToolResult> {
    const proc = await this.connect();
    const resp = await this.request(proc, "tools/call", { name, arguments: args });
    if (resp.error) return failure(`ductus: ${name} failed: ${resp.error.message}`);
    const result = (resp.result ?? {}) as {
      content?: { type?: string; text?: string }[];
      isError?: boolean;
    };
    const blocks = (result.content ?? []).map((block) =>
      block.type === "text" && typeof block.text === "string"
        ? { type: "text" as const, text: block.text }
        : { type: "text" as const, text: JSON.stringify(block) }
    );
    return { content: blocks, isError: result.isError === true };
  }
}

/** Repo root: the nearest directory, from pi's working directory upward, that
 *  holds `.ductus/` — the same anchor /ductus itself uses. pi loads extensions
 *  through jiti, where the file path is not a reliable anchor. With no
 *  `.ductus/` anywhere above, the working directory stands in, and the missing
 *  pointer is reported at the path it was looked for. */
function projectRoot(): string {
  let dir = resolve(process.cwd());
  while (true) {
    if (existsSync(join(dir, ".ductus"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) return resolve(process.cwd());
    dir = parent;
  }
}

export default function (pi: ExtensionAPI) {
  /* The pointer /ductus materializes per project; the bridge's sole contract
   * with the runtime is that path (same as every wired host). */
  const server = new DuctusServer(resolve(projectRoot(), ".ductus/bin/ductus"));
  const unavailable = (err: unknown) =>
    `ductus: ${server.pointerPath} is missing or unusable — run /ductus to re-acquire it (${reason(err)})`;

  /* Registration happens at load because the tool list comes from the
   * runtime. */
  const registration = server.listTools().then((tools) => {
    for (const tool of tools) {
      pi.registerTool({
        name: `ductus__${tool.name}`,
        label: tool.name,
        description: tool.description ?? `ductus primitive ${tool.name}`,
        parameters: tool.inputSchema ?? { type: "object" },
        execute: async (_id, params) => {
          try {
            return await server.callTool(tool.name, params ?? {});
          } catch (err) {
            return failure(unavailable(err));
          }
        },
      });
    }
  });
  /* Handled here so a failed load is never an unhandled rejection in pi's
   * process; session start reports it. */
  registration.catch(() => {});

  pi.on("session_start", async (_event, ctx) => {
    try {
      await registration;
    } catch (err) {
      ctx.ui.notify(unavailable(err), "error");
    }
  });
}
