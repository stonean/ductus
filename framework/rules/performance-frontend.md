# Performance Rules — Frontend

Enforceable performance rules for browser-rendered UI. These rules apply to any project with a web frontend adopting `ductus`. They derive from Google Core Web Vitals, browser-vendor performance guidance, and HTTP delivery best practices.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `FE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `VITALS` (Core Web Vitals budgets and measurement), `BUNDLE` (JavaScript/CSS bundle size), `IMAGE` (image delivery), `LOAD` (resource loading, prioritization, HTTP and back/forward caching, and layout reservation), `FONT` (web font discipline), `MAIN` (main-thread work). See `specs/008-security-rules/data-model.md` for the full schema.

These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts, including code snippets a plan quotes for its affected files.

Projects without a frontend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `frontend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## FE-VITALS — Core Web Vitals Budgets and Measurement

### FE-VITALS-001

> The project MUST publish documented performance budgets for the three Core Web Vitals — Largest Contentful Paint (LCP), Interaction to Next Paint (INP), and Cumulative Layout Shift (CLS) — measured at the 75th percentile of real-user page loads, segmented by mobile and desktop. Default budgets at Google's "Good" threshold: LCP ≤ 2.5s, INP ≤ 200ms, CLS ≤ 0.1. Stricter project-specific budgets are encouraged; looser budgets MUST be justified in `specs/system.md`.

**Rationale:** Core Web Vitals measure the loading, responsiveness, and visual stability users perceive. web.dev's "The business impact of Core Web Vitals" collects case studies in which improving them coincided with gains in sales, ad revenue, and session length, and Google Search Central says good Core Web Vitals, with other page experience aspects, align with what its core ranking systems seek to reward. Publishing budgets makes performance a falsifiable commitment instead of an aspirational nice-to-have. The p75 basis, segmented by device class, means a budget holds only when at least three in four page loads on both mobile and desktop meet it, so a slow tail cannot hide behind a healthy median or a fast desktop average. INP replaced First Input Delay (FID) as the responsiveness vital on March 12, 2024; a budget or dashboard still keyed on FID tracks a retired metric.

**Verification:** Any spec or plan that introduces or modifies user-facing UI MUST commit to maintaining the documented Web Vitals budgets. `/{project}:analyze` flags frontend-UI specs that omit the budget commitment, flags specs that propose looser thresholds without a `specs/system.md` justification reference, and flags budgets that name FID in place of INP.

**Source:** web.dev "Web Vitals", web.dev "Defining the Core Web Vitals metrics thresholds", web.dev "Interaction to Next Paint becomes a Core Web Vital on March 12", web.dev "The business impact of Core Web Vitals", Google Search Central "Understanding Core Web Vitals and Google search results"

### FE-VITALS-002

> Web Vitals MUST be measured continuously on real users (RUM), not only via synthetic CI runs. The application MUST emit `web-vitals` (or equivalent) measurements to a backend the team monitors. Synthetic Lighthouse runs in CI are complementary — they catch regressions before deploy — but MUST NOT be the sole measurement source.

**Rationale:** A synthetic run measures one fixed device and network profile from one location — by default Lighthouse applies a 4x CPU slowdown to emulate a mid-tier mobile device and simulates a slow 4G connection — with no real user input. A default (navigation-mode) Lighthouse run loads the page without interacting with it, so it reports no INP; Total Blocking Time stands in as a lab proxy that often correlates with INP but is not a substitute, and a scripted timespan run measures only the interactions its script performs. Field data reflects what a lab run fixes in place: the actual devices, network conditions, and locations of real users, and the resources they already hold in cache. Without RUM, you optimize for a benchmark and ship regressions to users.

**Verification:** Any spec or plan covering frontend instrumentation, observability, or CI performance MUST commit to (a) RUM emission of Web Vitals and (b) a monitored backend for them. `/{project}:analyze` flags performance specs that describe only synthetic measurement without a RUM commitment.

