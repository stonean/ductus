# Performance Rules — Frontend

Performance rules for browser-rendered UI, for any project with a web frontend. They verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts, including code snippets a plan quotes for its affected files. Rule IDs follow the format `FE-{CATEGORY}-{NNN}`. Categories: `VITALS` (Core Web Vitals budgets and measurement), `BUNDLE` (JavaScript/CSS bundle size), `IMAGE` (image delivery), `LOAD` (resource loading, prioritization, HTTP and back/forward caching, and layout reservation), `FONT` (web font discipline), `MAIN` (main-thread work). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## FE-VITALS — Core Web Vitals Budgets and Measurement

### FE-VITALS-001

> The project MUST publish documented performance budgets for the three Core Web Vitals — Largest Contentful Paint (LCP), Interaction to Next Paint (INP), and Cumulative Layout Shift (CLS) — measured at the 75th percentile of real-user page loads, segmented by mobile and desktop. Default budgets at Google's "Good" threshold: LCP ≤ 2.5s, INP ≤ 200ms, CLS ≤ 0.1. Stricter project-specific budgets are encouraged; looser budgets MUST be justified in `specs/system.md`.

**Verification:** Any spec or plan that introduces or modifies user-facing UI MUST commit to maintaining the documented Web Vitals budgets. `/{project}:analyze` flags a frontend-UI spec that omits that commitment, a spec proposing looser thresholds without a `specs/system.md` justification reference, and budgets that name First Input Delay (FID) in place of INP.

**Source:** web.dev "Web Vitals", web.dev "Defining the Core Web Vitals metrics thresholds", web.dev "Interaction to Next Paint becomes a Core Web Vital on March 12"

### FE-VITALS-002

> Web Vitals MUST be measured continuously on real users (RUM), not only via synthetic CI runs. The application MUST emit `web-vitals` (or equivalent) measurements to a backend the team monitors. Synthetic Lighthouse runs in CI are complementary but MUST NOT be the sole measurement source.

**Verification:** Any spec or plan covering frontend instrumentation, observability, or CI performance MUST commit to each obligation in the Statement. `/{project}:analyze` flags performance specs that describe only synthetic measurement without a RUM commitment.

**Source:** web.dev "Why lab and field data can be different (and what to do about it)", web.dev "Web Vitals", `web-vitals` JavaScript library documentation

## FE-BUNDLE — Bundle Size

### FE-BUNDLE-001

> The project MUST publish JavaScript and CSS size budgets for each route or entry point in `specs/system.md`, measured as transferred (compressed) bytes. A bundle that exceeds its budget MUST NOT ship until the budget is raised in `specs/system.md`.

**Verification:** Any spec or plan that introduces frontend code, dependencies, or build configuration MUST commit to the Statement's budgets and to how exceedance is caught before the change ships. `/{project}:analyze` flags frontend projects with no budgets published in `specs/system.md`, and flags dependency-adding specs that omit a bundle-impact estimate against the budget. **Not flagged:** a budget's figure, which the rule leaves to the project; and a means of catching exceedance that is not automated — a missing automated budget check is `FE-BUNDLE-003`'s.

**Source:** Addy Osmani, "Start Performance Budgeting" (addyosmani.com, 2018); web.dev "Performance budgets 101"; web.dev "Your first performance budget"

### FE-BUNDLE-002

> Routes that are not part of the initial render MUST be code-split and loaded on demand. The initial JavaScript payload MUST NOT include code for routes the user has not yet navigated to.

**Verification:** Any spec or plan that introduces a multi-route SPA or that adds a new route MUST commit to per-route code-splitting. `/{project}:analyze` flags SPA build specs that produce a single monolithic bundle and flags new-route specs that statically import the route module from the entry point. **Not flagged:** a route's chunk prefetched after the initial render, through a framework's route prefetching or `<link rel="prefetch">`.

