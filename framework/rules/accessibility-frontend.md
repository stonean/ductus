# Accessibility Rules — Frontend

Enforceable accessibility rules for browser-rendered UI. These rules apply to any project with a web frontend adopting `ductus`. They derive from WCAG 2.2 Level AA (W3C Recommendation, first published 5 October 2023 and republished 12 December 2024; the October 2023 text is ISO/IEC 40500:2025). W3C states that content conforming to WCAG 2.2 also conforms to WCAG 2.0 and 2.1: 2.2 adds criteria and removes only §4.1.1 (Parsing), now obsolete, though a project required by policy to conform to 2.0 or 2.1 may still need to test and report §4.1.1. Regimes that mandate web accessibility differ in the version they name: US Section 508 incorporates WCAG 2.0 Level A and AA; the US Department of Justice's ADA Title II rule for state and local governments adopts WCAG 2.1 Level AA; Ontario's AODA requires WCAG 2.0 Level AA except §1.2.4 and §1.2.5; GOV.UK's guidance on the UK Public Sector Bodies (Websites and Mobile Applications) (No. 2) Accessibility Regulations 2018 says to meet WCAG 2.2 AA; and the EU European Accessibility Act, applying since 28 June 2025, names no WCAG version itself, granting a presumption of conformity to harmonised standards cited in the Official Journal. Where a rule adopts a Level AAA criterion, it says so and is stated as SHOULD.

Rules use RFC 2119 language: **MUST** / **MUST NOT** are enforced as errors; **SHOULD** / **SHOULD NOT** are flagged as warnings.

Rule IDs follow the format `FE-{CATEGORY}-{NNN}` and are permanent — once assigned, an ID is never renumbered, even if the rule is moved within the file or deprecated. Categories: `SEMHTML` (semantic HTML), `KBD` (keyboard navigation and focus), `ARIA` (ARIA usage), `CONTRAST` (color and contrast), `A11YFORM` (accessible forms), `A11YMEDIA` (text alternatives and media), `POINTER` (pointer input and target size), `LAYOUT` (reflow, zoom, text spacing, and orientation), `MOTION` (motion, flashing, and auto-playing media), `TIMING` (time limits), `A11YTEST` (accessibility testing). See `specs/008-security-rules/data-model.md` for the full schema.

These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts, including code snippets a plan quotes for its affected files.

Projects without a frontend need not opt out: `/{project}:analyze` loads every rule file present, regardless of stack or `[rules] surfaces`, and these rules fire only on a spec or plan that meets their Verification trigger. In `.ductus/config.toml`, a `[rules] surfaces` list without `frontend` stops `ductus` from installing or updating this file (a copy already on disk is left in place) and stops `/{project}:review` from selecting it; a `[[review.disabled-rule-files]]` entry with its mandatory `reason` excludes it from `/{project}:review` alone; and listing it under `[pinned] files` only stops `ductus` updates from overwriting a customized copy, which still loads for review and analyze.

## FE-SEMHTML — Semantic HTML

### FE-SEMHTML-001

> Interactive UI elements MUST be implemented with the native HTML element that has the matching semantic role: `<button>` for buttons, `<a href>` for navigation, `<input>`/`<select>`/`<textarea>` for form fields, `<details>`/`<summary>` for disclosure widgets where suitable. Generic `<div>` and `<span>` elements styled as interactive (`onclick` on a `<div>`, "fake button" with `role="button"`) MUST NOT be used when a native element suffices.

**Rationale:** Native interactive elements arrive with keyboard focus behavior, accessible names from text content, screen-reader role announcements, browser-native state (`:disabled`, `:focus-visible`), and OS-level hooks (voice control, switch control, Windows High Contrast) — none of which a `div` provides. An ARIA role does not make the browser supply any of it — the Authoring Practices Guide calls a role "a promise" the author's script must keep — so a re-implementation has to script every behavior, and whatever it misses breaks for assistive-technology users.

**Verification:** Any spec or plan that introduces interactive UI MUST name the native HTML element used per interaction. `/{project}:analyze` flags affected-files snippets where `onClick` (or a click listener) is bound to a `<div>` or `<span>` that a native `<button>` or `<a href>` would serve — whether or not it carries `role`, `tabIndex`, and keyboard handling, since adding those re-implements the native element rather than satisfying this rule.

**Source:** WCAG 2.2 §4.1.2 (Name, Role, Value), WAI-ARIA Authoring Practices Guide "Read Me First" ("No ARIA is better than Bad ARIA"; "A role is a promise"), WAI-ARIA 1.2 §1.4 (Co-Evolution of WAI-ARIA and Host Languages)

### FE-SEMHTML-002

> Document structure MUST use landmark elements: `<header>`, `<nav>`, `<main>` (exactly one per page), `<footer>`, and `<aside>` for their semantic purposes. Pages MUST have a single `<h1>` and a logically nested heading hierarchy with no skipped levels (no `<h1>` immediately followed by `<h3>`).

**Rationale:** Screen-reader users navigate by landmarks and heading levels. A page without `<main>` forces sequential reading from the top; a broken heading hierarchy makes the document outline noise. Landmark structure is the assistive-tech equivalent of a sitemap. WCAG does not itself mandate a single `<h1>` or forbid skipped levels; this rule adopts both as the checkable form of §1.3.1 and §2.4.6.

**Verification:** Any spec or plan that introduces a new page or top-level UI surface MUST commit to landmark structure and a heading-hierarchy plan. `/{project}:analyze` flags page specs that omit `<main>` or that describe heading usage with skipped levels.

**Source:** WCAG 2.2 §1.3.1 (Info and Relationships), §2.4.1 (Bypass Blocks), §2.4.6 (Headings and Labels)

### FE-SEMHTML-003

> Every page MUST declare its primary language with a valid BCP 47 tag on the root `<html lang>` attribute, and a passage in a different language MUST carry its own `lang` attribute (proper names, technical terms, words of indeterminate language, and phrases absorbed into the surrounding language excepted). Every page and every client-side route MUST have a descriptive `<title>` naming its topic or purpose, and a client-side navigation MUST update `document.title` to the new route's title.

**Rationale:** Screen readers load pronunciation rules from `lang`, and visual browsers use it to display characters and scripts correctly; a missing or wrong tag makes a French page read with English phonetics, which is unintelligible. Hyphenation, spell-checking, and translation tools also key on it. User agents show the `<title>` in the window title bar or as the tab's name, and it is how people with visual disabilities tell open pages apart. W3C's Understanding 2.4.2 says a single-page application should change the title to reflect each view; one that never updates it leaves every route with the same title.

**Verification:** Any spec or plan that introduces a page, a client-side route, or the application shell MUST commit to the document language and a per-page title pattern. `/{project}:analyze` flags affected-files snippets whose root `<html>` element lacks a `lang` attribute, and flags client-side routing specs that do not update `document.title` on navigation.

**Source:** WCAG 2.2 §2.4.2 (Page Titled), §3.1.1 (Language of Page), §3.1.2 (Language of Parts), W3C Understanding 2.4.2 and 3.1.1, W3C Internationalization "Why use the language attribute?", HTML Living Standard (the `lang` attribute: a valid BCP 47 language tag)

## FE-KBD — Keyboard Navigation and Focus

### FE-KBD-001