**Source:** web.dev "Why lab and field data can be different (and what to do about it)", web.dev "Web Vitals", web.dev "Total Blocking Time (TBT)", Lighthouse throttling documentation, `web-vitals` JavaScript library documentation

## FE-BUNDLE — Bundle Size

### FE-BUNDLE-001

> The project MUST publish JavaScript and CSS size budgets for each route or entry point in `specs/system.md`, measured as transferred (compressed) bytes. A bundle that exceeds its budget MUST NOT ship until the budget is raised in `specs/system.md`.

**Rationale:** JavaScript costs more than its download: the browser then parses, compiles, and executes it on the main thread, which delays user interaction while it runs and costs most on phones with slow CPUs. CSS is render-blocking — the browser renders no content until it has built the CSSOM — so stylesheet size adds directly to the time to first render. Without an enforced budget, bundles grow unbounded as dependencies are added — and the regression is invisible until users complain. Holding what ships to the budget, or to a budget deliberately raised, makes every increase a decision rather than drift; `FE-BUNDLE-003` recommends automating the check so the cost of each dependency addition shows on the change that adds it. The rule sets no default figure, because a fitting budget depends on the devices and networks a project targets. Published examples show the scale: Addy Osmani's "Start Performance Budgeting" gives "< 170KB JS (min/gzip) for mobile" as an example device-class budget, and web.dev's "Your first performance budget" suggests keeping critical-path resources under 170 KB in total, with example rows of 100 KB JavaScript and 10 KB CSS for a Moto G4 on Slow 3G and 300 KB JavaScript and 50 KB CSS for a desktop on WiFi.

**Verification:** Any spec or plan that introduces frontend code, dependencies, or build configuration MUST commit to per-entry-point JavaScript and CSS budgets and to how exceedance is caught before the change ships. `/{project}:analyze` flags frontend projects with no budgets published in `specs/system.md`, and flags dependency-adding specs that omit a bundle-impact estimate against the budget.

**Source:** Addy Osmani, "Start Performance Budgeting" (addyosmani.com, 2018); web.dev "Performance budgets 101"; web.dev "Your first performance budget"; V8 blog "The cost of JavaScript in 2019"; web.dev "Render-blocking CSS"

### FE-BUNDLE-002

> Routes that are not part of the initial render MUST be code-split and loaded on demand. The initial JavaScript payload MUST NOT include code for routes the user has not yet navigated to. Framework-specific dynamic-import mechanisms (`React.lazy`, dynamic `import()`, route-level chunking) are the canonical implementation.

**Rationale:** A monolithic bundle forces every user to download, parse, and execute every route's code on first load, including code for features they may never use. Code splitting shortens startup by sending only the code the initial route needs, and in a client-side framework splitting at the route level is the simpler approach; it shrinks the initial payload by all the code that lives only in other routes. The fetch it moves to navigation time can be started early by prefetching likely next routes' chunks — through a framework's route prefetching where it offers one (Next.js `<Link>`, for example, prefetches a linked route when the link enters the viewport), or `<link rel="prefetch">` where supported (it is not Baseline: Safari ships it only behind a preference). The Speculation Rules API does not help here, because it prefetches navigations, not subresources such as script chunks.

**Verification:** Any spec or plan that introduces a multi-route SPA or that adds a new route MUST commit to per-route code-splitting. `/{project}:analyze` flags SPA build specs that produce a single monolithic bundle and flags new-route specs that statically import the route module from the entry point.

**Source:** web.dev "Reduce JavaScript payloads with code splitting", React `lazy` reference, Vue Router "Lazy Loading Routes", Angular "Route loading strategies", Next.js `<Link>` reference, MDN `rel=prefetch` and Speculation Rules API documentation

### FE-BUNDLE-003

> The size budgets of `FE-BUNDLE-001` SHOULD be checked automatically on every change to frontend code, dependencies, or build configuration, failing the change when a bundle exceeds its budget.