**Source:** web.dev "Reduce JavaScript payloads with code splitting", React `lazy` reference, Vue Router "Lazy Loading Routes", Angular "Route loading strategies", Next.js `<Link>` reference, MDN `rel=prefetch` documentation

### FE-BUNDLE-003

> The size budgets of `FE-BUNDLE-001` SHOULD be checked automatically on every change to frontend code, dependencies, or build configuration, failing the change when a bundle exceeds its budget.

**Verification:** Any spec or plan covering frontend build or CI configuration SHOULD include an automated budget check that fails on exceedance. `/{project}:analyze` emits a warning when such a spec omits it. **Not flagged:** where the check runs, such as CI, a pre-commit hook, or the bundler's own budget setting.

**Source:** web.dev "Incorporate performance budgets into your build process", Addy Osmani "Start Performance Budgeting"

## FE-IMAGE — Image Delivery

### FE-IMAGE-001

> Raster images served as content imagery (hero images, product photos, article media) MUST be delivered in a modern format — AVIF or WebP — selected by `<picture>` `<source type>` elements or by server/CDN negotiation on the request's `Accept` header. A JPEG/PNG fallback MUST be provided when the project's supported-browser policy includes a browser that lacks the delivered format. A bare `<img src="*.jpg">`/`<img src="*.png">` with no modern-format source or negotiation MUST NOT be used for non-decorative content imagery.

**Verification:** Any spec or plan that introduces image-bearing UI MUST commit to modern-format delivery and state whether its supported-browser policy requires a JPEG/PNG fallback. `/{project}:analyze` flags affected-files snippets with `<img src="*.jpg">` or `<img src="*.png">` for content imagery without an accompanying `<picture>` element, `Accept`-negotiating image pipeline, or other modern-format pipeline.

**Source:** web.dev "Use WebP images", web.dev "Using AVIF to compress images on your site", MDN `<picture>` documentation, Baseline (web-features) status for AVIF and WebP

### FE-IMAGE-002

> Images MUST declare their intrinsic dimensions via `width` and `height` attributes (or via CSS `aspect-ratio`).

**Verification:** Any spec or plan that introduces image-bearing UI MUST commit to declared intrinsic dimensions per image. `/{project}:analyze` flags affected-files snippets with `<img>` elements lacking `width` and `height` attributes (or a corresponding `aspect-ratio` rule).

**Source:** web.dev "Optimize Cumulative Layout Shift", MDN `<img>` attribute documentation

### FE-IMAGE-003

> Content images SHOULD be delivered at a resolution matched to their rendered size: an image whose rendered width varies with the viewport or device pixel ratio SHOULD offer multiple widths through `srcset` with an accurate `sizes` attribute (or come from an image CDN that resizes per request), rather than one large file the browser scales down.

**Verification:** Any spec or plan that introduces content imagery rendered at viewport-dependent sizes SHOULD commit to responsive delivery — named candidate widths with `srcset`/`sizes`, or an image CDN's per-request resizing. `/{project}:analyze` emits a warning when an image-bearing spec names a single source file for imagery that renders at different widths across breakpoints.

**Source:** web.dev "Serve responsive images", MDN `<img>` documentation (`srcset`, `sizes`), MDN "Using responsive images in HTML"

## FE-LOAD — Resource Loading

### FE-LOAD-001

> Non-critical resources (below-the-fold images, off-route JavaScript, third-party widgets, embedded media) MUST be lazy-loaded. Off-route JavaScript MUST be code-split per `FE-BUNDLE-002`.

**Verification:** Any spec or plan that introduces below-the-fold UI, third-party embeds, or non-critical media MUST commit to lazy-loading. `/{project}:analyze` flags affected-files snippets that statically include heavy third-party widgets in the initial render, and flags long-form-page specs that omit lazy-loading commitments for below-fold imagery. **Not flagged:** eager loading of an image in a route's initial viewport, which is critical; lazy-loading one is `FE-LOAD-005`'s.

