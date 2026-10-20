# Configuration Rules

Enforceable rules for operator-tunable values, named constants, environment variables and other configuration values, and feature flags. These rules apply to all projects adopting `ductus`, regardless of whether the code is backend or frontend — configuration is the same problem on both surfaces.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `CFG-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `CONST` (constants), `ENV` (environment and configuration values), `FLAG` (feature flags). See `specs/017-derive-dont-ask/data-model.md` for the full schema.

These rules verify **design-time commitments** — what a spec or plan must state, and the config-loading code its affected-files snippets show; `/{project}:analyze` enforces them against feature artifacts.

Cross-cutting (`-cross.md`) rule files load for every stack: `[rules] surfaces` cannot exclude them, and `/{project}:analyze` loads every rule file present. A project can exclude this file from `/{project}:review` with a `.ductus/config.toml` `[[review.disabled-rule-files]]` entry and its mandatory `reason`; listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## CFG-CONST — Constants

### CFG-CONST-001

> Shared constants — values used across multiple modules — MUST live in a centralized location idiomatic to the project's language (e.g., `shared/constants/` in JavaScript/TypeScript, `internal/constants/` in Go, a top-level constants module in Python) rather than being duplicated across modules.

**Rationale:** Cross-cutting defaults that drift between modules produce silent inconsistencies — a timeout treated as 30s in one place and 60s in another. A single location makes the canonical value findable and auditable.

**Verification:** Any spec or plan that introduces a value used by more than one module (timeouts, sizes, thresholds, rate limits, format strings, well-known headers, protocol versions) MUST name the centralized constants location it will live in. `/{project}:analyze` flags plans that introduce cross-module values without naming the shared-constants location, and flags duplicated literal definitions in plan affected-files snippets.

**Source:** "Don't Repeat Yourself" (Hunt & Thomas, *The Pragmatic Programmer*, 1999)

### CFG-CONST-002

> Module-local constants — values used only within a single module — MUST live within that module (a dedicated constants file inside the module, or at the top of the file that uses them), not in the shared constants location.

**Rationale:** Co-locating a single module's constants with that module keeps the module self-contained and avoids coupling unrelated modules through a shared import. The shared constants location stays focused on values that genuinely cross modules.

**Verification:** Any spec or plan that introduces a named constant scoped to one module MUST place it within that module, not in the shared location. `/{project}:analyze` flags plans that propose adding single-module values to the shared constants location.

### CFG-CONST-003

> Operator-tunable values (timeouts, retry counts, batch sizes, thresholds, rate limits, expiry durations) MUST be backed by a named constant or an environment variable. They MUST NOT appear as bare literals in business logic.

**Rationale:** Bare literals scattered across the codebase are invisible to operators, hard to audit, and easy to leave inconsistent during tuning. A single named source of truth makes the value findable, changeable, and auditable.

**Verification:** Any spec or plan that introduces operator-tunable behavior MUST commit to a named constant or env var for each value. `/{project}:analyze` flags plan affected-files snippets that show numeric or string literals of operator-tunable shape (durations, counts, thresholds, rate limits) without a constant or env var lookup. Ordinary literals used for local logic — loop indices, intermediate calculations, string formatting within a function — are out of scope.

**Source:** Martin Fowler, *Refactoring* (2nd ed.), "Replace Magic Literal" (formerly "Replace Magic Number with Symbolic Constant")

### CFG-CONST-004

> When an operator-tunable value has a valid range, the bounds (minimum and/or maximum) MUST be expressed as named constants alongside the default constant.

**Rationale:** Bare-literal bounds in validation code are invisible to operators looking for the safe range, drift between the defining spec and the implementation, and produce inconsistent enforcement when the same value is checked in more than one place. Naming the bounds makes the safe range discoverable and enforceable from one place.

**Verification:** Any spec or plan that introduces a value with a documented valid range MUST commit to named constants for each bound (e.g., `MinHTTPReadTimeoutSeconds = 1`, `MaxHTTPReadTimeoutSeconds = 300`) alongside the default constant. `/{project}:analyze` flags plans that propose range checks using bare literals.

**Source:** Martin Fowler, *Refactoring* (2nd ed.), "Replace Magic Literal"

## CFG-ENV — Environment and configuration values

### CFG-ENV-001

> Every configuration value the application reads — an environment variable, a key in a configuration file, or a command-line flag — MUST be declared as one of: **required with no default** (in which case `CFG-ENV-003` governs its startup validation); **optional with a default fallback defined as a named constant**; or **optional with no default**, where an absent value disables a named feature or integration (an error reporter's DSN, the API key of an integration a deployment may not use) and the declaration names what is disabled; under that third declaration an empty value counts as absent, as it does for a required value under `CFG-ENV-003`. Secrets MUST NOT carry an in-application default value (see `BE-DATA-003`), so a secret is declared required, or optional with no default when its absence disables the integration it authenticates. All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.

**Rationale:** Repeated env reads are a silent dependency on process state, slow hot paths, and make defaults invisible to readers. Reading once at startup, falling back to a constant for optional values, and failing fast for required values produces predictable behavior, makes defaults discoverable, and keeps the runtime fast. Excluding secrets from in-code defaults prevents accidentally shipping placeholder credentials. The third declaration exists for the same reason: requiring a secret that only an optional integration uses would force every deployment that does not use it to supply a placeholder credential, and naming the disabled feature makes its off state a documented decision rather than a side effect of a missing value. The declaration covers every source, because a key absent from a configuration file or a flag left off a command line raises the same question an unset variable does — a default, a disabled feature, or a broken deployment; the read-once requirement is about the process environment.

**Verification:** Any spec or plan that introduces a configuration value — an environment variable, a configuration-file key, or a command-line flag — MUST declare it required-no-default, optional-with-default-constant, or optional-no-default with the feature its absence disables, and for an environment variable MUST commit to startup-time resolution. `/{project}:analyze` flags plans that propose optional values without naming either the default constant or the disabled feature, plans that propose secrets with a hard-coded default, and plans that show per-call env reads in affected-files snippets.

**Source:** Twelve-Factor App (III. Config) — its litmus test: the codebase could be made open source at any moment without compromising any credentials; Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004) — a configuration getter that hides a missing property behind a default

### CFG-ENV-002

> Every environment variable and every configuration-file key the application reads MUST be listed in a canonical inventory maintained alongside the source code — one inventory for each kind of source, or one for both, and never two for the same source. The inventory MUST describe each value's purpose, declare whether it is required or optional, and provide a safe placeholder or default value. Acceptable forms include `.env.example` (projects that load configuration with a dotenv-style loader), a Helm chart's `values.yaml` (defaults) with its `values.schema.json` (types and required keys), a commented example of the configuration file that lists every key, a typed config module that lists every value, or an equivalent declarative manifest checked into the repository. Command-line flags are outside this rule: a command-line interface documents its flags in its own help output.

**Rationale:** Operators discover required configuration by reading this inventory. Variables introduced in code but absent from the inventory produce silent runtime failures in fresh deployments and obscure the application's true configuration surface. The form of the inventory follows the deployment model — `.env.example` for processes, Helm values for Kubernetes, etc. — but the single-source-of-truth requirement does not. A configuration-file key is discovered the same way, so it is inventoried too, and a commented example of the file serves as its inventory, as a Helm chart's `values.yaml` does for the chart's values. A flag is read where it is passed, in the help output of the command that takes it, so it is left there.

**Verification:** Any spec or plan that introduces an environment variable or a configuration-file key MUST name the inventory that lists it and include updating it as part of its tasks. `/{project}:analyze` flags plans that introduce either without a corresponding inventory update in affected-files.

**Source:** dotenv-safe README (`.env.example`, committed with the project, names the variables the application needs); Helm, "Charts" (`values.yaml` holds a chart's defaults; Schema Files: `values.schema.json`)

### CFG-ENV-003

> Every required configuration value — an environment variable, a configuration-file key, or a command-line flag — MUST be validated at startup. The application MUST fail fast — exit non-zero with a clear error message naming the setting, and the file for a file key — when a required value is absent: an environment variable unset or set to an empty value, a key missing from its configuration file (the file itself absent included) or set to an empty value, or a flag not given or given an empty value (a required value has no default to fall back to — `CFG-ENV-001`). A value read from more than one source is absent only when every source it is read from, resolved in `CFG-ENV-007`'s order, leaves it absent — an empty value counting as absent at each — and the error then names each source checked.

**Rationale:** Unvalidated config produces partial-failure modes that surface only at first use of the value, often deep in a request path. Fail-fast at startup turns a confusing intermittent error into an obvious deployment-time failure. An empty value is the common disguise of a missing one — a `FOO=` line in an env file, or a templated value that rendered blank — and a presence check that accepts it passes the deployment through to that same first-use failure. A required key missing from a configuration file is the same defect reached through another source: in the Fail Fast article's worked example, a typo in the configuration file leaves `maxConnections` absent, and a reader that returns a default for it lets the system run on into slowdowns that take days to trace, where failing names the property and the file it was not found in. That reader treats the key as optional; this rule applies once the key is declared required (`CFG-ENV-001`), which is the article's fix. A present but malformed value is `CFG-ENV-008`; an out-of-range one is `CFG-ENV-005`.

**Verification:** Any spec or plan that introduces a required configuration value — an environment variable, a configuration-file key, or a command-line flag — MUST commit to startup-time validation that treats an absent or empty value as missing — at every source a multi-source value is read from — and names the failing setting, and for a file key the file, in its error message. `/{project}:analyze` flags plans that introduce required configuration values without a startup-validation step.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004); dotenv README (parsing rules: `EMPTY=` becomes an empty string, not an unset variable)

### CFG-ENV-004

> Environment variables holding time values MUST express their unit unambiguously. Two patterns are acceptable:
>
> - **Unit-in-value (preferred):** the env var value is parsed as a duration string (e.g., `30s`, `5m`, `100ms`); the parser MUST reject a non-zero value without an explicit unit so a bare `30` is a parse error rather than an ambiguous quantity.
> - **Unit-in-name:** the env var name carries a unit suffix (`_MS`, `_SECONDS`, `_MINUTES`, `_HOURS`) and the value is an integer in that unit.
>
> The corresponding default constant MUST also make the unit explicit — either by being a typed duration value (e.g., Go `5 * time.Second`, Rust `Duration::from_secs(5)`, Java `Duration.ofSeconds(5)`) or by carrying a matching unit suffix when the constant is an integer (e.g., `DEFAULT_SHUTDOWN_TIMEOUT_SECONDS = 30`).

**Rationale:** Unit-less time variables produce off-by-1000x bugs — treating milliseconds as seconds, or vice versa. Parsers disagree on the unit a bare number carries: Spring Boot, for one, binds a unit-less `Duration` property as milliseconds unless the field declares another unit. The unit-in-value pattern eliminates the ambiguity at the source: the operator has to type a unit, and the parser rejects a value that lacks one. A bare `0` is exempt because zero is the same in every unit — Go's `time.ParseDuration` accepts `0` and rejects `30` with `missing unit in duration`. The unit-in-name pattern remains valid for cases where an integer is preferable (e.g., scripting environments that struggle with the duration-string format), but it shifts the safety burden to operator discipline rather than the parser.

**Verification:** Any spec or plan that introduces a time-valued env var MUST commit to one of the two patterns. For unit-in-value, the loader MUST use a duration parser that rejects bare non-zero numbers; for unit-in-name, both the env var name and the default constant name MUST carry the unit suffix. `/{project}:analyze` flags plans that propose `*_TIMEOUT`, `*_INTERVAL`, `*_DELAY`, `*_TTL`, etc. without committing to one of the two patterns.

**Source:** ISO 80000-3 (quantities and units — space and time); Yaron Minsky, "Make illegal states unrepresentable" ("Effective ML Revisited", Jane Street, 2011)

### CFG-ENV-005

> Every configuration value with a documented valid range (bounds named per `CFG-CONST-004`) — an environment variable, a configuration-file key, or a command-line flag — MUST be validated against that range at startup. An out-of-range value MUST cause fail-fast with an error message naming the setting, and the file for a file key, the offending value, and the violated bound.

**Rationale:** A misconfigured timeout, retry count, pool size, or threshold can produce silent degraded behavior at runtime — requests timing out under load, exhausting connections, retrying forever — long after deployment. Catching out-of-range values at startup turns a confusing production incident into an obvious deployment-time failure. The check covers every source, as `CFG-ENV-003`'s presence check does: an out-of-range key in a configuration file or flag on a command line degrades the system the same way an out-of-range environment variable does.

**Verification:** Any spec or plan that introduces a configuration value with a documented valid range — an environment variable, a configuration-file key, or a command-line flag — MUST include a startup-validation step that names the failing setting, and the file for a file key, the offending value, and the violated bound in the error message. `/{project}:analyze` flags plans that introduce ranged configuration values without a startup-range-check step.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004)

### CFG-ENV-006

> Configuration values flagged as sensitive (secrets, credentials, tokens, keys, PII) MUST be redacted before appearing in logs, error messages, diagnostic dumps, health-check responses, or any other operator-facing output. The redaction MUST occur at the config-loading layer so downstream code cannot accidentally bypass it.

**Rationale:** Configuration objects are routinely serialized for debugging — a `printf("%+v", config)`, a `console.log(config)`, an uncaught exception that includes the config in its message — and any of those paths can publish secrets to logs, dashboards, and on-call channels. Marking sensitivity at load time and redacting in a single chokepoint (a `Redacted[T]` wrapper type, a `__repr__` override, a logger filter) closes the leak at its source. This rule places the redaction at the config layer; what any log line may contain, whatever its origin, is `security-backend.md` `BE-LOG-002`.

**Verification:** Any spec or plan that introduces sensitive configuration MUST commit to a sensitivity marker and to a redaction mechanism applied at the config-loading layer. `/{project}:analyze` flags plans that handle secrets without naming the redaction wrapper, and flags affected-files snippets that log or serialize config objects without applying the wrapper.

**Source:** OWASP Logging Cheat Sheet (Data to exclude); Yaron Minsky, "Make illegal states unrepresentable" ("Effective ML Revisited", Jane Street, 2011)

### CFG-ENV-007

> Configuration value precedence MUST be documented and consistent across the application. The standard precedence is, from highest to lowest: command-line flag → environment variable → configuration file → in-code default constant. Deviations from this order MUST be justified in `specs/system.md`.

**Rationale:** Inconsistent precedence — some values reading the env first, others reading the file first — produces baffling deployment incidents where a flag "doesn't take effect" because a stale env value silently wins. A documented, uniform order makes the resolved value predictable and debuggable.

**Verification:** Any spec or plan that introduces a configurable value with more than one source (e.g., both a flag and an env var, both an env var and a file entry) MUST describe the resolution order and confirm it follows the standard. `/{project}:analyze` flags plans whose config-loading description departs from the standard order without an explicit justification reference in `specs/system.md`.

**Source:** Viper (Go) and Spring Boot externalized-configuration precedence orders, both flag over environment over file over default; "Principle of Least Surprise"

### CFG-ENV-008

> Every configuration value — whether read from an environment variable, a configuration file, or a command-line flag — MUST be parsed into its declared type (integer, boolean, enumerated member, duration, URL, list) at startup. A value that is present but fails to parse MUST cause fail-fast — exit non-zero with an error naming the setting and the offending value (redacted per `CFG-ENV-006` when the setting is sensitive) — and MUST NOT silently fall back to the default constant or be coerced to a zero, empty, or `false` value.

**Rationale:** JavaScript's `parseInt` returning `NaN` for `abc` and `30` for `30s`, `Number("")` returning `0`, a discarded `strconv.Atoi` error (which comes with a `0`), or a boolean parser that reads every string other than `true` as false (Java's `Boolean.parseBoolean`) turns an operator's typo or blank value into silently different behavior — the default, a zero, or the opposite of what was meant — with nothing to say the setting was ignored. `CFG-ENV-003` catches a missing value and `CFG-ENV-005` an out-of-range one; this rule closes the remaining gap, a value that is present but malformed. Parsing at the boundary also means the rest of the code handles a typed value and never re-parses a string.

**Verification:** Any spec or plan that introduces a configuration value MUST name its type and commit to a startup parse that fails fast on a malformed value. `/{project}:analyze` flags plans that describe falling back to the default when a value fails to parse, and flags affected-files snippets that discard a parse error, use a parser that yields `NaN`, zero, or `false` instead of an error, or use one that stops at the first invalid character and returns what it read so far.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004); Alexis King, "Parse, don't validate" (2019)

### CFG-ENV-009

> A value that varies between deployments — backing-service locators (database, cache, and queue URLs), external service endpoints, per-deploy hostnames and ports — SHOULD be supplied at deploy time through the environment (an environment variable, a mounted configuration file, or a secrets manager) rather than compiled into the code. Application code SHOULD NOT branch on the deployment environment's name (`if env == "production"`, `NODE_ENV === "staging"`) to select values or behavior; each setting that differs between deployments SHOULD be a configuration value of its own. Build tooling and libraries that key on a standard mode variable (e.g., `NODE_ENV`) MAY continue to read it; this rule governs application code.

**Rationale:** Per-deploy values compiled into code mean a new deployment target needs a code change and a rebuild, and the artifact tested in staging is not the one that runs in production. Branching on the environment's name bundles unrelated settings into a few named groups that multiply as deploys do — a second staging region, a developer's local stack — and hides which individual setting actually differs. Granular, orthogonal settings keep each difference explicit and let one artifact run anywhere. Credentials are governed separately, at MUST, by `security-backend.md` `BE-DATA-003`.

**Verification:** Any spec or plan that introduces a value or behavior that differs between deployments SHOULD source it from deploy-time configuration under its own name. `/{project}:analyze` emits a warning when a plan or its affected-files snippets select values or behavior by comparing against an environment name, or hard-code a backing-service locator as a constant.

**Source:** Twelve-Factor App (III. Config; V. Build, release, run)

### CFG-ENV-010

> Every configuration value injected into an artifact delivered to end users — a browser bundle (Vite `VITE_*`, Next.js `NEXT_PUBLIC_*`, a bundler define/replace plugin such as webpack `DefinePlugin`), a mobile app binary, or a distributed desktop or CLI build — is public. Secrets MUST NOT be injected into such artifacts, and the build MUST expose to client code only the variables explicitly designated for it (a public-prefix convention or an explicit allowlist), never the whole build environment. Values their issuer designs to be public (publishable API keys, OAuth client IDs for public clients, analytics IDs) MAY be injected.

**Rationale:** A build-time substitution writes the value into the shipped code as a literal, readable by anyone who downloads the bundle or unpacks the binary, and it cannot be rotated without rebuilding and redistributing every copy. Injecting the whole environment (`process.env` serialized into a define plugin) publishes every secret present on the build machine at once. Secrets that client features need stay on a server the client calls. This rule covers what the build bakes into a client artifact; `security-backend.md` `BE-DATA-003` covers source, version control, and container images, and `security-frontend.md` `FE-STORAGE-001` covers browser storage at run time.

**Verification:** Any spec or plan that introduces client-side build configuration, or a configuration value consumed by client code, MUST name which variables the build exposes to the client and confirm none is a secret. `/{project}:analyze` flags plans that route a credential, private API key, signing key, or private token through a public-prefixed variable or a define plugin, and flags build configurations that inject the entire environment into client code.

**Source:** Vite, "Env Variables and Modes" (Protecting secrets) and `envPrefix` (Security Notes: an empty prefix would expose every variable, so Vite refuses it); Next.js, "Environment Variables" (Bundling environment variables for the browser); RFC 6749, section 2.2 (the client identifier is not a secret)

### CFG-ENV-011

> Environment variables holding data sizes (byte counts such as buffer, request-body, upload, cache, and memory limits) MUST express their unit unambiguously. Two patterns are acceptable:
>
> - **Unit-in-value:** the env var value is parsed as a size string (e.g., `512KiB`, `10MiB`); the parser MUST reject a non-zero value without an explicit unit.
> - **Unit-in-name:** the env var name carries a unit suffix (e.g., `_BYTES`, `_KIB`, `_MIB`) and the value is an integer in that unit.
>
> Under either pattern, a unit whose multiplier differs between parsers (`KB`, `MB`, `GB`, `TB`, `k`, `M`, `G`, `T`) MUST have its multiplier — powers of 1000 or powers of 1024 — stated in the variable's inventory entry (`CFG-ENV-002`). The corresponding default constant MUST also make the unit explicit — either by being a typed size value or by carrying a matching unit suffix when the constant is an integer (e.g., `DEFAULT_MAX_BODY_BYTES = 1048576`).

**Rationale:** A unit-less size is read in whatever unit the parser assumes — Spring Boot reads one bound to a `DataSize` property as bytes unless the field declares another unit — so an operator's intended limit can land a factor of 1,024 or more from what they meant. That is far enough above it to leave a body or upload limit open to resource exhaustion, or far enough below to reject legitimate traffic. The suffixes disagree as well: Spring's `DataSize` defines `1MB` as 1,048,576 bytes and `1TB` as 2^40, while a Kubernetes quantity's `k`, `M`, `G` and `T` are decimal multipliers and `Ki`, `Mi`, `Gi` and `Ti` binary ones. The IEC binary prefixes (`Ki`, `Mi`, `Gi`) denote powers of 1024 and nothing else, so a binary unit needs no statement; a decimal-looking one is only as unambiguous as the inventory makes it. This is `CFG-ENV-004`'s reasoning applied to sizes.

**Verification:** Any spec or plan that introduces a data-size env var MUST commit to one of the two patterns and, for a unit whose multiplier differs between parsers, state the multiplier in the inventory entry. For unit-in-value, the loader MUST use a size parser that rejects bare non-zero numbers; for unit-in-name, both the env var name and the default constant name MUST carry the unit suffix. `/{project}:analyze` flags plans that propose an env var whose value is a byte count — a body, upload, buffer, cache, file, or memory limit, however it is named — without committing to one of the two patterns (a count of items, such as a batch, pool, or page size, is not a data size), and flags a `KB`, `MB`, `GB`, `TB`, `k`, `M`, `G`, or `T` unit, in a value or a name, whose multiplier the inventory does not state.

**Source:** IEC 80000-13 (binary prefixes; NIST, "Prefixes for binary multiples"); Spring Framework, `DataSize` Javadoc (`1MB` is 1,048,576 bytes) and `DataUnit` Javadoc (`TB` is 2^40 bytes); Spring Boot, "Externalized Configuration" (a unit-less `DataSize` value is bytes unless `@DataSizeUnit` says otherwise); Kubernetes, `Quantity` serialization format (decimal `k`, `M`, `G`, `T` beside binary `Ki`, `Mi`, `Gi`, `Ti`)

### CFG-ENV-012

> A key in a configuration file that the application does not recognize, within the settings the application owns, SHOULD fail startup with an error naming the key and the file. Where one file must be read by more than one version of the application at once — during a rolling deploy, or when older binaries read the same file — an unrecognized key SHOULD instead produce a startup warning that names it. An unrecognized key SHOULD NOT be silently ignored. Sections of a shared file that belong to other tools MAY pass unexamined.

**Rationale:** A misspelled key — `max_conections` for `max_connections` — is a setting the operator believes is applied, and a loader that drops unknown keys applies the default in its place with nothing to say so: the silent fallback `CFG-ENV-008` forbids for a malformed value, reached through the key instead of the value. Common decoders drop unknown keys unless told otherwise — serde unless the type sets `deny_unknown_fields`, Go's `encoding/json` unless the decoder calls `DisallowUnknownFields`, Spring Boot's `@ConfigurationProperties` while `ignoreUnknownFields` keeps its default of `true` — so rejecting one is a choice the loader has to make. A warning rather than a failure keeps a rolling deploy working when a newer file reaches an older binary. The process environment is out of scope: it carries variables for every program, so an unrecognized variable is not evidence of a mistake.

**Verification:** Any spec or plan that introduces a configuration file, or a section of one, SHOULD state how its loader treats an unrecognized key — failing startup, or warning by name where more than one application version reads the file. `/{project}:analyze` emits a warning when a plan's config-loading description or affected-files snippets decode a configuration file with a decoder left at its ignore-unknown-keys default and no warning for the keys it drops.

**Source:** serde, "Container attributes" (`deny_unknown_fields`: unknown fields are ignored by default); Go `encoding/json` (`Decoder.DisallowUnknownFields`); Spring Boot, `@ConfigurationProperties` (`ignoreUnknownFields`, default `true`: "An unknown field could be a sign of a mistake")

## CFG-FLAG — Feature flags

### CFG-FLAG-001

> Every feature flag MUST have a declared default value, and flag evaluation MUST resolve to that default — never throw, block indefinitely, or yield an undefined state — when the flag source is unreachable, times out, or returns a value of the wrong type.

**Rationale:** A flag lookup sits on the code path it gates, so a flag service outage or a mistyped flag value that surfaces as an exception takes that path down with it — often every request at once. A declared default makes the degraded state a decision recorded when the flag was introduced, typically the established behavior, rather than whatever the client library does on error.

**Verification:** Any spec or plan that introduces a feature flag MUST name its default value and commit to evaluation that returns it on any evaluation failure. `/{project}:analyze` flags plans that introduce a flag without a named default, and flags affected-files snippets whose flag lookup can throw or yield an undefined value. The evaluation timeout is an operator-tunable value governed by `CFG-CONST-003`.

**Source:** OpenFeature specification, Flag Evaluation API (Requirement 1.4.10: evaluation never throws and returns the default on abnormal execution; Requirement 1.3.4: a value of the wrong type is abnormal execution, answered with the default)

### CFG-FLAG-002

> A feature flag introduced to gate a release, rollout, or experiment SHOULD be recorded with an owner and a removal criterion when it is introduced, and its removal SHOULD be tracked as a work item from that point. Long-lived operational flags (kill switches, entitlement toggles) MAY omit the removal criterion when they are declared permanent.

**Rationale:** Every flag at least doubles the states the gated code can be in, and a flag left in place after its rollout completes keeps the dead branch, its tests, and its configuration alive indefinitely. Recording the owner and the exit condition up front, and scheduling the removal, keeps the flag inventory small and stops an old flag from being reused for a new meaning.

**Verification:** Any spec or plan that introduces a release, rollout, or experiment flag SHOULD name the flag's owner and removal criterion and include a removal work item. `/{project}:analyze` emits a warning when a plan introduces such a flag with neither a removal criterion nor a declaration that it is permanent.

**Source:** Pete Hodgson, "Feature Toggles (aka Feature Flags)" (martinfowler.com)