**Rationale:** Bundles grow one dependency at a time, and a regression found at release is hard to attribute to the change that caused it; an automated check surfaces the cost on the change that adds it, while it is still under review. Where the check runs — CI, a pre-commit hook, the bundler's own budget setting — is adopter-owned and outside what a spec or plan can bind, so this rule is advisory.

**Verification:** Any spec or plan covering frontend build or CI configuration SHOULD include an automated budget check that fails on exceedance. `/{project}:analyze` emits a warning when such a spec omits it.

**Source:** web.dev "Incorporate performance budgets into your build process", Addy Osmani "Start Performance Budgeting"

## FE-IMAGE — Image Delivery

### FE-IMAGE-001

> Raster images served as content imagery (hero images, product photos, article media) MUST be delivered in a modern format — AVIF or WebP — selected by `<picture>` `<source type>` elements or by server/CDN negotiation on the request's `Accept` header. A JPEG/PNG fallback MUST be provided when the project's supported-browser policy includes a browser that lacks the delivered format. A bare `<img src="*.jpg">`/`<img src="*.png">` with no modern-format source or negotiation MUST NOT be used for non-decorative content imagery.

**Rationale:** WebP files are usually 25–35% smaller than equivalent JPEG and PNG files, and AVIF offers further compression gains over both JPEG and WebP, with savings above 50% against JPEG reported. Images account for more bytes than any other resource type on the median home page (911 KB of 2.56 MB on mobile in the 2025 Web Almanac), so the format choice moves a large share of the payload. Both formats are Baseline widely available (WebP since March 2023, AVIF since July 2026), so a fallback is owed only to older browsers a project's support policy still names. `<picture>` source selection and `Accept` negotiation both add no runtime script.

**Verification:** Any spec or plan that introduces image-bearing UI MUST commit to modern-format delivery and state whether its supported-browser policy requires a JPEG/PNG fallback. `/{project}:analyze` flags affected-files snippets with `<img src="*.jpg">` or `<img src="*.png">` for content imagery without an accompanying `<picture>` element, `Accept`-negotiating image pipeline, or other modern-format pipeline.

**Source:** web.dev "Use WebP images", web.dev "Using AVIF to compress images on your site", HTTP Archive Web Almanac 2025 "Page Weight", MDN `<picture>` documentation, Baseline (web-features) status for AVIF and WebP

### FE-IMAGE-002

> Images MUST declare their intrinsic dimensions via `width` and `height` attributes (or via CSS `aspect-ratio`) so the browser can reserve layout space before the image loads. Images that load without declared dimensions cause layout shift (CLS regression).

**Rationale:** A late-arriving image with undeclared dimensions pushes content down after the user has started reading or interacting — measurable as CLS, perceived as jarring. Declared dimensions let the browser reserve the space and load the image without disturbing surrounding content.

**Verification:** Any spec or plan that introduces image-bearing UI MUST commit to declared intrinsic dimensions per image. `/{project}:analyze` flags affected-files snippets with `<img>` elements lacking `width` and `height` attributes (or a corresponding `aspect-ratio` rule).

**Source:** web.dev "Optimize Cumulative Layout Shift", MDN `<img>` attribute documentation

### FE-IMAGE-003

> Content images SHOULD be delivered at a resolution matched to their rendered size: an image whose rendered width varies with the viewport or device pixel ratio SHOULD offer multiple widths through `srcset` with an accurate `sizes` attribute (or come from an image CDN that resizes per request), rather than one large file the browser scales down.

**Rationale:** The browser downloads the whole file before scaling it, so a 2400px-wide hero rendered 400px wide on a phone spends most of its bytes on pixels the user never sees; web.dev puts the cost of serving desktop-sized images to mobile devices at 2–4x more data than needed. `srcset` width descriptors and `sizes` tell the browser each candidate's width and the image's rendered width, so it can choose a candidate that fits the layout and the device's pixel density (the final choice is at its discretion). A `sizes` that overstates the rendered width makes it choose too large, and an omitted `sizes` defaults to `100vw`, which overstates the width of any image narrower than the viewport.