**Source:** web.dev "Browser-level image lazy loading for the web", MDN `loading` attribute documentation

### FE-LOAD-002

> The critical rendering path MUST be defined and documented per route — the minimum set of CSS and JavaScript required to render above-the-fold content. Render-blocking JavaScript MUST be minimized: every external classic script (`<script src>`) MUST use `defer` or `async`, or be a module script (`type="module"`), unless parser-blocking execution is required for correctness, with the reason documented.

**Verification:** Any spec or plan that introduces a new route or that modifies the document `<head>` MUST commit to a documented critical rendering path and to `defer`/`async` (or module scripts) for every external classic script not documented as requiring parser-blocking execution. `/{project}:analyze` flags affected-files snippets with external classic `<script src>` tags lacking `defer`, `async`, or `type="module"`, other than documented parser-blocking scripts. An inline classic script in the `<head>` belongs in the documented critical path.

**Source:** web.dev "Understand the critical path", MDN `<script>` documentation (`async`, `defer`, `type="module"`)

### FE-LOAD-003

> Lazy-loaded images and iframes SHOULD use the native `loading="lazy"` attribute.

**Verification:** Any spec or plan that lazy-loads images or iframes SHOULD commit to the native `loading="lazy"` attribute. `/{project}:analyze` emits a warning when affected-files snippets lazy-load images or iframes through a script-based loader without a stated reason the native attribute does not fit.

**Source:** MDN `loading` attribute documentation, web.dev "Browser-level image lazy loading for the web"

### FE-LOAD-004

> Critical CSS SHOULD be inlined in the document `<head>` in a `<style>` element that carries the page's CSP nonce or whose content matches a hash the policy lists, and non-critical CSS SHOULD be loaded asynchronously by a mechanism that needs no inline event handler (e.g., a `<link rel="preload" as="style">` or `media="print"` stylesheet that an external or nonce-bearing script promotes to an applied stylesheet).

**Verification:** Any spec or plan that introduces a new route or that modifies the document `<head>` SHOULD commit to an inline critical-CSS strategy compatible with the project's CSP. `/{project}:analyze` emits a warning when HTML specs propose synchronous external stylesheets without an inline critical-CSS strategy, or an async-CSS mechanism that relies on an inline `onload` handler or an inline `<style>` without a nonce or hash. This rule works within the nonce- or hash-based policy `FE-CSP-002` (`security-frontend.md`) requires: a policy relaxed to admit the inline handler or the inline `<style>` this rule warns on is never a fix here, and `'unsafe-inline'` in its `script-src` or `style-src` is `FE-CSP-002`'s.

**Source:** web.dev "Understand the critical path", web.dev "Extract critical CSS", web.dev "Defer non-critical CSS", Filament Group "The Simplest Way to Load CSS Asynchronously", W3C Content Security Policy Level 3; `security-frontend.md` §FE-CSP

### FE-LOAD-005

> Images in a route's initial viewport — the LCP element's image above all — MUST NOT be lazy-loaded, whether by `loading="lazy"` or by a script-based loader.

**Verification:** Any spec or plan that introduces or modifies above-the-fold UI MUST identify the route's likely LCP element and commit to loading its image eagerly. `/{project}:analyze` flags specs that apply lazy-loading to every image without exempting the initial viewport, and flags affected-files snippets that set `loading="lazy"` on a hero or other above-the-fold image.

**Source:** web.dev "Optimize Largest Contentful Paint", web.dev "Browser-level image lazy loading for the web"

### FE-LOAD-006

> The resource of each route's LCP element SHOULD be discoverable in the initial HTML response and SHOULD carry `fetchpriority="high"`, on no more than one or two images per route. A critical resource referenced only from CSS or JavaScript — an LCP background image, a web font used above the fold — SHOULD be preloaded with `<link rel="preload">` whose `as` names the resource's type, whose `type` names its MIME type, and whose `crossorigin` matches the eventual request's CORS mode; font preloads SHOULD carry `crossorigin` even for same-origin fonts.

