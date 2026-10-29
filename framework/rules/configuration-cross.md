# Configuration Rules

Rules for operator-tunable values, named constants, environment variables and other configuration values, and feature flags, for backend and frontend code alike. They verify **design-time commitments** — what a spec or plan must state, and the config-loading code its affected-files snippets show; `/{project}:analyze` enforces them against feature artifacts. A command-line flag is a configuration value here when it supplies a setting that could equally come from an environment variable, a configuration file, or a default, the sources `CFG-ENV-007` orders; an argument that names what one invocation acts on, or how, belongs to the command-line interface and is left to its help output. Rule IDs follow the format `CFG-{CATEGORY}-{NNN}`. Categories: `CONST` (constants), `ENV` (environment and configuration values), `FLAG` (feature flags). Severity, ID permanence, and how a project excludes this file from review follow constitution §rules.

## CFG-CONST — Constants

### CFG-CONST-001

> Shared constants — values used across multiple modules — MUST live in a centralized location idiomatic to the project's language rather than being duplicated across modules.

**Verification:** Any spec or plan that introduces a value used by more than one module, such as a timeout, size, threshold, rate limit, format string, well-known header, or protocol version, MUST name the centralized constants location it will live in. `/{project}:analyze` flags plans that introduce such a value without naming that location, and duplicated literal definitions in plan affected-files snippets.

**Source:** "Don't Repeat Yourself" (Hunt & Thomas, *The Pragmatic Programmer*, 1999)

### CFG-CONST-002

> Module-local constants — values used only within a single module — MUST live within that module (a dedicated constants file inside the module, or at the top of the file that uses them), not in the shared constants location.

**Verification:** Any spec or plan that introduces a named constant scoped to one module MUST place it within that module; `/{project}:analyze` flags plans that propose adding a single-module value to the shared constants location.

### CFG-CONST-003

> Operator-tunable values (timeouts, retry counts, batch sizes, thresholds, rate limits, expiry durations) MUST be backed by a named constant or by a configuration value declared per `CFG-ENV-001` — an environment variable, a configuration-file key, or a command-line flag. They MUST NOT appear as bare literals in business logic.

**Verification:** Any spec or plan that introduces operator-tunable behavior MUST commit to a named constant or a configuration value for each value. `/{project}:analyze` flags plan affected-files snippets that show a numeric or string literal of operator-tunable shape (durations, counts, thresholds, rate limits) without a constant or configuration lookup. **Not flagged:** ordinary literals for local logic — loop indices, intermediate calculations, string formatting within a function.

**Source:** Martin Fowler, *Refactoring* (2nd ed.), "Replace Magic Literal"

### CFG-CONST-004

> When an operator-tunable value has a valid range, the bounds (minimum and/or maximum) MUST be expressed as named constants alongside the default constant.

**Verification:** Any spec or plan that introduces a value with a documented valid range MUST commit to a named constant for each bound, alongside the default constant. `/{project}:analyze` flags plans that propose range checks using bare literals.

**Source:** Martin Fowler, *Refactoring* (2nd ed.), "Replace Magic Literal"

## CFG-ENV — Environment and configuration values

### CFG-ENV-001

> Every configuration value the application reads — an environment variable, a key in a configuration file, or a command-line flag — MUST be declared as one of: **required with no default** (in which case `CFG-ENV-003` governs its startup validation); **optional with a default fallback defined as a named constant**; or **optional with no default**, where an absent value disables a named feature or integration and the declaration names what is disabled; under that third declaration an empty value counts as absent, as it does for a required value under `CFG-ENV-003`. Secrets MUST NOT carry an in-application default value, so a secret is declared required, or optional with no default when its absence disables the integration it authenticates. All environment variables MUST be read once at startup and the value cached; per-call reads from `os.environ` (or equivalent) are forbidden.

**Verification:** Any spec or plan that introduces a configuration value MUST declare it one of the Statement's three ways and, for an environment variable, commit to startup-time resolution. `/{project}:analyze` flags plans that propose an optional value naming neither its default constant nor the disabled feature, plans that propose a secret with an in-application default value (a fallback the application's own code supplies when the configured value is absent), and plans whose affected-files snippets show per-call env reads. A secret's value or default in a committed configuration or `.env` file (a configuration-file source in `CFG-ENV-007`'s order), or a fallback in build, dev-server or deploy configuration or a script, is `CFG-ENV-013`'s, or `CFG-ENV-010`'s when the secret reaches a client artifact (a browser bundle, a mobile app binary, or a distributed desktop or CLI build); a fallback the application's own code supplies for the same secret stays this rule's.