**Verification:** Any spec or plan that introduces content imagery rendered at viewport-dependent sizes SHOULD commit to responsive delivery — named candidate widths with `srcset`/`sizes`, or an image CDN's per-request resizing. `/{project}:analyze` emits a warning when an image-bearing spec names a single source file for imagery that renders at different widths across breakpoints.

**Source:** web.dev "Serve responsive images", MDN `<img>` documentation (`srcset`, `sizes`), MDN "Using responsive images in HTML"

## FE-LOAD — Resource Loading

### FE-LOAD-001

> Non-critical resources (below-the-fold images, off-route JavaScript, third-party widgets, embedded media) MUST be lazy-loaded. Off-route JavaScript MUST be code-split per `FE-BUNDLE-002`.

**Rationale:** Eagerly loading every resource on initial page load competes for bandwidth and CPU with the critical-path content the user is waiting for. Lazy-loading defers non-critical work until it is actually needed, freeing the main thread and the network for what matters first. Images in a route's initial viewport are critical, not deferrable; lazy-loading them is the LCP regression `FE-LOAD-005` forbids.

**Verification:** Any spec or plan that introduces below-the-fold UI, third-party embeds, or non-critical media MUST commit to lazy-loading. `/{project}:analyze` flags affected-files snippets that statically include heavy third-party widgets in the initial render, and flags long-form-page specs that omit lazy-loading commitments for below-fold imagery.

**Source:** web.dev "Browser-level image lazy loading for the web", MDN `loading` attribute documentation

### FE-LOAD-002

> The critical rendering path MUST be defined and documented per route — the minimum set of CSS and JavaScript required to render above-the-fold content. Render-blocking JavaScript MUST be minimized: every external classic script (`<script src>`) MUST use `defer` or `async`, or be a module script (`type="module"`, which defers by default), unless parser-blocking execution is required for correctness, with the reason documented.

**Rationale:** A parser-blocking script halts HTML parsing for its full download and execution time, so nothing after it in the document can render until it finishes. Deferred and async scripts download without blocking the parser, and a documented critical path is what tells a later change whether it adds to the render-blocking set. The difference is measured in seconds on slow networks. `defer` and `async` have no effect on inline classic scripts, which execute where they appear, so an inline classic script in the `<head>` belongs in the documented critical path.

**Verification:** Any spec or plan that introduces a new route or that modifies the document `<head>` MUST commit to a documented critical rendering path and to `defer`/`async` (or module scripts) for every external classic script not documented as requiring parser-blocking execution. `/{project}:analyze` flags affected-files snippets with external classic `<script src>` tags lacking `defer`, `async`, or `type="module"` (other than documented parser-blocking scripts).

**Source:** web.dev "Understand the critical path", MDN `<script>` documentation (`async`, `defer`, `type="module"`)

### FE-LOAD-003

> Lazy-loaded images and iframes SHOULD use the native `loading="lazy"` attribute.

**Rationale:** The native attribute defers the fetch until the element nears the viewport without shipping or running any JavaScript, and the browser chooses the load distance itself (Chromium varies it with the effective connection type). A script-based loader costs main-thread time and leaves the element unloaded when its script fails. Every current engine supports the attribute on both images and iframes (iframes since Firefox 121 and Safari 16.4).

**Verification:** Any spec or plan that lazy-loads images or iframes SHOULD commit to the native `loading="lazy"` attribute. `/{project}:analyze` emits a warning when affected-files snippets lazy-load images or iframes through a script-based loader without a stated reason the native attribute does not fit.

**Source:** MDN `loading` attribute documentation, web.dev "Browser-level image lazy loading for the web"

### FE-LOAD-004

> Critical CSS SHOULD be inlined in the document `<head>` in a `<style>` element that carries the page's CSP nonce or whose content matches a hash the policy lists, and non-critical CSS SHOULD be loaded asynchronously by a mechanism that needs no inline event handler (e.g., a `<link rel="preload" as="style">` or `media="print"` stylesheet that an external or nonce-bearing script promotes to an applied stylesheet).