> All interactive UI MUST be operable by keyboard alone. Every interactive element MUST be reachable by keyboard — via `Tab` for a standalone control, or via arrow keys inside a composite widget (menu, tablist, listbox, grid, tree) whose single tab stop is reachable via `Tab` — and operable via documented key bindings (`Enter`/`Space` for buttons, arrow keys within composite widgets per the WAI-ARIA Authoring Practices Guide). Mouse-only or pointer-only interactions MUST NOT exist, except where the underlying function depends on the path of the user's movement (freehand drawing, watercolor painting), which §2.1.1 exempts.

**Rationale:** Keyboard operability is what many assistive technologies build on: W3C's Understanding 2.1.1 names speech input, sip-and-puff, on-screen keyboards, and scanning software among the keyboard emulators, and people who are blind operate by keyboard because a mouse needs eye-hand coordination. A mouse-only interaction is unavailable to all of them, and to people with hand tremors who find a mouse very difficult and use a keyboard instead.

**Verification:** Any spec or plan that introduces interactive UI MUST commit to keyboard operability and name the key bindings for any non-default interaction (menus, dialogs, custom widgets). `/{project}:analyze` flags interactive-UI specs that describe drag-and-drop, hover-reveal, or pointer-only flows without a keyboard equivalent.

**Source:** WCAG 2.2 §2.1.1 (Keyboard), W3C Understanding 2.1.1, WAI-ARIA Authoring Practices Guide ("Developing a Keyboard Interface")

### FE-KBD-002

> Visible focus indication MUST be present on every interactive element when it receives keyboard focus. The default browser focus ring MUST NOT be removed (`outline: none`, `outline: 0`) without an author-styled replacement, and an author-styled focus indicator MUST have a contrast ratio of at least 3:1 against the colors adjacent to it.

**Rationale:** Keyboard users navigate by following the focus ring. Removing it without replacement leaves them blind — they cannot tell which element they are about to activate. Removing the ring without a replacement is also easy to detect mechanically. At Level AA, WCAG measures an author-styled indicator through §1.4.11 (Non-text Contrast), which applies to the visual information that identifies a component's focused state; the browser's unmodified default ring is exempt from that ratio but must still be visible. The larger-area bar of §2.4.13 (Focus Appearance) is Level AAA and is `FE-KBD-005`.

**Verification:** Any spec or plan that introduces styling for interactive elements MUST commit to visible focus indication and, where focus styles are customized, to a 3:1 indicator contrast against adjacent colors. `/{project}:analyze` flags CSS in affected files that contains `outline: none` or `outline: 0` on focusable elements without an accompanying `:focus`/`:focus-visible` replacement, and flags documented focus-indicator colors below 3:1 against the surfaces they sit on.

**Source:** WCAG 2.2 §2.4.7 (Focus Visible), §1.4.11 (Non-text Contrast; W3C Understanding 1.4.11, "Relationship with Focus Visible")

### FE-KBD-003

> Modal dialogs and disclosure UI that take over the page MUST trap focus while open (Tab and Shift+Tab cycle within the dialog) and MUST return focus to the triggering element on close. Pressing `Escape` MUST close the dialog. A native `<dialog>` opened with `showModal()`, which makes the rest of the document inert, or a page whose content outside the dialog is made `inert`, MAY satisfy the trap provided focus never reaches the page beneath, whether or not it can reach the browser's own controls. Where the triggering element no longer exists, or the dialog's task leads directly to a next step elsewhere in the workflow, focus MAY move on close to the element that continues that workflow instead.

**Rationale:** Without a focus trap, Tab takes the user out of the dialog and into the page underneath — interacting with elements they cannot see. Without focus restoration, the user loses their place when the dialog closes. Both behaviors are disorienting; together they make modal UI effectively unusable with a keyboard or screen reader. The `Escape` exit is what keeps a modal's focus containment consistent with §2.1.2: the user can always leave by keyboard.

**Verification:** Any spec or plan that introduces modal dialogs, drawers, or full-screen overlays MUST commit to focus-trap, focus-restoration, and Escape-key dismissal. `/{project}:analyze` flags modal-UI specs that omit any of the three.

**Source:** WAI-ARIA Authoring Practices Guide: Dialog (Modal) Pattern, WCAG 2.2 §2.1.2 (No Keyboard Trap), §2.4.3 (Focus Order), HTML Living Standard (the `dialog` element: `showModal()` blocks the document by the modal dialog)

### FE-KBD-004

> When an element receives keyboard focus, it MUST NOT be entirely hidden by author-created content — sticky or fixed headers and footers, cookie and consent banners, chat launchers, toasts, or non-modal overlays. Content the user opened MAY cover the focused element when the user can reveal that element without advancing keyboard focus (for example, by dismissing the content with `Escape`).

**Rationale:** A keyboard user tabbing down a page with a sticky header or a cookie banner reaches controls that scroll underneath the fixed region: focus lands on an element they cannot see, and the focus ring `FE-KBD-002` requires is drawn underneath the banner. The failure appears only when sticky UI and keyboard navigation meet, so it survives mouse-driven testing.

**Verification:** Any spec or plan that introduces sticky or fixed-position UI, or persistent banners and launchers, MUST commit to how focused elements stay at least partially visible (e.g., `scroll-padding-top` / `scroll-margin` sized to the sticky region, banners that do not overlap page content). `/{project}:analyze` flags affected-files snippets that introduce `position: fixed` or `position: sticky` regions with no `scroll-padding` / `scroll-margin` offset, and specs that introduce persistent overlays without addressing focus visibility.

**Source:** WCAG 2.2 §2.4.11 (Focus Not Obscured (Minimum), new in 2.2), W3C Technique C43 (Using CSS scroll-padding to un-obscure content)

### FE-KBD-005

> Author-styled focus indicators SHOULD meet WCAG 2.2 §2.4.13 (Focus Appearance, Level AAA): an indicator area at least as large as a 2 CSS pixel thick perimeter of the unfocused component, with a contrast ratio of at least 3:1 between the same pixels in the focused and unfocused states.

**Rationale:** `FE-KBD-002` guarantees an indicator exists and contrasts with its surroundings, but a 1-pixel ring or a faint color shift can pass it and still be hard to find on a busy page, especially for low-vision users. §2.4.13 sets a measurable floor on both the size of the indicator and how much the focused state differs from the unfocused one. It is Level AAA, so it is advisory here.

**Verification:** Any spec or plan that introduces custom focus styles SHOULD commit to the §2.4.13 area and change-of-contrast bar. `/{project}:analyze` emits a warning when a custom focus-style spec names no indicator thickness, or names one thinner than 2 CSS pixels (e.g., a 1px outline).

**Source:** WCAG 2.2 §2.4.13 (Focus Appearance, Level AAA, new in 2.2)

### FE-KBD-006