**Source:** Twelve-Factor App (III. Config); Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004)

### CFG-ENV-002

> Every environment variable and every configuration-file key the application reads MUST be listed in a canonical inventory maintained alongside the source code — one inventory for each kind of source, or one for both, and never two for the same source. The inventory MUST describe each value's purpose, declare whether it is required or optional, and provide a safe placeholder or default value. Acceptable forms include `.env.example` (projects that load configuration with a dotenv-style loader), a Helm chart's `values.yaml` (defaults) with its `values.schema.json` (types and required keys), a commented example of the configuration file that lists every key, a typed config module that lists every value, or an equivalent declarative manifest checked into the repository. Command-line flags are outside this rule: a command-line interface documents its flags in its own help output.

**Verification:** Any spec or plan that introduces an environment variable or a configuration-file key MUST name the inventory that lists it and include updating it in its tasks. `/{project}:analyze` flags plans that introduce either without a corresponding inventory update in affected-files, and plans that start a second inventory for a source that already has one.

**Source:** dotenv-safe README; Helm, "Charts" (Schema Files: `values.schema.json`); Helm, "Chart Best Practices: Values"

### CFG-ENV-003

> Every required configuration value — an environment variable, a configuration-file key, or a command-line flag — MUST be validated at startup. The application MUST fail fast — exit non-zero with a clear error message naming the setting, and the file for a file key — when a required value is absent: an environment variable unset or set to an empty value, a key missing from its configuration file (the file itself absent included) or set to an empty value, or a flag not given or given an empty value. A value read from more than one source is absent only when every source it is read from, resolved in `CFG-ENV-007`'s order, leaves it absent — an empty value counting as absent at each — and the error then names each source checked.

**Verification:** Any spec or plan that introduces a required configuration value MUST commit to the Statement's startup validation and error message. `/{project}:analyze` flags plans that introduce a required configuration value without a startup-validation step. This rule governs an absent value declared required under `CFG-ENV-001`: a present but malformed one is `CFG-ENV-008`'s, and an out-of-range one `CFG-ENV-005`'s.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004); dotenv README (parsing rules)

### CFG-ENV-004

> Environment variables holding time values MUST express their unit unambiguously. Two patterns are acceptable:
>
> - **Unit-in-value (preferred):** the env var value is parsed as a duration string; the parser MUST reject a non-zero value without an explicit unit.
> - **Unit-in-name:** the env var name carries a unit suffix (`_MS`, `_SECONDS`, `_MINUTES`, `_HOURS`) and the value is an integer in that unit.
>
> The corresponding default constant MUST also make the unit explicit — either by being a typed duration value (e.g., Go `5 * time.Second`, Rust `Duration::from_secs(5)`, Java `Duration.ofSeconds(5)`) or by carrying a matching unit suffix when the constant is an integer.

**Verification:** Any spec or plan that introduces a time-valued env var MUST commit to one of the two patterns — for unit-in-value, a duration parser that rejects bare non-zero numbers; for unit-in-name, the unit suffix on the env var name — and, where it has a default, to a default constant that is a typed duration or carries a unit suffix, the env var's own under unit-in-name. `/{project}:analyze` flags plans that propose `*_TIMEOUT`, `*_INTERVAL`, `*_DELAY`, `*_TTL`, etc. without committing to one of the two patterns, and plans whose default constant is a bare integer with no unit suffix.

**Source:** ISO 80000-3 (quantities and units — space and time); Yaron Minsky, "Make illegal states unrepresentable" ("Effective ML Revisited", Jane Street, 2011); Spring Boot, "Externalized Configuration"; Go, `time.ParseDuration`

### CFG-ENV-005

> Every configuration value with a documented valid range (bounds named per `CFG-CONST-004`) — an environment variable, a configuration-file key, or a command-line flag — MUST be validated against that range at startup. An out-of-range value MUST cause fail-fast with an error message naming the setting, and the file for a file key, the offending value, and the violated bound.

**Verification:** Any spec or plan that introduces a configuration value with a documented valid range MUST include a startup-validation step whose error message names what the Statement requires. `/{project}:analyze` flags plans that introduce ranged configuration values without a startup-range-check step.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004)