**Rationale:** An external stylesheet blocks rendering for its full round trip. Inlining the critical CSS lets the browser paint above-the-fold content on receipt of the HTML, and loading the rest asynchronously keeps it off the critical path. The widely copied `onload="this.rel='stylesheet'"` pattern is an inline event handler, which the nonce- or hash-based policy `FE-CSP-002` requires blocks (under CSP Level 3 a nonce never applies to an attribute, and a hash reaches an event handler only when the policy adds `'unsafe-hashes'`), so under that policy the stylesheet never applies — web.dev's "Defer non-critical CSS", which shows the pattern, itself notes that a Content Security Policy might not allow inline `onload` JavaScript. An inline `<style>` with neither the nonce nor a listed hash is blocked the same way. A `media="print"` stylesheet loads without delaying rendering because its media does not match the screen, and switching its `media` to `all` once it loads applies it.

**Verification:** Any spec or plan that introduces a new route or that modifies the document `<head>` SHOULD commit to an inline critical-CSS strategy compatible with the project's CSP. `/{project}:analyze` emits a warning when HTML specs propose synchronous external stylesheets without an inline critical-CSS strategy, or an async-CSS mechanism that relies on an inline `onload` handler or an inline `<style>` without a nonce or hash.

**Source:** web.dev "Understand the critical path", web.dev "Extract critical CSS", web.dev "Defer non-critical CSS", Filament Group "The Simplest Way to Load CSS Asynchronously", W3C Content Security Policy Level 3; `security-frontend.md` §FE-CSP

### FE-LOAD-005

> Images in a route's initial viewport — the LCP element's image above all — MUST NOT be lazy-loaded, whether by `loading="lazy"` or by a script-based loader.

**Rationale:** Lazy-loading holds a fetch until layout shows the element near the viewport, so on an image already in it the deferral is pure resource-load delay added to LCP; web.dev's LCP guidance is never to lazy-load the LCP image. `FE-LOAD-001` and `FE-LOAD-003` push toward lazy-loading, and applying them blanket-wide — a shared image component that sets `loading="lazy"` on every image — is how this regression usually ships.

**Verification:** Any spec or plan that introduces or modifies above-the-fold UI MUST identify the route's likely LCP element and commit to loading its image eagerly. `/{project}:analyze` flags specs that apply lazy-loading to every image without exempting the initial viewport, and flags affected-files snippets that set `loading="lazy"` on a hero or other above-the-fold image.

**Source:** web.dev "Optimize Largest Contentful Paint", web.dev "Browser-level image lazy loading for the web"

### FE-LOAD-006

> The resource of each route's LCP element SHOULD be discoverable in the initial HTML response and SHOULD carry `fetchpriority="high"`, on no more than one or two images per route. A critical resource referenced only from CSS or JavaScript — an LCP background image, a web font used above the fold — SHOULD be preloaded with `<link rel="preload">` whose `as` names the resource's type, whose `type` names its MIME type, and whose `crossorigin` matches the eventual request's CORS mode; font preloads SHOULD carry `crossorigin` even for same-origin fonts.

**Rationale:** web.dev breaks LCP into time to first byte, resource load delay, resource load duration, and element render delay. A resource the preload scanner cannot see in the HTML waits for a stylesheet or script to run before its request starts, which is pure load delay. `fetchpriority="high"` lifts the LCP image above other images competing for bandwidth; it is a hint, not a directive, and has been Baseline newly available since October 2024. `as` lets the browser reuse the preloaded response for the real request and apply the right Content Security Policy to it, and `type` lets a browser skip a format it does not support. Fonts are fetched in anonymous-mode CORS even from the same origin, so a font preload without `crossorigin` does not match the real request and the font is fetched twice. Prioritizing or preloading many resources defeats itself: web.dev's LCP guidance warns that a high priority on more than one or two images makes priority setting unhelpful, and its preload guidance warns that too many preloads contend for bandwidth, most visibly on slower networks.