**Verification:** Any spec or plan that introduces or modifies a route's above-the-fold content SHOULD state how its LCP resource is discovered and prioritized. `/{project}:analyze` emits a warning when a spec's LCP image is injected by client-side script or referenced only from CSS with no preload, and when affected-files snippets preload a font without `crossorigin`.

**Source:** web.dev "Optimize Largest Contentful Paint", web.dev "Optimize resource loading with the Fetch Priority API", web.dev "Preload critical assets to improve loading speed", MDN `rel=preload` documentation

### FE-LOAD-007

> Build-emitted static assets (JavaScript, CSS, fonts, images) SHOULD be served from content-hashed URLs with a long-lived cache lifetime (`Cache-Control: max-age=31536000, immutable`), and the HTML documents that reference them SHOULD be served with `Cache-Control: no-cache` or with a short `max-age`, which bounds how long a returning visitor keeps the old asset URLs after a deploy.

**Verification:** Any spec or plan that introduces or modifies a frontend build, CDN, or static-asset serving configuration SHOULD state the cache policy for hashed assets and for HTML documents. `/{project}:analyze` emits a warning when such a spec omits the cache policy, or applies a long-lived `max-age` to unhashed asset URLs or to HTML documents. **Not flagged:** HTML that displays sensitive data served `no-store`, as `FE-PII-003` (`security-frontend.md`) requires. An uncompressed asset is `BE-PAYLOAD-003`'s, and the cache policy of any other server response `BE-CACHE-004`'s (`performance-backend.md`).

**Source:** web.dev "Prevent unnecessary network requests with the HTTP Cache", MDN `Cache-Control` documentation, RFC 9111 (HTTP Caching), RFC 8246 (HTTP Immutable Responses)

### FE-LOAD-008

> Content that loads or renders after first paint — iframes, embeds, ads, video, async-loaded components, injected banners — SHOULD reserve its layout space before it arrives (`width`/`height` attributes, CSS `aspect-ratio`, or a `min-height` placeholder sized to the expected content) and keep it reserved when nothing arrives, and SHOULD NOT be inserted above content the user is already viewing except in response to user input.

**Verification:** Any spec or plan that introduces third-party embeds, ads, iframes, video, or client-rendered content that appears after the initial render SHOULD state how its layout space is reserved. `/{project}:analyze` emits a warning when such a spec describes late-arriving content with no reserved dimensions or placeholder, or inserts content above the current viewport on a timer or network response. A reserved slot that collapses when nothing arrives, such as an unfilled ad slot, counts as unreserved. An `<img>` without declared dimensions is `FE-IMAGE-002`'s.

**Source:** web.dev "Optimize Cumulative Layout Shift", web.dev "Cumulative Layout Shift (CLS)"

### FE-LOAD-009

> Pages SHOULD NOT register `unload` event listeners. Work that must run when the user leaves or hides a page — saving state, sending analytics or RUM beacons — SHOULD run on `visibilitychange` (when `document.visibilityState` becomes `hidden`) or on `pagehide`.

**Verification:** Any spec or plan that sends analytics or RUM data, saves state as the user leaves a page, or otherwise handles page-exit events SHOULD name `visibilitychange` or `pagehide` as the trigger. `/{project}:analyze` emits a warning when such a spec relies on `unload`, or when affected-files snippets register an `unload` listener (`addEventListener('unload', …)`, `window.onunload`, or an `onunload` attribute). A page served `Cache-Control: no-store` may also be ineligible for the bfcache (back/forward cache), but this rule does not flag that header: HTML cache policy is `FE-LOAD-007`'s, and a page displaying sensitive data stays `no-store` per `FE-PII-003` (`security-frontend.md`).