### CFG-ENV-006

> Configuration values flagged as sensitive (secrets, credentials, tokens, keys, PII) MUST be redacted before appearing in logs, error messages, diagnostic dumps, health-check responses, or any other operator-facing output. Each sensitive value MUST be marked as sensitive when it is loaded, and the redaction MUST occur at the config-loading layer.

**Verification:** Any spec or plan that introduces sensitive configuration MUST commit to a sensitivity marker and to a redaction mechanism applied at the config-loading layer. `/{project}:analyze` flags plans that handle secrets without naming the redaction mechanism, and affected-files snippets that log or serialize config objects without applying it. This rule governs redaction at the config-loading layer, a configuration object logged or serialized unredacted included: any other sensitive value in a log line is `security-backend.md` `BE-LOG-002`'s.

**Source:** OWASP Logging Cheat Sheet (Data to exclude)

### CFG-ENV-007

> Configuration value precedence MUST be documented and consistent across the application. The standard precedence is, from highest to lowest: command-line flag → environment variable → configuration file → in-code default constant. Deviations from this order MUST be justified in `specs/system.md`.

**Verification:** Any spec or plan that introduces a configurable value with more than one source MUST describe the resolution order and confirm it follows the standard. `/{project}:analyze` flags plans whose config-loading description departs from the standard order without an explicit justification reference in `specs/system.md`.

**Source:** Viper (Go) and Spring Boot externalized-configuration precedence orders; "Principle of Least Surprise"

### CFG-ENV-008

> Every configuration value — whether read from an environment variable, a configuration file, or a command-line flag — MUST be parsed into its declared type (integer, boolean, enumerated member, duration, URL, list) at startup. A value that is present but fails to parse MUST cause fail-fast — exit non-zero with an error naming the setting and the offending value (redacted per `CFG-ENV-006` when the setting is sensitive) — and MUST NOT silently fall back to the default constant or be coerced to a zero, empty, or `false` value.

**Verification:** Any spec or plan that introduces a configuration value MUST name its type and commit to a startup parse that fails fast on a malformed value. `/{project}:analyze` flags plans that describe falling back to the default when a value fails to parse, and flags affected-files snippets that discard a parse error, use a parser that yields `NaN`, zero, or `false` instead of an error, or use one that stops at the first invalid character and returns what it read so far. This rule governs a present but malformed value: an absent one is `CFG-ENV-003`'s, and an out-of-range one `CFG-ENV-005`'s.

**Source:** Jim Shore, "Fail Fast" (IEEE Software 21(5), 2004); Alexis King, "Parse, don't validate" (2019)

### CFG-ENV-009

> A value that varies between deployments — backing-service locators (database, cache, and queue URLs), external service endpoints, per-deploy hostnames and ports — SHOULD be supplied at deploy time through the environment (an environment variable, a mounted configuration file, or a secrets manager) rather than compiled into the code. Application code SHOULD NOT branch on the deployment environment's name (`if env == "production"`, `NODE_ENV === "staging"`) to select values or behavior; each setting that differs between deployments SHOULD be a configuration value of its own. Build tooling and libraries that key on a standard mode variable (e.g., `NODE_ENV`) MAY continue to read it; this rule governs application code.

**Verification:** Any spec or plan that introduces a value or behavior that differs between deployments SHOULD source it from deploy-time configuration under its own name. `/{project}:analyze` emits a warning when a plan or its affected-files snippets select values or behavior by comparing against an environment name, or hard-code a backing-service locator as a constant. **Not flagged:** a credential compiled into the code, which `CFG-ENV-001` owns when it is an in-code default, `CFG-ENV-010` when the code ships in a client artifact (a browser bundle, a mobile app binary, or a distributed desktop or CLI build), and `CFG-ENV-013` otherwise.

**Source:** Twelve-Factor App (III. Config; V. Build, release, run)

### CFG-ENV-010

> Every configuration value injected into an artifact delivered to end users — a browser bundle (Vite `VITE_*`, Next.js `NEXT_PUBLIC_*`, a bundler define/replace plugin such as webpack `DefinePlugin`), a mobile app binary, or a distributed desktop or CLI build — is public. Secrets MUST NOT reach such artifacts, whether injected by the build or written in client source, and the build MUST expose to client code only the variables explicitly designated for it (a public-prefix convention or an explicit allowlist), never the whole build environment. Values their issuer designs to be public (publishable API keys, OAuth client IDs for public clients, analytics IDs) MAY be injected.