**Verification:** Any spec or plan that introduces or modifies a route's above-the-fold content SHOULD state how its LCP resource is discovered and prioritized. `/{project}:analyze` emits a warning when a spec's LCP image is injected by client-side script or referenced only from CSS with no preload, and when affected-files snippets preload a font without `crossorigin`.

**Source:** web.dev "Optimize Largest Contentful Paint", web.dev "Optimize resource loading with the Fetch Priority API", web.dev "Preload critical assets to improve loading speed", MDN `rel=preload` documentation, Baseline (web-features) status for Fetch priority

### FE-LOAD-007

> Build-emitted static assets (JavaScript, CSS, fonts, images) SHOULD be served from content-hashed URLs with a long-lived cache lifetime (`Cache-Control: max-age=31536000, immutable`), and the HTML documents that reference them SHOULD be served with `Cache-Control: no-cache`, so each navigation revalidates them, or with a short `max-age`, which bounds how long a returning visitor keeps the old asset URLs after a deploy.

**Rationale:** A repeat visit that revalidates or re-downloads unchanged assets spends round trips and bytes on nothing. A content hash in the filename changes the URL whenever the content changes, so a browser holding the old file in a long-lived cache simply requests the new URL, and the long lifetime is safe. Long-caching an unhashed URL serves stale code after a deploy, and long-caching the HTML pins users to old asset URLs. `immutable` additionally tells a cache not to revalidate a fresh response, even on reload; Safari and desktop Firefox honor it, and Chromium-based browsers ignore it. HTML that displays sensitive data stays `no-store` per `FE-PII-003` (security-frontend.md), and compressing these assets is `BE-PAYLOAD-003`'s concern (performance-backend.md); cache policy for other server responses is `BE-CACHE-004` (performance-backend.md).

**Verification:** Any spec or plan that introduces or modifies a frontend build, CDN, or static-asset serving configuration SHOULD state the cache policy for hashed assets and for HTML documents. `/{project}:analyze` emits a warning when such a spec omits the cache policy, or applies a long-lived `max-age` to unhashed asset URLs or to HTML documents.

**Source:** web.dev "Prevent unnecessary network requests with the HTTP Cache", MDN `Cache-Control` documentation and browser compatibility data, RFC 9111 (HTTP Caching), RFC 8246 (HTTP Immutable Responses)

### FE-LOAD-008

> Content that loads or renders after first paint — iframes, embeds, ads, video, async-loaded components, injected banners — SHOULD reserve its layout space before it arrives (`width`/`height` attributes, CSS `aspect-ratio`, or a `min-height` placeholder sized to the expected content), and SHOULD NOT be inserted above content the user is already viewing except in response to user input.

**Rationale:** Late content without reserved space pushes the page down after the user has started reading or aiming a tap — counted as CLS, and one of the causes web.dev lists beside undimensioned images. Reserved space lets the content arrive without moving anything; collapsing a reserved slot when nothing arrives, such as an unfilled ad slot, shifts the page just the same. Layout shifts within 500ms of a user input are excluded from CLS, which is why content inserted in response to input is exempt; content injected on a timer or a network response is not. Image dimensions are `FE-IMAGE-002`'s concern.

**Verification:** Any spec or plan that introduces third-party embeds, ads, iframes, video, or client-rendered content that appears after the initial render SHOULD state how its layout space is reserved. `/{project}:analyze` emits a warning when such a spec describes late-arriving content with no reserved dimensions or placeholder, or inserts content above the current viewport on a timer or network response.

**Source:** web.dev "Optimize Cumulative Layout Shift", web.dev "Cumulative Layout Shift (CLS)"

### FE-LOAD-009

> Pages SHOULD NOT register `unload` event listeners. Work that must run when the user leaves or hides a page — saving state, sending analytics or RUM beacons — SHOULD run on `visibilitychange` (when `document.visibilityState` becomes `hidden`) or on `pagehide`.