> Keyboard focus order MUST follow the visual and reading order of the page. Positive `tabindex` values (`tabindex="1"` and above) MUST NOT be used. When the focused element is removed or replaced — a client-side route change, a deleted list item, a closed panel — focus MUST be moved to a logical target (the new view's heading or main region, the adjacent item, the triggering control) rather than dropped to `<body>`.

**Rationale:** A positive `tabindex` pulls an element ahead of everything else in the tab sequence, so focus jumps around the page in an order no sighted user would predict and every later change to the page has to renumber around it. Focus lost to `<body>` after a route change or a deletion sends keyboard and screen-reader users back to the top of the document with no announcement of what changed. CSS reordering (`order`, grid placement, `row-reverse`) that makes the visual order diverge from DOM order produces the same disorientation.

**Verification:** Any spec or plan that introduces client-side routing, or UI that removes or replaces the focused element, MUST name the focus target after the change. `/{project}:analyze` flags affected-files snippets with a `tabindex` / `tabIndex` value greater than 0, and flags client-side routing specs that omit post-navigation focus handling.

**Source:** WCAG 2.2 §2.4.3 (Focus Order), §1.3.2 (Meaningful Sequence), W3C Understanding 2.4.3 (Technique C27, Failure F44), WAI-ARIA Authoring Practices Guide ("Developing a Keyboard Interface": authors are strongly advised not to use `tabindex` to set tab order)

### FE-KBD-007

> Additional content that pointer hover or keyboard focus makes visible — custom tooltips, hover cards, sub-menus, and other non-modal popups — MUST be dismissible without moving hover or focus (for example, with `Escape`), unless it communicates an input error or neither obscures nor replaces other content; MUST stay visible while the pointer moves from the trigger onto it, where hover can trigger it; and MUST remain visible until hover or focus leaves, the user dismisses it, or its information is no longer valid. Content whose presentation the browser controls and the author has not modified, such as the tooltip of the HTML `title` attribute, MAY behave as the browser provides.

**Rationale:** Screen-magnifier users see only a small part of the page, and the magnified view follows the pointer or focus, so a tooltip that cannot be dismissed without moving away hides the content they were reading. Content that vanishes when the pointer leaves the trigger cannot be read by someone who has to move the pointer onto it to see it under magnification or past a large cursor, and content that disappears on a timer gives users who need more time to perceive it no chance to.

**Verification:** Any spec or plan that introduces custom tooltips, hover cards, hover- or focus-triggered menus, or non-modal popovers MUST commit to dismissal without moving hover or focus, to keeping the content visible while it is hovered, and to persistence until the trigger loses hover or focus. `/{project}:analyze` flags affected-files snippets that show content on `mouseenter` or `mouseover` and hide it on the trigger's `mouseleave` without keeping it open while the pointer is over the content, that show content which obscures or replaces other content with no `Escape` (or equivalent) dismissal, or that hide hover- or focus-triggered content on a timer while the trigger still has hover or focus.

**Source:** WCAG 2.2 §1.4.13 (Content on Hover or Focus), W3C Understanding 1.4.13 (Technique SCR39, Failure F95)

### FE-KBD-008

> A keyboard shortcut implemented using only letter, punctuation, number, or symbol characters — a single key such as `k` or `?`, or a sequence such as `g` then `a` — MUST be one the user can turn off or remap to include a non-printable key (Ctrl, Alt), unless the shortcut is active only while the component it belongs to has focus.

**Rationale:** Speech-input users dictate into the page, and their speech arrives as strings of letters, so a page that acts on bare character keys turns a sentence into a burst of commands — W3C's example is a webmail user whose "Hey Kim", picked up by the microphone, archives one message, moves down a conversation, and mutes another. Keyboard users prone to stray keypresses trigger the same accidents. Requiring a modifier, or letting the user switch the shortcuts off, removes the hazard while keeping shortcuts for those who want them.

**Verification:** Any spec or plan that introduces keyboard shortcuts MUST list them and, for each made only of character keys, name the setting that turns it off or remaps it, or state that it is active only while its component has focus. `/{project}:analyze` flags affected-files snippets with a `document`- or `window`-level `keydown`, `keypress`, or `keyup` listener that acts on a printable key without checking for a modifier (`ctrlKey`, `altKey`, `metaKey`) and with no setting that disables it.

**Source:** WCAG 2.2 §2.1.4 (Character Key Shortcuts), W3C Understanding 2.1.4 (Technique G217, Failure F99)

## FE-ARIA — ARIA Usage

### FE-ARIA-001

> ARIA roles, states, and properties MUST be used to fill gaps in native HTML semantics — not to override them. A native HTML element with the needed semantics and behavior built in MUST take precedence over an ARIA role: `role="button"` on a `<div>` is forbidden when a `<button>` would serve. ARIA roles MUST be added only when no native element exists for the pattern (tabs, comboboxes, treeviews, custom listboxes, etc.), only on a host element that ARIA in HTML allows the role on (`role="tab"` on a `<button>` is allowed; `role="heading"` on a `<button>` is not), and MUST NOT restate an element's implicit role (`<button role="button">`). A list whose markers are removed by styling MAY restate `role="list"`, because some browsers otherwise stop exposing it as a list. ARIA states and properties MAY be added to native elements to expose state or relationships HTML cannot express (`aria-expanded`, `aria-current`, `aria-describedby`, `aria-invalid`).

**Rationale:** Wrong ARIA is worse than no ARIA — it overrides correct native semantics with a broken substitute. Common ARIA misuse is unnecessary or overriding ARIA: `aria-label` on text already labeled, `<button role="button">`, a widget role on a `<div>` that should have been a native control, `aria-hidden` on focusable elements (which, W3C's ACT rule notes, lets sequential focus reach an element hidden from the accessibility tree). The "fill gaps, don't override" discipline eliminates the bulk of these failures by construction. States and properties are the exception the rule permits, because HTML has no native way to say a disclosure is expanded or an input is invalid — and other rules in this file require exactly those attributes.

**Verification:** Any spec or plan that introduces custom widgets MUST justify each ARIA role/state/property against the WAI-ARIA Authoring Practices Guide pattern it implements. `/{project}:analyze` flags affected-files snippets that restate an element's implicit role (`<button role="button">`, `<a href role="link">`) other than the styled-list case above, that apply a role ARIA in HTML does not allow on its host element, that put an ARIA widget role on a `<div>`/`<span>` where a native element provides the pattern, or that apply `aria-hidden="true"` to focusable elements.

**Source:** WAI-ARIA 1.2 §1.4 (Co-Evolution of WAI-ARIA and Host Languages), W3C ARIA in HTML §1 (Author requirements for use of ARIA in HTML), §3.1 (Avoid overriding interactive elements with non-interactive roles), §3.2 (Avoid specifying redundant roles), W3C ACT rule 6cfa84 "Element with aria-hidden has no content in sequential focus navigation" (mapped to WCAG 2.2 §4.1.2), WAI-ARIA Authoring Practices Guide "Read Me First"

### FE-ARIA-002

> Every interactive element MUST have an accessible name. Where visible text serves as the name (button labels, link text), no `aria-label` is required. Where the element has no visible text (icon-only buttons, image links, decorative-text-only links), a meaningful name describing the action or destination MUST be provided — by native means where available (`alt` text on an image inside a link, visually hidden text, an SVG `<title>`) or by `aria-label` / `aria-labelledby`. Generic names (`"button"`, `"link"`, `"click here"`, `"read more"` without context) MUST NOT be used.

**Rationale:** Screen readers announce elements by their accessible name. An icon-only button with no name announces as `"button"` — useless. A meaningful name (`"close dialog"`, `"add to cart"`, `"read more about pricing"`) makes the UI navigable by ear, and it is the text assistive technology shows when it offers the user a list of the page's links to choose from (W3C Understanding 2.4.4).

**Verification:** Any spec or plan that introduces icon-only buttons, image links, or non-text interactive elements MUST commit to an accessible-name strategy per element. `/{project}:analyze` flags affected-files snippets with a `<button>` containing only an `<svg>` or icon character and no name from any source (`aria-label`, `aria-labelledby`, visually hidden text, SVG `<title>`), flags `<a>` elements whose only content is an `<img>` with an empty or missing `alt`, and flags accessible-name values from a generic-name denylist.

**Source:** WCAG 2.2 §4.1.2 (Name, Role, Value), §2.4.4 (Link Purpose (In Context))

### FE-ARIA-003

> When an interactive element has a visible text label, its accessible name MUST contain that visible text, preferably at the start. An `aria-label` or `aria-labelledby` that replaces an element's visible text with different wording MUST NOT be used.

**Rationale:** Speech-input users operate controls by saying what they see — "click Send". A button visibly labeled "Send" whose `aria-label` is "Submit message" does not respond to that command, and a screen-reader user who hears "Submit message" cannot match it to the sighted colleague's instruction to press Send. Keeping the visible words inside the name keeps every input method addressing the same control by the same words.

**Verification:** Any spec or plan that introduces `aria-label` or `aria-labelledby` on elements that also carry visible text MUST justify the override. `/{project}:analyze` flags affected-files snippets where an element with visible text content carries an `aria-label` that does not contain that text.

**Source:** WCAG 2.2 §2.5.3 (Label in Name)

### FE-ARIA-004

> Status messages that appear without moving focus — the result of an action (saved, submitted, added to cart), progress and completion of asynchronous work, search-result counts, toasts, and error summaries shown on submit — MUST be exposed through a live region (`role="status"`, `role="alert"`, `role="log"`, `<output>`, or `aria-live`) so assistive technology announces them without the message receiving focus.

**Rationale:** A sighted user sees "Saved" appear in a corner; a screen-reader user, whose focus is still on the Save button, hears nothing and cannot tell whether the action worked. A live region is the programmatic channel that carries the message to them without stealing focus. Assistive technologies generally announce only changes to a live region that is already established, so the container is rendered empty — most reliably in the initial markup — and filled later rather than inserted with its message; `role="alert"` content is usually announced even when inserted with its message, but behavior varies across browser and screen-reader combinations.

**Verification:** Any spec or plan that introduces asynchronous feedback, toasts, inline save states, or result counts MUST name the live-region role per message type (`status` for advisory messages, `alert` for urgent errors). `/{project}:analyze` flags async-feedback specs with no live-region commitment, and flags affected-files snippets that inject toast or status text into a container lacking a live-region role or `aria-live`.

**Source:** WCAG 2.2 §4.1.3 (Status Messages), W3C Understanding 4.1.3 (status message examples), WAI-ARIA 1.2 (`status`, `alert`, and `log` roles; `aria-live`), W3C ARIA in HTML (`output` has implicit role `status`), MDN "ARIA live regions"

## FE-CONTRAST — Color and Contrast

### FE-CONTRAST-001

> Text and functional graphics MUST meet the WCAG 2.2 AA contrast ratios against adjacent colors: 4.5:1 for normal text; 3:1 for large text (≥18pt, or ≥14pt bold — W3C's Understanding 1.4.3 gives these as approximately 24 and 18.5 CSS pixels); and 3:1 for the visual information needed to identify UI components and their states and for graphics needed to understand content (form-field borders, focus indicators, status indicators, icons that convey information). Color combinations that fail these ratios MUST NOT be used for text or for functional UI elements. Inactive (disabled) components, purely decorative text and graphics, logotypes, and components whose appearance the browser determines and the author has not modified MAY fall below these ratios, per the criteria's own exceptions.

**Rationale:** Low-contrast text is hard or impossible to read for users with low vision or color-deficient vision. The thresholds are not aesthetic guidelines: W3C chose 4.5:1 for Level AA to compensate for the loss of contrast sensitivity that usually accompanies vision of about 20/40, commonly reported as typical acuity at roughly age 80.

**Verification:** Any spec or plan that introduces a color palette, theme, or text-on-color UI element MUST commit to AA contrast across all foreground/background pairs in use. `/{project}:analyze` runs against the project's documented palette and flags pair combinations below threshold; flags affected-files snippets that hard-code colors without referencing the palette.

**Source:** WCAG 2.2 §1.4.3 (Contrast (Minimum)), §1.4.11 (Non-text Contrast), W3C Understanding 1.4.3

### FE-CONTRAST-002

> Information MUST NOT be conveyed by color alone. Status, validation state, required-field marking, and selection state MUST be reinforced by a non-color cue: icon, text label, underline, weight, pattern, or position.

**Rationale:** Color-only signals are invisible to color-blind users (about 1 in 12 men and 1 in 200 women), to grayscale displays, to screen readers, and to grayscale printouts. A red border with no error icon and no error text is silent to a significant fraction of users — they see the form reject their submission and do not know why.

**Verification:** Any spec or plan that introduces state indicators (errors, warnings, success, selection, required) MUST commit to a non-color cue per state. `/{project}:analyze` flags spec descriptions that describe state via color only ("the field turns red"), and flags affected-files snippets that style validation state without an accompanying icon or text node.

**Source:** WCAG 2.2 §1.4.1 (Use of Color), Colour Blind Awareness "About Colour Blindness" (prevalence), US National Eye Institute "Color Blindness"

## FE-A11YFORM — Accessible Forms

### FE-A11YFORM-001

> Every form input MUST have a programmatically associated label. The label MUST be one of: a `<label>` element with `for` matching the input's `id`, an enclosing `<label>` wrapping the input, or an `aria-labelledby` reference to a visible text element. Placeholder text MUST NOT substitute for a label. A field whose purpose is conveyed by an adjacent visible button (a search box beside a "Search" button) MAY be named with `aria-label` instead.

**Rationale:** Screen readers announce labels when focus enters an input — without a programmatic association, the input is just "edit text." A placeholder is shown only while the field is empty — some browsers hide it on focus — so it disappears once the user types or the browser autofills, leaving no field identity at the moment the user needs it; the HTML standard says it should not be used as an alternative to a label. The label↔input pairing is also what lets a click on the label activate a checkbox or radio button on platforms that support it — the difference between a small click target and a usable one for users with motor impairments.

**Verification:** Any spec or plan that introduces a form MUST commit to programmatic labels per input. `/{project}:analyze` flags affected-files snippets with an `<input>` (other than `type="hidden"`, `submit`, `reset`, `button`, or `image`), `<select>`, or `<textarea>` that has no associated `<label>` and no `aria-labelledby` reference — including fields identified only by a `placeholder`, a `title`, or an `aria-label` outside the adjacent-button case.

**Source:** WCAG 2.2 §1.3.1 (Info and Relationships), §3.3.2 (Labels or Instructions), §4.1.2 (Name, Role, Value), W3C Technique G167 (Using an adjacent button to label the purpose of a field), HTML Living Standard (the `placeholder` attribute; the `label` element)

### FE-A11YFORM-002

> Form validation errors MUST be associated with their inputs via `aria-describedby` pointing at the error message element, AND inputs in an error state MUST set `aria-invalid="true"`. Error messages MUST be visible (not screen-reader-only) and MUST identify the field by name in their text. Where the expected format or allowed values are known, the error message MUST state how to correct the entry, unless doing so would compromise security (e.g., revealing which credential was wrong).

**Rationale:** Screen-reader users move from input to input via Tab; the `aria-describedby` reference makes the error message part of the field's accessible description, which assistive technology can present when the field receives focus. `aria-invalid` exposes the field's error state, which WAI-ARIA 1.2 defines as an entered value that "does not conform to the format expected by the application"; without it, an error the application's own validation detects exists only in visual styling. Visible, named errors help every user — not just screen-reader users — because they survive losing focus on the offending field, and a stated correction ("Enter the date as DD/MM/YYYY") turns a rejection into a fix. An error summary that appears on submit without moving focus is a status message and also falls under `FE-ARIA-004`.

**Verification:** Any spec or plan that introduces form validation UI MUST commit to `aria-describedby` + `aria-invalid` per error-bearing input and to correction text for format-constrained fields. `/{project}:analyze` flags form-validation specs that omit either ARIA attribute, that hide errors visually (a visually-hidden or `sr-only` class, or `display: none` / `visibility: hidden` on an element still referenced by `aria-describedby`) while revealing them only to screen readers, that surface errors without naming the offending field, or that reject a format-constrained field without saying what format is expected.

**Source:** WCAG 2.2 §3.3.1 (Error Identification), §3.3.3 (Error Suggestion), §4.1.2 (Name, Role, Value), W3C Technique ARIA21 (Using aria-invalid to Indicate An Error Field), WAI-ARIA 1.2 (`aria-invalid`)

### FE-A11YFORM-003

> Inputs that collect information about the user MUST carry the HTML `autocomplete` token matching their purpose (`name`, `given-name`, `email`, `tel`, `street-address`, `postal-code`, `country`, `bday`, `username`, and the rest of WCAG 2.2's Input Purposes list). Credential, one-time-code, and payment fields carry the same obligation under `FE-PII-002`.

**Rationale:** A correct `autocomplete` token lets the browser fill the field from the user's stored details, which removes typing for users with motor impairments and recall for users with memory or cognitive impairments, and it lets assistive technology present the field's purpose in a familiar form (W3C's Understanding 1.3.5 gives a birthday-cake icon beside `autocomplete="bday"`). Without the token a browser can only guess the purpose heuristically, which the same document says does not meet the criterion.

**Verification:** Any spec or plan that introduces a form collecting personal data (identity, contact, address, birth date) MUST name the `autocomplete` token per field. `/{project}:analyze` flags affected-files snippets with inputs whose `type`, `name`, or `id` indicates personal data (`email`, `tel`, `name`, `address`, `postal`/`zip`) but that carry no `autocomplete` attribute or `autocomplete="off"`.

**Source:** WCAG 2.2 §1.3.5 (Identify Input Purpose), WCAG 2.2 §7 (Input Purposes for User Interface Components), W3C Understanding 1.3.5, HTML Living Standard (Autofill, the `autocomplete` attribute)

### FE-A11YFORM-004

> Authentication flows (sign-in, sign-up, multi-factor, account recovery) MUST NOT require a cognitive function test at any step — remembering a password, transcribing a code, solving a puzzle, performing a calculation — unless that step also offers an alternative method without one or a mechanism that assists the user with it. Password and one-time-code fields MUST permit paste and password-manager autofill. A CAPTCHA MUST NOT be the only path through a step when it requires text transcription or puzzle solving; object-recognition and personal-content checks MAY be used, per the criterion's exceptions.

**Rationale:** People with cognitive limitations relating to memory, reading (dyslexia), numbers (dyscalculia), or perception processing cannot reliably pass a test that depends on recall, transcription, or calculation. Paste and password-manager autofill are the assisting mechanism WCAG names: they turn "remember this password" into "retrieve it". Blocking paste or autofill on password or code fields removes that mechanism, and W3C's Understanding 3.3.8 says such a page fails the criterion unless an alternative is provided — including a one-time code split across per-digit inputs that drop a pasted code.

**Verification:** Any spec or plan that introduces sign-in, sign-up, multi-factor, account recovery, or a CAPTCHA MUST commit to paste and autofill support on credential and code fields and name the non-cognitive alternative for any puzzle or transcription step. `/{project}:analyze` flags affected-files snippets that block paste on credential or code inputs (`onpaste` handlers that call `preventDefault`), split one-time codes across per-digit inputs that do not accept a pasted code, and flags authentication specs whose only bot check is a text-transcription or puzzle CAPTCHA.

**Source:** WCAG 2.2 §3.3.8 (Accessible Authentication (Minimum), new in 2.2), W3C Understanding 3.3.8, `FE-PII-002`

### FE-A11YFORM-005

> Within a multi-step process, information the user has already entered MUST NOT be requested again unless it is auto-populated or offered for selection (e.g., a "billing address same as shipping" option). Re-entry MAY be required when it is essential to the activity, when it is needed for security, or when the earlier value is no longer valid.

**Rationale:** Re-typing the same address or account number on a later step costs users with motor impairments effort and users with memory or cognitive impairments accuracy, and every re-entry is another chance for an error that `FE-A11YFORM-002` then has to report. Carrying the earlier value forward removes the step rather than accommodating it.

**Verification:** Any spec or plan that introduces a multi-step flow (checkout, onboarding, an application wizard) MUST identify each data point requested in more than one step and commit to auto-population or selection for it. `/{project}:analyze` flags multi-step-flow specs that request the same data point in two steps without naming the reuse mechanism.

**Source:** WCAG 2.2 §3.3.7 (Redundant Entry, new in 2.2)

### FE-A11YFORM-006

> A component receiving focus MUST NOT by itself initiate a change of context — submitting a form, opening a new window, moving focus to another component, or navigating to a new page. Changing the setting of a control — selecting an option, checking a box, choosing a radio button, entering text — MUST NOT automatically cause a change of context unless the user is advised of the behavior before using the control.

**Rationale:** WCAG defines a change of context as one that can disorient users who cannot view the entire page at once, and W3C adds that it confuses users who do not easily perceive the change or are easily distracted — so merely tabbing to a field or picking an option must not take them somewhere else. A keyboard user moving through a `<select>`'s options changes its value at each step, so one that acts on change fires before the user reaches the option they wanted. Making a change of context follow an explicit activation, such as a button beside the `<select>`, lets the user inspect values and correct a wrong choice first.

**Verification:** Any spec or plan that introduces a control whose focus or change triggers navigation, form submission, a new window, or a focus move MUST name the explicit activation (a submit or go button) that performs it instead, or the advance notice given before the control is used. `/{project}:analyze` flags affected-files snippets with a `focus` handler, or a `change` or `input` handler on a `<select>`, checkbox, radio button, or text field, that navigates (`location` assignment, router navigation), submits the form, calls `window.open`, or moves focus elsewhere.

**Source:** WCAG 2.2 §3.2.1 (On Focus), §3.2.2 (On Input), WCAG 2.2 definition of "changes of context", W3C Understanding 3.2.1 and 3.2.2 (Techniques G80, H84, G13; Failures F36, F37)

### FE-A11YFORM-007

> A submission that causes a legal commitment or a financial transaction for the user, that modifies or deletes user-controllable data in storage, or that submits user test responses MUST be reversible, MUST check the entered data for input errors and give the user a chance to correct them, or MUST let the user review, confirm, and correct the information before it is finalized — at least one of the three. A save command, and the simple creation or editing of a document or record, MAY proceed without any of them.

**Rationale:** A mistyped travel date on a non-refundable ticket or an extra digit in a share purchase cannot be taken back, and W3C notes that users with disabilities may be more likely to make such mistakes. Reversal, validation with correction, or a review step gives them the chance to catch it before it costs them. W3C's Understanding 3.3.4 aims the data clause at mass loss, such as deleting a file or record, not at confirming every save.

**Verification:** Any spec or plan that introduces a purchase, payment, or other financial transaction, acceptance of a contract or other legal commitment, deletion of user-controllable data (an account, a file, a record, or a bulk deletion), or submission of test answers MUST name which of the three safeguards it provides. `/{project}:analyze` flags such specs that finalize the action on a single activation with no undo, no validation with an opportunity to correct, and no review or confirmation step.

**Source:** WCAG 2.2 §3.3.4 (Error Prevention (Legal, Financial, Data)), W3C Understanding 3.3.4 (Techniques G98, G99, G164, G168)

## FE-A11YMEDIA — Text Alternatives and Media

### FE-A11YMEDIA-001

> Every `<img>` element MUST have an `alt` attribute. Informational images MUST have descriptive alt text; purely decorative images MUST have `alt=""` (empty string) so screen readers skip them. SVG icons used as interactive triggers MUST carry an accessible name per `FE-ARIA-002`, either via the enclosing element's `aria-label` or via `<title>` inside the SVG.

**Rationale:** With `alt` missing, a screen reader may fall back to announcing the image's file name or URL — useless and noisy. An empty `alt` correctly skips decorative images. The distinction between informational and decorative is a content-author judgement the `alt` attribute exists to capture — omitting it forces the screen reader to guess, badly.

**Verification:** Any spec or plan that introduces image rendering MUST commit to an alt-text policy. `/{project}:analyze` flags affected-files snippets with `<img>` elements lacking an `alt` attribute, and flags SVG-as-button patterns without an accessible name.

**Source:** WCAG 2.2 §1.1.1 (Non-text Content), W3C Technique H67 (Using null alt text and no title attribute on img elements for images that assistive technology should ignore), WebAIM "Alternative Text"

### FE-A11YMEDIA-002

> Prerecorded video with audio MUST provide synchronized captions; prerecorded audio-only content MUST provide a transcript; prerecorded video-only content MUST provide a text alternative or an audio track that conveys the same information. Captions and transcripts for content authored by the application MUST be human-edited (not solely auto-generated).

**Rationale:** Without captions, video is inaccessible to deaf and hard-of-hearing users and to anyone in a sound-off context (open offices, public transit, libraries). Auto-generated captions are useful as a starting point, but W3C notes their text is often wrong in ways that change the meaning — missing a single "not" makes the captions contradict the audio — and that they do not meet user needs or accessibility requirements unless confirmed fully accurate. Video with no soundtrack is equally closed to blind users unless its content is available as text or audio. Live video is `FE-A11YMEDIA-003`.

**Verification:** Any spec or plan that introduces video or audio media MUST commit to captions/transcripts and to the editorial pipeline that produces them. `/{project}:analyze` flags media-introducing specs that omit caption/transcript commitments or that rely solely on auto-generation for application-authored content.

**Source:** WCAG 2.2 §1.2.1 (Audio-only and Video-only (Prerecorded)), §1.2.2 (Captions (Prerecorded)), W3C WAI "Captions/Subtitles" ("Automatic Captions are Not Sufficient")

### FE-A11YMEDIA-003

> All prerecorded video content in synchronized media that the application's pages present — whether the application produces it, embeds it from another source, or accepts it from users — MUST have audio description of the important visual details that cannot be understood from the main soundtrack alone, for example as a user-selectable described-audio track, a described version of the video, or narration of those details in the main soundtrack. All live audio content in synchronized media those pages present MUST have captions. Media that is a media alternative for text and is clearly labeled as such is not synchronized media under WCAG 2.2's definition and is outside both obligations; where the existing audio already provides all of the video information, no additional audio description is necessary.

**Rationale:** Captions serve users who cannot hear; audio description serves users who cannot see. A product demo whose narrator says "now click here" while the screen shows which button is meaningless to a blind viewer unless the visual step is described. Live streams are equally closed to deaf and hard-of-hearing viewers without real-time captions, and an after-the-fact caption file does not help someone watching live. The obligation does not depend on who produced the media. §1.2.4 and §1.2.5 cover "all" live audio and prerecorded video content in synchronized media; conformance is for full web pages and "cannot be achieved if part of a web page is excluded" (§5.2.2); and WCAG 2.2's definition of a web page takes in the other resources rendered together with it, "all embedded images and media" in its first example. For content outside the author's control, such as user uploads or aggregated feeds, §5.4 offers two options: a page that is monitored and repaired, its non-conforming content removed or brought into conformance within two business days, can still be determined to conform; otherwise the author may make a statement of partial conformance, saying the page does not conform but would if the identified parts from uncontrolled sources were removed. W3C's Understanding Conformance calls that "a statement of non-conformance": it records the gap and does not exempt the content. W3C's Understanding 1.2.4 says that criterion was intended to apply to broadcast of synchronized media and is not intended to require that two-way multimedia calls between two or more individuals through web apps be captioned regardless of the needs of users, placing responsibility for captions on the callers or the host caller rather than the application.

**Verification:** Any spec or plan that introduces pages presenting prerecorded video in synchronized media — produced by the application, embedded from another source, or contributed by users — MUST commit to an audio-description approach for it, or record that its existing audio already provides all of the video information, that it is a video of only a speaker whose visual details a static text alternative describes (W3C technique G203), or that it is a media alternative for text clearly labeled as such. Any spec or plan that introduces pages presenting live synchronized media broadcast to viewers, such as a live stream or web cast, MUST commit to a real-time captioning mechanism. Where media under either commitment comes from a source outside the application's control, the spec or plan MUST state how the obligation is met, such as requiring the contributor to supply it or monitoring and repairing within two business days as §5.4 describes; a statement of partial conformance records that the obligation is not met and does not meet it. `/{project}:analyze` flags media-introducing specs that omit the applicable commitment, including specs that commit to it only for the application's own media.

**Source:** WCAG 2.2 §1.2.5 (Audio Description (Prerecorded)), §1.2.3 (Audio Description or Media Alternative (Prerecorded)), §1.2.4 (Captions (Live)), the WCAG 2.2 definitions of synchronized media (excluding a media alternative for text that is clearly labeled as such), audio description (Note 3: where all of the video information is already provided in existing audio, no additional audio description is necessary), and web page (Example 1: a web resource including all embedded images and media), §5.2.2 (Full pages), §5.4 (Statement of Partial Conformance - Third Party Content), W3C Understanding 1.2.5 (sufficient techniques G78, G173, G226, and G203 — the last, "Using a static text alternative to describe a talking head video"), W3C Understanding 1.2.4 (intended for broadcast of synchronized media; not intended to require captioning of two-way multimedia calls regardless of the needs of users), W3C Understanding Conformance ("Partial conformance claims due to third party content")

## FE-POINTER — Pointer Input and Target Size

### FE-POINTER-001

> Pointer targets MUST be at least 24×24 CSS pixels, or be spaced so that a 24 CSS pixel diameter circle centered on the bounding box of each undersized target intersects no other target and no other undersized target's circle. Targets in a sentence or whose size is otherwise constrained by the line-height of non-target text, targets whose size the browser determines and the author has not modified, targets whose function another control on the same page that meets this size also performs, and targets whose particular presentation is essential or legally required MAY be smaller.

**Rationale:** Small targets close to other targets are hard to activate without hitting a neighbor for people with hand tremors, spasticity, or other dexterity limitations, and for users of specialized input devices less accurate than a mouse; enough size or spacing also makes touchscreens easier to use. A mis-hit close or delete button is an error the user then has to recover from. 24×24 is the WCAG AA floor; §2.5.5 (Target Size (Enhanced), Level AAA) sets 44×44 CSS pixels.

**Verification:** Any spec or plan that introduces interactive controls (icon buttons, toolbars, pagination, close buttons, chips, table-row actions) MUST commit to a minimum target size of 24×24 CSS pixels or to the spacing alternative. `/{project}:analyze` flags affected-files snippets that size interactive elements below 24 CSS pixels in width or height — including icon buttons whose only box is a sub-24px icon with no padding — without spacing that satisfies the 24-pixel-circle test.

**Source:** WCAG 2.2 §2.5.8 (Target Size (Minimum), new in 2.2), §2.5.5 (Target Size (Enhanced), Level AAA), W3C Understanding 2.5.8

### FE-POINTER-002

> Functionality operated by a dragging movement (drag-and-drop reordering, sliders, map panning, resizable panes) MUST also be operable by a single pointer without dragging — taps or clicks on controls such as move-up/move-down buttons, a destination picker, or a position on a slider track. Functionality operated by multipoint or path-based gestures (pinch, two-finger swipe, swipe paths) MUST also be operable by a single pointer without a path-based gesture. Dragging or a path-based gesture MAY be the only method where the movement itself is essential (freehand drawing, signature capture), and dragging MAY also be where the browser determines the functionality and the author has not modified it.

**Rationale:** Some people cannot perform dragging movements or gestures precisely, and others use a trackball, head pointer, eye-gaze system, or speech-controlled mouse emulator that makes dragging cumbersome and error-prone; some pointing methods cannot perform multipoint or path-based gestures at all. The keyboard equivalent `FE-KBD-001` requires does not help a touchscreen user with no physical keyboard, and W3C's Understanding 2.5.7 says a keyboard equivalent meets this criterion only if it also offers controls that can be clicked or tapped, so the single-pointer alternative is a separate obligation.

**Verification:** Any spec or plan that introduces drag-and-drop, sliders, swipe, or pinch interactions MUST name the single-pointer, non-dragging alternative for each; the keyboard equivalent required by `FE-KBD-001` satisfies this rule only where it also provides controls a pointer can click or tap. `/{project}:analyze` flags interaction specs that describe drag, swipe, or pinch operations without a click or tap alternative, and flags affected-files snippets that bind drag or multi-touch handlers (`draggable`, `dragstart`, pointer-move drag logic, multi-touch checks) with no control that performs the same action.

**Source:** WCAG 2.2 §2.5.7 (Dragging Movements, new in 2.2), §2.5.1 (Pointer Gestures), W3C Understanding 2.5.7 and 2.5.1

### FE-POINTER-003

> Functionality operated by a single pointer MUST NOT execute any part of its function on the pointer's down-event (`pointerdown`, `mousedown`, `touchstart`); it completes on the up-event, as a `click` handler does. A down-event MAY begin a function that completes on the up-event when the user can abort it before completion or undo it after (a drag the user can drop back where it started), MAY trigger an outcome the up-event reverses (a preview shown only while pressed), and MAY execute the function where completing it on the down-event is essential (an on-screen keyboard or keypad).

**Rationale:** People with motor impairments, visual disabilities, or cognitive limitations can press the wrong target or start an action they did not intend. When activation waits for the up-event, they can slide the pointer off the target before releasing and nothing happens; an action fired on the down-event gives them no such escape. The JavaScript `click` event fires on release, and on touch and keyboard as well, so the native pattern already meets the rule.

**Verification:** Any spec or plan that introduces custom pointer handling — press-and-hold, drag, or controls built on pointer events rather than `click` — MUST name the event that completes each action. `/{project}:analyze` flags affected-files snippets that navigate, submit, delete, or change state inside a `pointerdown`, `mousedown`, or `touchstart` handler, unless the handler only begins a drag or press that the up-event completes, cancels, or reverses.

**Source:** WCAG 2.2 §2.5.2 (Pointer Cancellation), W3C Understanding 2.5.2 (Techniques G210, G212; Failure F101)

## FE-LAYOUT — Reflow, Zoom, Text Spacing, and Orientation

### FE-LAYOUT-001

> Content MUST remain usable, without loss of information or functionality, when the user enlarges it: pages MUST reflow to a viewport 320 CSS pixels wide without horizontal scrolling (content that needs two-dimensional layout for its meaning — data tables, maps, diagrams, video — excepted), and text other than captions and images of text MUST resize to 200% through browser zoom or text-size settings without assistive technology. The viewport meta tag MUST NOT disable or cap zoom (`user-scalable=no`, or a `maximum-scale` below 2).

**Rationale:** Low-vision users read by zooming. A 320 CSS pixel viewport is what a 1280-pixel-wide desktop window becomes at 400% zoom; a layout that does not reflow there forces the user to scroll horizontally on every line of text, which makes reading impractical. A viewport tag that disables or caps zoom blocks pinch-zoom in the mobile browsers that honor it; W3C's ACT rule notes that desktop browsers ignore the tag and most modern mobile browsers ignore it or offer a setting to override it, so the harm falls on users of the browsers that do not.

**Verification:** Any spec or plan that introduces a page layout, a responsive breakpoint set, or the viewport meta tag MUST commit to reflow at 320 CSS pixels and to 200% text resize without loss. `/{project}:analyze` flags affected-files snippets whose viewport meta tag contains `user-scalable=no` or a `maximum-scale` below 2, and flags layout specs that set fixed widths above 320 CSS pixels on page-level containers without a narrower breakpoint.

**Source:** WCAG 2.2 §1.4.10 (Reflow), §1.4.4 (Resize Text), W3C ACT rule b4f0c3 "Meta viewport allows for zoom"

### FE-LAYOUT-002

> Text containers MUST NOT clip, truncate, or overlap content when the user overrides text spacing to a line height of 1.5×, paragraph spacing of 2×, letter spacing of 0.12×, and word spacing of 0.16× the font size. Content MUST NOT restrict its display to a single orientation (portrait or landscape) unless a specific orientation is essential (a bank-check capture, a piano keyboard).

**Rationale:** Users with dyslexia and low vision apply their own spacing through browser extensions or user stylesheets; a container with a fixed height and hidden overflow cuts off whatever no longer fits. Users with a device mounted in one orientation — on a wheelchair, for example — cannot rotate it to suit an app that locks the other.

**Verification:** Any spec or plan that introduces text containers with fixed dimensions, or that constrains orientation, MUST commit to tolerating spacing overrides and to no orientation lock without an essential reason. `/{project}:analyze` flags affected-files snippets that set a fixed pixel `height` on text containers together with `overflow: hidden`, that call `screen.orientation.lock()`, or that hide or rotate content under an `orientation` media query to force one orientation.

**Source:** WCAG 2.2 §1.4.12 (Text Spacing), §1.3.4 (Orientation), W3C Understanding 1.4.12 and 1.3.4

## FE-MOTION — Motion, Flashing, and Auto-Playing Media

### FE-MOTION-001

> Content that moves, blinks, or scrolls automatically for more than five seconds alongside other content (carousels, marquees, animated banners, background video) MUST provide a control to pause, stop, or hide it, and content that updates automatically (tickers, live feeds) MUST provide a control to pause, stop, hide it, or set its update frequency. Audio that plays automatically for more than three seconds MUST provide a control to pause or stop it, or to set its volume independently of the system volume. Movement or updating that is essential to the activity MAY omit the control, such as a loading indicator shown while no interaction is possible.

**Rationale:** Movement next to the content a user is trying to read pulls attention away, and for users with attention-related or cognitive disabilities it can make the page unusable. Auto-playing audio talks over a screen reader, so the user cannot hear the very tool they would use to find the stop button.

**Verification:** Any spec or plan that introduces carousels, auto-advancing slides, animated or auto-updating regions, background video, or auto-playing audio MUST name the pause, stop, or hide control. `/{project}:analyze` flags affected-files snippets with `autoplay` on `<audio>`, or on `<video>` without `muted`, that render no controls, and flags auto-advancing carousel or ticker specs that omit a pause control.

**Source:** WCAG 2.2 §2.2.2 (Pause, Stop, Hide), §1.4.2 (Audio Control), W3C Understanding 2.2.2 and 1.4.2

### FE-MOTION-002

> Content MUST NOT flash more than three times in any one-second period, unless the flashing stays below the general flash and red flash thresholds.

**Rationale:** Content that flashes at certain frequencies for more than a few flashes can trigger seizures in people with photosensitive seizure disorders, and people are even more sensitive to red flashing, which is why WCAG has a separate saturated-red threshold. Unlike most accessibility failures, the harm is physical and immediate, so there is no remediation after the fact.

**Verification:** Any spec or plan that introduces animation, video, or visual effects with rapid luminance changes (strobing, flashing alerts, glitch effects) MUST commit to the three-flashes limit. `/{project}:analyze` flags specs that describe flashing or strobing effects without a flash-rate commitment, and flags affected-files snippets whose CSS animations or timers complete a full on-and-off cycle of visibility, opacity, or high-contrast colors in less than 333 ms (a flash is a pair of opposing changes, so a shorter cycle exceeds three flashes per second).

**Source:** WCAG 2.2 §2.3.1 (Three Flashes or Below Threshold), WCAG 2.2 definitions of "flash" and "general flash and red flash thresholds", W3C Understanding 2.3.1

### FE-MOTION-003

> Non-essential motion triggered by interaction — parallax, zoom and slide transitions, scroll-linked animation, animated page transitions — SHOULD be removed or reduced when the user has set the `prefers-reduced-motion: reduce` media feature.

**Rationale:** Non-essential motion, such as elements that move as the page scrolls or parallax, can trigger vestibular disorder reactions — dizziness, nausea, and migraine headaches. The operating-system "reduce motion" setting is how those users say so, and `prefers-reduced-motion` is how a page hears it. The underlying criterion, §2.3.3, is Level AAA, so this rule is advisory.

**Verification:** Any spec or plan that introduces animated transitions, parallax, or scroll-linked effects SHOULD commit to a reduced-motion variant. `/{project}:analyze` emits a warning when affected files define animations or transitions of `transform` or position, or use a JavaScript animation library, with no `prefers-reduced-motion` media query or `matchMedia` check governing them.

**Source:** WCAG 2.2 §2.3.3 (Animation from Interactions, Level AAA), W3C Understanding 2.3.3 (Techniques C39 and SCR40), W3C Media Queries Level 5 (`prefers-reduced-motion`)

## FE-TIMING — Time Limits

### FE-TIMING-001

> Every time limit the application sets (session inactivity timeout, timed form, held reservation) MUST let the user turn it off, adjust it to at least ten times its default length, or extend it — warning the user before it expires, giving at least 20 seconds to extend with a simple action, and allowing at least ten extensions. Real-time events (an auction), limits that are essential and that extending would invalidate the activity (a time-limited one-time code), and limits longer than 20 hours MAY be exempt.

**Rationale:** People with physical disabilities, low vision, or cognitive or language limitations, and blind screen-reader users, often need more time to read, type, and operate controls than a default limit allows, and losing a half-completed form to a silent timeout forces them to start over. `BE-AUTHN-005` governs how long a session may live and `FE-AUTHN-002` what happens after it expires; this rule governs the warning and the chance to continue before it does. A limit set for security is still a limit the application sets — W3C's Understanding 2.2.1 counts a time limit added for security concerns as set by the content, and says only certain security limits, such as time-limited two-factor authentication tokens, can be considered essential and may be exempt — so the idle timeout `BE-AUTHN-005` requires is warned about and extendable, and its absolute lifetime is chosen with this rule in view.

**Verification:** Any spec or plan that introduces a session timeout, timed task, or held reservation MUST commit to one of turn-off, adjust, or warn-and-extend, and name the warning lead time. `/{project}:analyze` flags timeout-introducing specs that end the session or task with no pre-expiry warning and extend action, or that allow less than 20 seconds to respond.

**Source:** WCAG 2.2 §2.2.1 (Timing Adjustable), W3C Understanding 2.2.1

## FE-A11YTEST — Accessibility Testing

### FE-A11YTEST-001

> A project with a web frontend MUST check its rendered pages or components with an automated accessibility rule engine (axe-core, Pa11y, Lighthouse's accessibility audit, or an equivalent WCAG-mapped checker) before each release, and a release MUST NOT ship with a new violation at or above the project's documented severity threshold.

**Rationale:** Many accessibility regressions arrive silently — a refactor drops a label, a theme change lowers contrast, a new icon button ships unnamed — and are mechanically detectable. Running an engine before each release catches that class at near-zero marginal cost, and holding the release to the threshold keeps a known, detectable violation from shipping. How and where the engine runs — CI, a pre-commit hook, a component test suite, a release script — is the project's choice; `FE-A11YTEST-003` recommends running it on every change. It cannot judge whether alt text is accurate, whether focus order makes sense, or whether a custom widget works with a screen reader, which is why `FE-A11YTEST-002` pairs it with manual testing; a green automated run is not a conformance claim.

**Verification:** `specs/system.md` or the spec that introduces the frontend MUST name the automated accessibility tool, the pages or components it covers, when it runs before release, and the release-blocking severity threshold; any spec or plan that adds a page, route, or shared component MUST extend that coverage. `/{project}:analyze` flags frontend projects with no automated accessibility-check commitment, and UI-introducing specs whose plan adds no coverage for the new surface.

**Source:** W3C WAI "Selecting Web Accessibility Evaluation Tools", Deque axe-core rule documentation

### FE-A11YTEST-002

> New custom widgets, page templates, and interaction patterns SHOULD be tested manually before release with the keyboard alone and with at least one screen reader and browser pairing (NVDA or JAWS with Chrome or Firefox on Windows, VoiceOver with Safari on macOS or iOS, TalkBack with Chrome on Android).

**Rationale:** The failures automated engines miss are the ones that decide whether a feature is usable: a focus order that jumps, a widget that announces the wrong state, a dialog that traps focus with no way out. Only operating the UI the way an assistive-technology user does surfaces them, and pairings matter because screen readers differ in how they interpret the same markup.

**Verification:** Any spec or plan that introduces a custom widget, a new page template, or a novel interaction pattern SHOULD include a manual keyboard and screen-reader test step naming the screen-reader and browser pairing. `/{project}:analyze` emits a warning when such a spec's plan or tasks contain no manual assistive-technology test step.

**Source:** W3C WCAG Evaluation Methodology (WCAG-EM) 2.0 (W3C Group Note, 23 July 2026; Methodology Requirement 1.3, defining the browser and assistive-technology combinations to evaluate), W3C WAI "Evaluating Web Accessibility Overview"

### FE-A11YTEST-003

> The automated accessibility check of `FE-A11YTEST-001` SHOULD also run automatically on every change to the pages or components it covers, failing the change on a new violation at or above the project's documented severity threshold.

**Rationale:** A check that runs only before release finds a regression after the change that caused it has merged and become entangled with other work; running it on each change attributes the violation to its cause while that change is still under review. Whether a project runs it in CI, a pre-commit hook, or a component test suite is its own choice, so this rule is advisory.

**Verification:** Any spec or plan covering frontend build, test, or CI configuration SHOULD include the automated accessibility check on each change. `/{project}:analyze` emits a warning when such a spec omits it.

**Source:** W3C WAI "Selecting Web Accessibility Evaluation Tools", Deque axe-core rule documentation