**Verification:** Any spec or plan that introduces client-side build configuration, a configuration value consumed by client code, or client code that embeds a credential MUST name which variables the build exposes to the client and confirm that no secret reaches the artifact, through those variables or written in client source. `/{project}:analyze` flags plans that route a credential, private API key, signing key, or private token through a public-prefixed variable or a define plugin, or write one as a literal in client source, and flags build configurations that inject the entire environment into client code. This rule governs a secret that reaches a client artifact, wherever the secret sits before the build: in source, in version control, in the CI environment, or in a build stage's container image. A secret's in-application default value is `CFG-ENV-001`'s, while its exposure in the artifact stays this rule's; any other secret that never reaches a client artifact but sits in source, version control, a container build or another build artifact is `CFG-ENV-013`'s; and one in browser storage at run time is `security-frontend.md` `FE-STORAGE-001`'s.

**Source:** Vite, "Env Variables and Modes" (Protecting secrets) and `envPrefix` (Security Notes); Next.js, "Environment Variables" (Bundling environment variables for the browser); RFC 6749, section 2.2

### CFG-ENV-011

> Environment variables holding data sizes (byte counts such as buffer, request-body, upload, cache, and memory limits) MUST express their unit unambiguously. Two patterns are acceptable:
>
> - **Unit-in-value:** the env var value is parsed as a size string; the parser MUST reject a non-zero value without an explicit unit.
> - **Unit-in-name:** the env var name carries a unit suffix (e.g., `_BYTES`, `_KIB`, `_MIB`) and the value is an integer in that unit.
>
> Under either pattern, a unit whose multiplier differs between parsers (`KB`, `MB`, `GB`, `TB`, `k`, `M`, `G`, `T`) MUST have its multiplier — powers of 1000 or powers of 1024 — stated in the variable's inventory entry (`CFG-ENV-002`). The corresponding default constant MUST also make the unit explicit — either by being a typed size value or by carrying a matching unit suffix when the constant is an integer.

**Verification:** Any spec or plan that introduces a data-size env var MUST commit to one of the two patterns — for unit-in-value, a size parser that rejects bare non-zero numbers; for unit-in-name, the unit suffix on the env var name — and, where it has a default, to a default constant that is a typed size or carries a unit suffix, the env var's own under unit-in-name. `/{project}:analyze` flags plans that propose an env var whose value is a byte count — a body, upload, buffer, cache, file, or memory limit, however it is named — without committing to one of the two patterns, plans whose default constant is a bare integer with no unit suffix, and a `KB`, `MB`, `GB`, `TB`, `k`, `M`, `G`, or `T` unit, in a value or a name, whose multiplier the inventory does not state. **Not flagged:** a count of items, such as a batch, pool, or page size, which is not a data size; and a binary unit such as `KiB` or `MiB` whose multiplier the inventory does not state, since the Statement's list omits it.

**Source:** IEC 80000-13 (binary prefixes; NIST, "Prefixes for binary multiples"); Spring Framework, `DataSize` Javadoc and `DataUnit` Javadoc; Spring Boot, "Externalized Configuration"; Kubernetes, `Quantity` serialization format

### CFG-ENV-012

> A key in a configuration file that the application does not recognize, within the settings the application owns, SHOULD fail startup with an error naming the key and the file. Where one file must be read by more than one version of the application at once — during a rolling deploy, or when older binaries read the same file — an unrecognized key SHOULD instead produce a startup warning that names it. An unrecognized key SHOULD NOT be silently ignored. Sections of a shared file that belong to other tools MAY pass unexamined.

**Verification:** Any spec or plan that introduces a configuration file, or a section of one, SHOULD state how its loader treats an unrecognized key — failing startup, or warning by name where more than one application version reads the file. `/{project}:analyze` emits a warning when a plan's config-loading description or affected-files snippets decode a configuration file with a decoder left at its ignore-unknown-keys default and no warning for the keys it drops. **Not flagged:** an unrecognized variable in the process environment.

**Source:** serde, "Container attributes" (`deny_unknown_fields`); Go `encoding/json` (`Decoder.DisallowUnknownFields`); Spring Boot, `@ConfigurationProperties` (`ignoreUnknownFields`)