**Rationale:** The back/forward cache (bfcache) makes back and forward navigation instant by keeping a page the user left in memory and restoring it without going to the network. An `unload` listener works against it: Firefox does not place a page that has one in the bfcache, Safari may cache such a page but does not run the handler when the user navigates away, and Chrome has completed deprecating the event, so it no longer fires by default (it may still fire in Chrome on iOS, which uses WebKit). Where it does fire, it is unreliable, especially on mobile — a user who switches apps and later closes the browser from the app manager never triggers it — so a beacon sent from it is lost. MDN names `visibilitychange` the last event reliably fired and the best place to save state or send analytics, which makes it the place to flush the RUM measurements `FE-VITALS-002` requires; `pagehide` fires when the user navigates away and does not block the bfcache. A page served `Cache-Control: no-store` may also be ineligible for the bfcache; `FE-LOAD-007`'s `no-cache` for HTML avoids that, and a page displaying sensitive data stays `no-store` per `FE-PII-003` (security-frontend.md).

**Verification:** Any spec or plan that sends analytics or RUM data, saves state as the user leaves a page, or otherwise handles page-exit events SHOULD name `visibilitychange` or `pagehide` as the trigger. `/{project}:analyze` emits a warning when such a spec relies on `unload`, or when affected-files snippets register an `unload` listener (`addEventListener('unload', …)`, `window.onunload`, or an `onunload` attribute).

**Source:** web.dev "Back/forward cache", Chrome for Developers "Deprecating the unload event", MDN "Window: unload event"

## FE-FONT — Web Font Discipline

### FE-FONT-001

> Web fonts MUST be loaded with a documented fallback strategy. `@font-face` declarations MUST use `font-display: swap`, `fallback`, or `optional` so text remains visible during font load — `swap` where the web font must eventually render (branding), `fallback` or `optional` where speed and layout stability come first. `font-display: auto` (the default, which Chromium and Firefox implement as a block period of up to 3 seconds) and `block` produce Flash of Invisible Text (FOIT) and MUST NOT be relied on.

**Rationale:** A blank page while a font downloads is worse than a momentarily mis-styled page — users read text content; they tolerate a font swap. `swap` shows fallback text immediately and swaps to the web font when it arrives; `fallback` and `optional` cap invisible text at about 100ms, and `optional` drops a late font entirely, so it never re-lays out the page. `auto` and `block` keep text invisible until the font lands or the block period ends, which on slow connections means seconds — by default Chromium-based browsers and Firefox block text for up to 3 seconds, and web.dev reports that Safari blocks it indefinitely. A swap re-lays out text and can shift layout, which counts toward CLS and so against the `FE-VITALS-001` budget. `optional` avoids the re-layout; matching the fallback font's metrics reduces it — `size-adjust` works in every current engine (Safari since 17), while `ascent-override`, `descent-override`, and `line-gap-override` are not yet in a stable Safari release.

**Verification:** Any spec or plan that introduces web fonts MUST commit to `font-display: swap`, `fallback`, or `optional` on every `@font-face` declaration. `/{project}:analyze` flags CSS in affected files containing `@font-face` rules without a `font-display` declaration or with `font-display: auto`/`block`.

**Source:** web.dev "Best practices for fonts", web.dev "Optimize Cumulative Layout Shift", MDN `font-display` documentation, MDN browser compatibility data for the `@font-face` metric descriptors

### FE-FONT-002

> Self-hosted web fonts MUST be served as WOFF2 (supported by every current browser, smallest size); legacy formats (TTF, OTF, EOT, WOFF) MUST NOT be served as the primary source.

**Rationale:** WOFF2 uses Brotli and compresses about 30% better than WOFF, and every current browser engine supports it (Chrome 36, Firefox 39, Safari 10).

**Verification:** Any spec or plan that introduces self-hosted fonts MUST commit to WOFF2 delivery. `/{project}:analyze` flags affected-files snippets with `@font-face` rules that load `.ttf`/`.otf`/`.eot`/`.woff` (not `.woff2`) URLs as the primary source.