**Source:** web.dev "Back/forward cache", Chrome for Developers "Deprecating the unload event", MDN "Window: unload event"

## FE-FONT — Web Font Discipline

### FE-FONT-001

> Web fonts MUST be loaded with a documented fallback strategy. `@font-face` declarations MUST use `font-display: swap`, `fallback`, or `optional` — `swap` where the web font must eventually render (branding), `fallback` or `optional` where speed and layout stability come first. `font-display: auto` (the default) and `block` MUST NOT be relied on.

**Verification:** Any spec or plan that introduces web fonts MUST commit to `font-display: swap`, `fallback`, or `optional` on every `@font-face` declaration. `/{project}:analyze` flags CSS in affected files containing `@font-face` rules without a `font-display` declaration or with `font-display: auto`/`block`.

**Source:** web.dev "Best practices for fonts", web.dev "Optimize Cumulative Layout Shift", MDN `font-display` documentation

### FE-FONT-002

> Self-hosted web fonts MUST be served as WOFF2; legacy formats (TTF, OTF, EOT, WOFF) MUST NOT be served as the primary source.

**Verification:** Any spec or plan that introduces self-hosted fonts MUST commit to WOFF2 delivery. `/{project}:analyze` flags affected-files snippets with `@font-face` rules that load `.ttf`/`.otf`/`.eot`/`.woff` (not `.woff2`) URLs as the primary source.

**Source:** web.dev "Best practices for fonts", web.dev "Reduce web font size"

### FE-FONT-003

> The font payload SHOULD be subset to the Unicode ranges the application actually uses.

**Verification:** Any spec or plan that introduces self-hosted fonts SHOULD commit to a subsetting strategy (a build-time subset, or `unicode-range` splits). `/{project}:analyze` emits a warning when a self-hosted font spec names no subsetting strategy.

**Source:** web.dev "Best practices for fonts", web.dev "Reduce web font size", MDN `unicode-range` documentation

## FE-MAIN — Main-Thread Work

### FE-MAIN-001

> CPU-intensive client work — parsing or transforming large payloads, sorting or filtering large collections, image or file processing — SHOULD NOT run as a single long task (over 50ms) on the main thread. It SHOULD run in a Web Worker when it needs no DOM access, or be broken into chunks that yield to the main thread between them (`scheduler.yield()` with a fallback, or `setTimeout`).

**Verification:** Any spec or plan that introduces client-side processing of large or unbounded data, or other CPU-heavy work in the browser, SHOULD state where that work runs — a Web Worker, or chunked with yields to the main thread. `/{project}:analyze` emits a warning when such a spec describes synchronous processing of a user-controlled or unbounded dataset in an event handler or on page load with no off-main-thread or chunking commitment.

**Source:** web.dev "Optimize long tasks", W3C Long Tasks API, MDN Web Workers API, MDN browser compatibility data for `Scheduler.yield()`

### FE-MAIN-002

> A client-rendered collection whose length is unbounded or user-controlled SHOULD bound the number of items it keeps in the DOM — by pagination or windowing (virtualization) — rather than rendering every item at once, including when infinite scroll accumulates items.

**Verification:** Any spec or plan that renders a list, table, feed, or grid whose length can grow with user data SHOULD state how many items are in the DOM at once and the mechanism that bounds it. `/{project}:analyze` emits a warning when such a spec renders the full collection, or accumulates items under infinite scroll, with no pagination or windowing commitment. CSS `content-visibility: auto` skips rendering work for off-screen content but keeps every node in the DOM, so it complements a bound rather than replacing one. This rule governs what the client keeps in the DOM: an unpaginated list response from the server is `BE-PAGE`'s (`api-backend.md`).

**Source:** web.dev "Virtualize large lists with react-window", web.dev "content-visibility: the new CSS property that boosts your rendering performance", Lighthouse "Optimize DOM size" insight (formerly the "Avoid an excessive DOM size" audit)