### CFG-ENV-013

> Secrets (API keys, database credentials, encryption keys, signing keys, third-party tokens) MUST NOT be written in source code or a container-build manifest, committed to version control, passed to a container build as a build argument or environment variable, or placed into a container image or other build artifact. A build step that needs a secret MAY receive it through the build tool's secret mechanism, such as a Docker secret mount, which makes it available only for the duration of the build instruction that uses it. A secret MAY be committed encrypted when each service that uses it in each environment has its own copy, each environment has its own value, decryption key and decryptor, no developer can decrypt any copy, nothing but that service or the controller or sidecar decrypting for it can decrypt a copy, and every decryption key is held outside version control and every artifact (one `SealedSecret` per service in the default `strict` scope, which binds it to its name and namespace, sealed for that environment's own controller, whose private key no developer can read, for example).

**Verification:** Any spec or plan that introduces a secret, or a build, container-build, dev-server or deploy configuration that uses one, MUST keep the secret out of every place the Statement names. `/{project}:analyze` flags plans that commit a secret to the repository, in a `.env` or other configuration file included; affected-files snippets that write a secret as a literal in source, in build or dev-server configuration, or in a script; container builds that pass a secret through an `ENV` or `ARG` instruction or a build argument, or `COPY` a secret file into the image; and builds that package a secret into an artifact. A plan that commits an encrypted secret MUST show, for each copy, that it meets every condition of the Statement's permission, naming its environment, the one service that uses it, what decrypts it, and where each key that can decrypt it is held and who can use it, and is flagged when it omits or contradicts any condition for any copy. **Not flagged:** a secret a build step receives through the build tool's secret mechanism and writes into no image or artifact; the commit of a secret encrypted as the Statement permits, though not its placement into an image or artifact; and a public key or certificate, such as an age recipient in `.sops.yaml` or a certificate from `kubeseal --fetch-cert`. A secret's in-application default value (a fallback the application's own code supplies when the configured value is absent) is `CFG-ENV-001`'s wherever the source holding it goes, its commit and any image or build artifact built from it included. Of the secret's other placements, those of a secret that reaches a client artifact (a browser bundle, a mobile app binary, or a distributed desktop or CLI build) are `CFG-ENV-010`'s, wherever they sit before the build, and the rest stay this rule's, a `Dockerfile` `ENV`, a value or default in a committed configuration file (a configuration-file source in `CFG-ENV-007`'s order), and a fallback in build, dev-server or deploy configuration or a script included.

**Source:** OWASP Secrets Management Cheat Sheet (3.6 Pipeline Created Secrets; 5.1 Injection of Secrets; 8.1 General detection approaches); Docker documentation, "Build variables" and "Build secrets"; Bitnami Sealed Secrets README; CWE-798 (Use of Hard-coded Credentials)

## CFG-FLAG — Feature flags

### CFG-FLAG-001

> Every feature flag MUST have a declared default value, and flag evaluation MUST resolve to that default — never throw, block indefinitely, or yield an undefined state — on any evaluation failure, such as a flag source that is unreachable or times out, or a value of the wrong type.

**Verification:** Any spec or plan that introduces a feature flag MUST name its default value and commit to evaluation that returns it on any evaluation failure. `/{project}:analyze` flags plans that introduce a flag without a named default, and flags affected-files snippets whose flag lookup can throw or yield an undefined value. The evaluation timeout is an operator-tunable value governed by `CFG-CONST-003`.

**Source:** OpenFeature specification, Flag Evaluation API (Requirement 1.4.10; Requirement 1.3.4)

### CFG-FLAG-002

> A feature flag introduced to gate a release, rollout, or experiment SHOULD be recorded with an owner and a removal criterion when it is introduced, and its removal SHOULD be tracked as a work item from that point. Long-lived operational flags (kill switches, entitlement toggles) MAY omit the removal criterion when they are declared permanent.

**Verification:** Any spec or plan that introduces a release, rollout, or experiment flag SHOULD commit to each obligation in the Statement. `/{project}:analyze` emits a warning when a plan introduces such a flag with neither a removal criterion nor a declaration that it is permanent.

**Source:** Pete Hodgson, "Feature Toggles (aka Feature Flags)" (martinfowler.com)