**Source:** web.dev "Best practices for fonts", web.dev "Reduce web font size", MDN browser compatibility data for `@font-face` WOFF 2

### FE-FONT-003

> The font payload SHOULD be subset to the Unicode ranges the application actually uses.

**Rationale:** A font file carries every glyph it supports — Latin fonts typically hold 100 to 1,000 glyphs, and CJK fonts can hold more than 10,000 — and the browser downloads all of them. Subsetting to the scripts the application renders (e.g., dropping CJK glyphs from a Latin-only application) removes bytes WOFF2 compression (`FE-FONT-002`) cannot, and `unicode-range` splits let the browser fetch only the subsets a page actually uses.

**Verification:** Any spec or plan that introduces self-hosted fonts SHOULD commit to a subsetting strategy (a build-time subset, or `unicode-range` splits). `/{project}:analyze` emits a warning when a self-hosted font spec names no subsetting strategy.

**Source:** web.dev "Best practices for fonts", web.dev "Reduce web font size", MDN `unicode-range` documentation

## FE-MAIN — Main-Thread Work

### FE-MAIN-001

> CPU-intensive client work — parsing or transforming large payloads, sorting or filtering large collections, image or file processing — SHOULD NOT run as a single long task (over 50ms) on the main thread. It SHOULD run in a Web Worker when it needs no DOM access, or be broken into chunks that yield to the main thread between them (`scheduler.yield()` with a fallback, or `setTimeout`).

**Rationale:** The main thread both runs JavaScript and responds to input; while a task runs, clicks and keystrokes wait, and the wait is measured as poor INP. Any task over 50ms is a long task. A Web Worker runs script on a background thread and cannot touch the DOM, so it takes work that needs no DOM access off the main thread entirely; yielding lets the browser handle pending input and paint between chunks. `scheduler.yield()` needs the fallback because Safari does not yet support it (Chrome and Edge 129, Firefox 142). Lab tools surface the risk as Total Blocking Time, before RUM reports it as INP (`FE-VITALS-002`).

**Verification:** Any spec or plan that introduces client-side processing of large or unbounded data, or other CPU-heavy work in the browser, SHOULD state where that work runs — a Web Worker, or chunked with yields to the main thread. `/{project}:analyze` emits a warning when such a spec describes synchronous processing of a user-controlled or unbounded dataset in an event handler or on page load with no off-main-thread or chunking commitment.

**Source:** web.dev "Optimize long tasks", web.dev "Interaction to Next Paint (INP)", web.dev "Total Blocking Time (TBT)", W3C Long Tasks API, MDN Web Workers API, MDN browser compatibility data for `Scheduler.yield()`

### FE-MAIN-002

> A client-rendered collection whose length is unbounded or user-controlled SHOULD bound the number of items it keeps in the DOM — by pagination or windowing (virtualization) — rather than rendering every item at once, including when infinite scroll accumulates items.

**Rationale:** Every rendered node costs memory and style and layout time on each update; a list that renders thousands of rows makes every interaction touching it slow (INP), and under infinite scroll it grows without limit. Rendering only what is near the viewport keeps that cost proportional to the screen rather than the dataset. CSS `content-visibility: auto` skips rendering work for off-screen content but keeps every node in the DOM, so it complements a bound rather than replacing one. Bounding what the server returns is `BE-PAGE`'s concern (api-backend.md); this rule bounds what the client keeps in the DOM.

**Verification:** Any spec or plan that renders a list, table, feed, or grid whose length can grow with user data SHOULD state how many items are in the DOM at once and the mechanism that bounds it. `/{project}:analyze` emits a warning when such a spec renders the full collection, or accumulates items under infinite scroll, with no pagination or windowing commitment.

**Source:** web.dev "Virtualize large lists with react-window", web.dev "content-visibility: the new CSS property that boosts your rendering performance", Lighthouse "Optimize DOM size" insight (formerly the "Avoid an excessive DOM size" audit)
