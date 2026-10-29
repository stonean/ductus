# Accessibility Rules — Frontend

Accessibility rules for browser-rendered UI, for any project with a web frontend. They derive from WCAG 2.2 Level AA (W3C Recommendation, first published 5 October 2023 and republished 12 December 2024; the October 2023 text is ISO/IEC 40500:2025). W3C states that content conforming to WCAG 2.2 also conforms to WCAG 2.0 and 2.1: 2.2 adds criteria and removes only §4.1.1 (Parsing), now obsolete, though a project required by policy to conform to 2.0 or 2.1 may still need to test and report §4.1.1. Regimes that mandate web accessibility differ in the version they name: US Section 508 incorporates WCAG 2.0 Level A and AA; the US Department of Justice's ADA Title II rule for state and local governments adopts WCAG 2.1 Level AA; Ontario's AODA requires WCAG 2.0 Level AA except §1.2.4 and §1.2.5; GOV.UK's guidance on the UK Public Sector Bodies (Websites and Mobile Applications) (No. 2) Accessibility Regulations 2018 says to meet WCAG 2.2 AA; and the EU European Accessibility Act, applying since 28 June 2025, names no WCAG version itself, granting a presumption of conformity to harmonised standards cited in the Official Journal. Where a rule adopts a Level AAA criterion, it says so and is stated as SHOULD.

These rules verify **design-time commitments** — what a spec or plan must state — rather than code patterns; `/{project}:analyze` enforces them against feature artifacts, including code snippets a plan quotes for its affected files. Rule IDs follow the format `FE-{CATEGORY}-{NNN}`. Categories: `SEMHTML` (semantic HTML), `KBD` (keyboard navigation and focus), `ARIA` (ARIA usage), `CONTRAST` (color and contrast), `A11YFORM` (accessible forms), `A11YMEDIA` (text alternatives and media), `POINTER` (pointer input and target size), `LAYOUT` (reflow, zoom, text spacing, and orientation), `MOTION` (motion, flashing, and auto-playing media), `TIMING` (time limits), `A11YTEST` (accessibility testing). Severity, ID permanence, and how review selects or excludes this file follow constitution §rules.

## FE-SEMHTML — Semantic HTML

### FE-SEMHTML-001

> Interactive UI elements MUST be implemented with the native HTML element that has the matching semantic role: `<button>` for buttons, `<a href>` for navigation, `<input>`/`<select>`/`<textarea>` for form fields, `<details>`/`<summary>` for disclosure widgets where suitable. Generic `<div>` and `<span>` elements styled as interactive (`onclick` on a `<div>`, "fake button" with `role="button"`) MUST NOT be used when a native element suffices.

**Verification:** Any spec or plan that introduces interactive UI MUST name the native HTML element used per interaction. `/{project}:analyze` flags affected-files snippets where `onClick` (or a click listener) is bound to a `<div>` or `<span>` that a native `<button>` or `<a href>` would serve, whether or not it carries `role`, `tabIndex`, and keyboard handling.

**Source:** WCAG 2.2 §4.1.2 (Name, Role, Value), WAI-ARIA Authoring Practices Guide "Read Me First" ("No ARIA is better than Bad ARIA"; "A role is a promise"), WAI-ARIA 1.2 §1.4 (Co-Evolution of WAI-ARIA and Host Languages)

### FE-SEMHTML-002

> Document structure MUST use landmark elements: `<header>`, `<nav>`, `<main>` (exactly one per page), `<footer>`, and `<aside>` for their semantic purposes. Pages MUST have a single `<h1>` and a logically nested heading hierarchy with no skipped levels.

**Verification:** Any spec or plan that introduces a new page or top-level UI surface MUST commit to landmark structure and a heading-hierarchy plan. `/{project}:analyze` flags page specs that omit `<main>` or that describe heading usage with skipped levels.

**Source:** WCAG 2.2 §1.3.1 (Info and Relationships), §2.4.1 (Bypass Blocks), §2.4.6 (Headings and Labels)

### FE-SEMHTML-003

> Every page MUST declare its primary language with a valid BCP 47 tag on the root `<html lang>` attribute, and a passage in a different language MUST carry its own `lang` attribute (proper names, technical terms, words of indeterminate language, and phrases absorbed into the surrounding language excepted). Every page and every client-side route MUST have a descriptive `<title>` naming its topic or purpose, and a client-side navigation MUST update `document.title` to the new route's title.

**Verification:** Any spec or plan that introduces a page, a client-side route, or the application shell MUST commit to the document language and a per-page title pattern. `/{project}:analyze` flags affected-files snippets whose root `<html>` element lacks a `lang` attribute, and flags client-side routing specs that do not update `document.title` on navigation.

**Source:** WCAG 2.2 §2.4.2 (Page Titled), §3.1.1 (Language of Page), §3.1.2 (Language of Parts), W3C Understanding 2.4.2 and 3.1.1, HTML Living Standard (the `lang` attribute)

## FE-KBD — Keyboard Navigation and Focus

### FE-KBD-001

> All interactive UI MUST be operable by keyboard alone. Every interactive element MUST be reachable by keyboard — via `Tab` for a standalone control, or via arrow keys inside a composite widget (menu, tablist, listbox, grid, tree) whose single tab stop is reachable via `Tab` — and operable via documented key bindings (`Enter`/`Space` for buttons, arrow keys within composite widgets per the WAI-ARIA Authoring Practices Guide). Mouse-only or pointer-only interactions MUST NOT exist, except where the underlying function depends on the path of the user's movement (freehand drawing, watercolor painting).

**Verification:** Any spec or plan that introduces interactive UI MUST commit to keyboard operability and name the key bindings for any non-default interaction (menus, dialogs, custom widgets). `/{project}:analyze` flags interactive-UI specs that describe drag-and-drop, hover-reveal, or pointer-only flows without a keyboard equivalent. **Not flagged:** an interaction whose underlying function, not its input technique, depends on the path of the user's movement, such as freehand drawing; handwriting that enters text, or a signature that only acknowledges something or confirms identity, is flagged without a keyboard alternative.

**Source:** WCAG 2.2 §2.1.1 (Keyboard), W3C Understanding 2.1.1 and 2.5.1, WAI-ARIA Authoring Practices Guide ("Developing a Keyboard Interface")

### FE-KBD-002

> Visible focus indication MUST be present on every interactive element when it receives keyboard focus. The default browser focus ring MUST NOT be removed (`outline: none`, `outline: 0`) without an author-styled replacement, and an author-styled focus indicator MUST have a contrast ratio of at least 3:1 against the colors adjacent to it.

**Verification:** Any spec or plan that introduces styling for interactive elements MUST commit to visible focus indication and, where focus styles are customized, to a 3:1 indicator contrast against adjacent colors. `/{project}:analyze` flags CSS in affected files that contains `outline: none` or `outline: 0` on focusable elements without an accompanying `:focus`/`:focus-visible` replacement, and flags documented focus-indicator colors below 3:1 against the surfaces they sit on. An indicator too small or too thin, or too little changed from the unfocused state, is `FE-KBD-005`'s.

**Source:** WCAG 2.2 §2.4.7 (Focus Visible), §1.4.11 (Non-text Contrast), W3C Understanding 1.4.11 ("Relationship with Focus Visible")

### FE-KBD-003

> Modal dialogs and disclosure UI that take over the page MUST trap focus while open (Tab and Shift+Tab cycle within the dialog) and MUST return focus to the triggering element on close. Pressing `Escape` MUST close the dialog. A native `<dialog>` opened with `showModal()`, or a page whose content outside the dialog is made `inert`, MAY satisfy the trap provided focus never reaches the page beneath, whether or not it can reach the browser's own controls. Where the triggering element no longer exists, or the dialog's task leads directly to a next step elsewhere in the workflow, focus MAY move on close to the element that continues that workflow instead.

**Verification:** Any spec or plan that introduces modal dialogs, drawers, or full-screen overlays MUST commit to focus-trap, focus-restoration, and Escape-key dismissal. `/{project}:analyze` flags modal-UI specs that omit any of the three.

**Source:** WAI-ARIA Authoring Practices Guide: Dialog (Modal) Pattern, WCAG 2.2 §2.1.2 (No Keyboard Trap), §2.4.3 (Focus Order), HTML Living Standard (the `dialog` element)

### FE-KBD-004

> When an element receives keyboard focus, it MUST NOT be entirely hidden by author-created content — sticky or fixed headers and footers, cookie and consent banners, chat launchers, toasts, or non-modal overlays. Content the user opened MAY cover the focused element when the user can reveal that element without advancing keyboard focus (for example, by dismissing the content with `Escape`).

**Verification:** Any spec or plan that introduces sticky or fixed-position UI, or persistent banners and launchers, MUST commit to how focused elements stay at least partially visible (e.g., `scroll-padding-top` / `scroll-margin` sized to the sticky region, banners that do not overlap page content). `/{project}:analyze` flags affected-files snippets that introduce `position: fixed` or `position: sticky` regions with no `scroll-padding` / `scroll-margin` offset, and specs that introduce persistent overlays without addressing focus visibility.

**Source:** WCAG 2.2 §2.4.11 (Focus Not Obscured (Minimum)), W3C Technique C43 (Using CSS scroll-padding to un-obscure content)

### FE-KBD-005

> Author-styled focus indicators SHOULD meet WCAG 2.2 §2.4.13 (Focus Appearance, Level AAA): an indicator area at least as large as a 2 CSS pixel thick perimeter of the unfocused component, with a contrast ratio of at least 3:1 between the same pixels in the focused and unfocused states.

**Verification:** Any spec or plan that introduces custom focus styles SHOULD commit to the §2.4.13 area and change-of-contrast bar. `/{project}:analyze` emits a warning when a custom focus-style spec names no indicator thickness, or names one thinner than 2 CSS pixels. This rule governs the indicator's size and its change from the unfocused state: a missing indicator, or one below 3:1 against adjacent colors, is `FE-KBD-002`'s.

**Source:** WCAG 2.2 §2.4.13 (Focus Appearance, Level AAA)

### FE-KBD-006

> Keyboard focus order MUST follow the visual and reading order of the page. Positive `tabindex` values (`tabindex="1"` and above) MUST NOT be used. When the focused element is removed or replaced — a client-side route change, a deleted list item, a closed panel — focus MUST be moved to a logical target (the new view's heading or main region, the adjacent item, the triggering control) rather than dropped to `<body>`.

**Verification:** Any spec or plan that introduces client-side routing, or UI that removes or replaces the focused element, MUST name the focus target after the change. `/{project}:analyze` flags affected-files snippets with a `tabindex` / `tabIndex` value greater than 0, flags affected-files snippets that reorder focusable content with CSS `order`, grid placement, or `row-reverse` so that the visual order diverges from DOM order, and flags client-side routing specs that omit post-navigation focus handling.

**Source:** WCAG 2.2 §2.4.3 (Focus Order), §1.3.2 (Meaningful Sequence), W3C Understanding 2.4.3 (Technique C27, Failure F44), WAI-ARIA Authoring Practices Guide ("Developing a Keyboard Interface")

### FE-KBD-007

> Additional content that pointer hover or keyboard focus makes visible — custom tooltips, hover cards, sub-menus, and other non-modal popups — MUST be dismissible without moving hover or focus (for example, with `Escape`), unless it communicates an input error or neither obscures nor replaces other content; MUST stay visible while the pointer moves from the trigger onto it, where hover can trigger it; and MUST remain visible until hover or focus leaves, the user dismisses it, or its information is no longer valid. Content whose presentation the browser controls and the author has not modified, such as the tooltip of the HTML `title` attribute, MAY behave as the browser provides.

**Verification:** Any spec or plan that introduces custom tooltips, hover cards, hover- or focus-triggered menus, or non-modal popovers MUST commit to dismissal without moving hover or focus, to keeping the content visible while it is hovered, and to persistence until the trigger loses hover or focus. `/{project}:analyze` flags affected-files snippets that show content on `mouseenter` or `mouseover` and hide it on the trigger's `mouseleave` without keeping it open while the pointer is over the content, that show content which obscures or replaces other content with no `Escape` (or equivalent) dismissal, or that hide hover- or focus-triggered content on a timer while the trigger still has hover or focus. **Not flagged:** content that communicates an input error and has no `Escape` (or equivalent) dismissal; the hover and timer flags still apply to it.

**Source:** WCAG 2.2 §1.4.13 (Content on Hover or Focus), W3C Understanding 1.4.13 (Technique SCR39, Failure F95)

### FE-KBD-008

> A keyboard shortcut implemented using only letter, punctuation, number, or symbol characters — a single key such as `k` or `?`, or a sequence such as `g` then `a` — MUST be one the user can turn off or remap to include a non-printable key (Ctrl, Alt), unless the shortcut is active only while the component it belongs to has focus.

**Verification:** Any spec or plan that introduces keyboard shortcuts MUST list them and, for each made only of character keys, name the setting that turns it off or remaps it, or state that it is active only while its component has focus. `/{project}:analyze` flags affected-files snippets with a `document`- or `window`-level `keydown`, `keypress`, or `keyup` listener that acts on a printable key without checking for a modifier (`ctrlKey`, `altKey`, `metaKey`) and with no setting that disables it.

**Source:** WCAG 2.2 §2.1.4 (Character Key Shortcuts), W3C Understanding 2.1.4 (Technique G217, Failure F99)

## FE-ARIA — ARIA Usage

### FE-ARIA-001

> ARIA roles, states, and properties MUST be used to fill gaps in native HTML semantics — not to override them. A native HTML element with the needed semantics and behavior built in MUST take precedence over an ARIA role: `role="button"` on a `<div>` is forbidden when a `<button>` would serve. ARIA roles MUST be added only when no native element exists for the pattern (tabs, comboboxes, treeviews, custom listboxes, etc.), only on a host element that ARIA in HTML allows the role on (`role="tab"` on a `<button>` is allowed; `role="heading"` on a `<button>` is not), and MUST NOT restate an element's implicit role (`<button role="button">`). A list whose markers are removed by styling MAY restate `role="list"`. ARIA states and properties MAY be added to native elements to expose state or relationships HTML cannot express (`aria-expanded`, `aria-current`, `aria-describedby`, `aria-invalid`).

**Verification:** Any spec or plan that introduces custom widgets MUST justify each ARIA role/state/property against the WAI-ARIA Authoring Practices Guide pattern it implements. `/{project}:analyze` flags affected-files snippets that restate an element's implicit role (`<button role="button">`, `<a href role="link">`) other than the Statement's styled-list case, that apply a role ARIA in HTML does not allow on its host element, that put an ARIA widget role on a `<div>`/`<span>` where a native element provides the pattern, other than a click-listening `<div>`/`<span>` that a native `<button>` or `<a href>` would serve, which `FE-SEMHTML-001` owns, or that apply `aria-hidden="true"` to focusable elements.

**Source:** WAI-ARIA 1.2 §1.4 (Co-Evolution of WAI-ARIA and Host Languages), W3C ARIA in HTML §1 (Author requirements for use of ARIA in HTML), §3.1 (Avoid overriding interactive elements with non-interactive roles), §3.2 (Avoid specifying redundant roles), W3C ACT rule 6cfa84 "Element with aria-hidden has no content in sequential focus navigation" (mapped to WCAG 2.2 §4.1.2), WAI-ARIA Authoring Practices Guide "Read Me First"

### FE-ARIA-002

> Every interactive element MUST have an accessible name. Where visible text serves as the name (button labels, link text), no `aria-label` is required. Where the element has no visible text (icon-only buttons, image links, decorative-text-only links), a meaningful name describing the action or destination MUST be provided — by native means where available (`alt` text on an image inside a link, visually hidden text, an SVG `<title>`) or by `aria-label` / `aria-labelledby`. Generic names (`"button"`, `"link"`, `"click here"`, `"read more"` without context) MUST NOT be used.

**Verification:** Any spec or plan that introduces icon-only buttons, image links, or non-text interactive elements MUST commit to an accessible-name strategy per element. `/{project}:analyze` flags affected-files snippets with a `<button>` containing only an `<svg>` or icon character and no name from any source (`aria-label`, `aria-labelledby`, visually hidden text, SVG `<title>`), flags `<a>` elements whose only content is an `<img>` with an empty or missing `alt`, and flags accessible-name values from a generic-name denylist.

**Source:** WCAG 2.2 §4.1.2 (Name, Role, Value), §2.4.4 (Link Purpose (In Context))

### FE-ARIA-003

> When an interactive element has a visible text label, its accessible name MUST contain that visible text, preferably at the start. An `aria-label` or `aria-labelledby` that replaces an element's visible text with different wording MUST NOT be used.

**Verification:** Any spec or plan that introduces `aria-label` or `aria-labelledby` on elements that also carry visible text MUST show that each such accessible name contains the element's visible text. `/{project}:analyze` flags affected-files snippets where an element with visible text content carries an `aria-label` that does not contain that text.

**Source:** WCAG 2.2 §2.5.3 (Label in Name)

### FE-ARIA-004

> Status messages that appear without moving focus — the result of an action (saved, submitted, added to cart), progress and completion of asynchronous work, search-result counts, toasts, and error summaries shown on submit — MUST be exposed through a live region (`role="status"`, `role="alert"`, `role="log"`, `<output>`, or `aria-live`) that is present before the message is added. A `role="alert"` container MAY be inserted together with its message.

**Verification:** Any spec or plan that introduces asynchronous feedback, toasts, inline save states, or result counts MUST name the live-region role per message type (`status` for advisory messages, `alert` for urgent errors). `/{project}:analyze` flags async-feedback specs with no live-region commitment, and flags affected-files snippets that inject toast or status text into a container lacking a live-region role or `aria-live`, or that insert a live-region container other than `role="alert"` together with its message rather than rendering it before the message is added.

**Source:** WCAG 2.2 §4.1.3 (Status Messages), W3C Understanding 4.1.3 (status message examples), WAI-ARIA 1.2 (`status`, `alert`, and `log` roles; `aria-live`), W3C ARIA in HTML (`output` has implicit role `status`), MDN "ARIA live regions"

## FE-CONTRAST — Color and Contrast

### FE-CONTRAST-001

> Text and functional graphics MUST meet the WCAG 2.2 AA contrast ratios against adjacent colors: 4.5:1 for normal text; 3:1 for large text (≥18pt, or ≥14pt bold — W3C's Understanding 1.4.3 gives these as approximately 24 and 18.5 CSS pixels); and 3:1 for the visual information needed to identify UI components and their states and for graphics needed to understand content (form-field borders, focus indicators, status indicators, icons that convey information). Inactive (disabled) components, purely decorative text and graphics, logotypes, and components whose appearance the browser determines and the author has not modified MAY fall below these ratios.

**Verification:** Any spec or plan that introduces a color palette, theme, or text-on-color UI element MUST commit to AA contrast across all foreground/background pairs in use. `/{project}:analyze` runs against the project's documented palette and flags pair combinations below threshold, and flags foreground/background pairs hard-coded in affected-files snippets that fall below threshold. **Not flagged:** a pair the Statement's MAY covers. A documented focus-indicator color below 3:1 against adjacent colors is `FE-KBD-002`'s.

**Source:** WCAG 2.2 §1.4.3 (Contrast (Minimum)), §1.4.11 (Non-text Contrast), W3C Understanding 1.4.3

### FE-CONTRAST-002

> Information MUST NOT be conveyed by color alone. Status, validation state, required-field marking, and selection state MUST be reinforced by a non-color cue: icon, text label, underline, weight, pattern, or position.

**Verification:** Any spec or plan that introduces state indicators (errors, warnings, success, selection, required) MUST commit to a non-color cue per state. `/{project}:analyze` flags spec descriptions that describe state via color only ("the field turns red"), and flags affected-files snippets that style validation state without an accompanying icon or text node.

**Source:** WCAG 2.2 §1.4.1 (Use of Color)

## FE-A11YFORM — Accessible Forms

### FE-A11YFORM-001

> Every form input MUST have a programmatically associated label. The label MUST be one of: a `<label>` element with `for` matching the input's `id`, an enclosing `<label>` wrapping the input, or an `aria-labelledby` reference to a visible text element. Placeholder text MUST NOT substitute for a label. A field whose purpose is conveyed by an adjacent visible button (a search box beside a "Search" button) MAY be named with `aria-label` instead.

**Verification:** Any spec or plan that introduces a form MUST commit to programmatic labels per input. `/{project}:analyze` flags affected-files snippets with an `<input>` (other than `type="hidden"`, `submit`, `reset`, `button`, or `image`), `<select>`, or `<textarea>` that has no associated `<label>` and no `aria-labelledby` reference — including fields identified only by a `placeholder`, a `title`, or an `aria-label` outside the adjacent-button case.

**Source:** WCAG 2.2 §1.3.1 (Info and Relationships), §3.3.2 (Labels or Instructions), §4.1.2 (Name, Role, Value), W3C Technique G167 (Using an adjacent button to label the purpose of a field), HTML Living Standard (the `placeholder` attribute; the `label` element)

### FE-A11YFORM-002

> Form validation errors MUST be associated with their inputs via `aria-describedby` pointing at the error message element, AND inputs in an error state MUST set `aria-invalid="true"`. Error messages MUST be visible (not screen-reader-only) and MUST identify the field by name in their text. Where the expected format or allowed values are known, the error message MUST state how to correct the entry, unless doing so would compromise security (e.g., revealing which credential was wrong).

**Verification:** Any spec or plan that introduces form validation UI MUST commit to `aria-describedby` + `aria-invalid` per error-bearing input and to correction text for format-constrained fields. `/{project}:analyze` flags form-validation specs that omit either ARIA attribute, that hide errors visually (a visually-hidden or `sr-only` class, or `display: none` / `visibility: hidden` on an element still referenced by `aria-describedby`) while revealing them only to screen readers, that surface errors without naming the offending field, or that reject a format-constrained field without saying what format is expected. An error summary shown on submit without moving focus is also a status message, and its missing live region is `FE-ARIA-004`'s.

**Source:** WCAG 2.2 §3.3.1 (Error Identification), §3.3.3 (Error Suggestion), §4.1.2 (Name, Role, Value), W3C Technique ARIA21 (Using aria-invalid to Indicate An Error Field), WAI-ARIA 1.2 (`aria-invalid`)

### FE-A11YFORM-003

> Inputs that collect information about the user MUST carry the HTML `autocomplete` token matching their purpose (`name`, `given-name`, `email`, `tel`, `street-address`, `postal-code`, `country`, `bday`, `username`, and the rest of WCAG 2.2's Input Purposes list). Credential, one-time-code, and payment fields carry the same obligation under `FE-PII-002`.

**Verification:** Any spec or plan that introduces a form collecting personal data (identity, contact, address, birth date) MUST name the `autocomplete` token per field. `/{project}:analyze` flags affected-files snippets with inputs whose `type`, `name`, or `id` indicates personal data (`email`, `tel`, `name`, `address`, `postal`/`zip`) but that carry no `autocomplete` attribute or `autocomplete="off"`.

**Source:** WCAG 2.2 §1.3.5 (Identify Input Purpose), WCAG 2.2 §7 (Input Purposes for User Interface Components), W3C Understanding 1.3.5, HTML Living Standard (Autofill, the `autocomplete` attribute)

### FE-A11YFORM-004

> Authentication flows (sign-in, sign-up, multi-factor, account recovery) MUST NOT require a cognitive function test at any step — remembering a password, transcribing a code, solving a puzzle, performing a calculation — unless that step also offers an alternative method without one or a mechanism that assists the user with it. Password and one-time-code fields MUST permit paste and password-manager autofill. A CAPTCHA MUST NOT be the only path through a step when it requires text transcription or puzzle solving; object-recognition and personal-content checks MAY be used.

**Verification:** Any spec or plan that introduces sign-in, sign-up, multi-factor, account recovery, or a CAPTCHA MUST commit to paste and autofill support on credential and code fields and name the non-cognitive alternative for any puzzle or transcription step. `/{project}:analyze` flags affected-files snippets that block paste on credential or code inputs (`onpaste` handlers that call `preventDefault`), split one-time codes across per-digit inputs that do not accept a pasted code, and flags authentication specs whose only bot check is a text-transcription or puzzle CAPTCHA.

**Source:** WCAG 2.2 §3.3.8 (Accessible Authentication (Minimum)), W3C Understanding 3.3.8, `FE-PII-002`

### FE-A11YFORM-005

> Within a multi-step process, information the user has already entered MUST NOT be requested again unless it is auto-populated or offered for selection. Re-entry MAY be required when it is essential to the activity, when it is needed for security, or when the earlier value is no longer valid.

**Verification:** Any spec or plan that introduces a multi-step flow (checkout, onboarding, an application wizard) MUST identify each data point requested in more than one step and commit to auto-population or selection for it. `/{project}:analyze` flags multi-step-flow specs that request the same data point in two steps without naming the reuse mechanism.

**Source:** WCAG 2.2 §3.3.7 (Redundant Entry)

### FE-A11YFORM-006

> A component receiving focus MUST NOT by itself initiate a change of context — submitting a form, opening a new window, moving focus to another component, or navigating to a new page. Changing the setting of a control — selecting an option, checking a box, choosing a radio button, entering text — MUST NOT automatically cause a change of context unless the user is advised of the behavior before using the control.

**Verification:** Any spec or plan that introduces a control whose focus or change triggers navigation, form submission, a new window, or a focus move MUST name the explicit activation (a submit or go button) that performs it instead, or the advance notice given before the control is used. `/{project}:analyze` flags affected-files snippets with a `focus` handler, or a `change` or `input` handler on a `<select>`, checkbox, radio button, or text field, that navigates (`location` assignment, router navigation), submits the form, calls `window.open`, or moves focus elsewhere.

**Source:** WCAG 2.2 §3.2.1 (On Focus), §3.2.2 (On Input), WCAG 2.2 definition of "changes of context", W3C Understanding 3.2.1 and 3.2.2 (Techniques G80, H84, G13; Failures F36, F37)

### FE-A11YFORM-007

> A submission that causes a legal commitment or a financial transaction for the user, that modifies or deletes user-controllable data in storage, or that submits user test responses MUST be reversible, MUST check the entered data for input errors and give the user a chance to correct them, or MUST let the user review, confirm, and correct the information before it is finalized — at least one of the three. A save command, and the simple creation or editing of a document or record, MAY proceed without any of them.

**Verification:** Any spec or plan that introduces a purchase, payment, or other financial transaction, acceptance of a contract or other legal commitment, deletion of user-controllable data in storage (an account, a file, a record, or a bulk deletion), a modification of it other than a save command or the simple creation or editing of a document or record (such as a change to the contact details of the user's own account, or a bulk change), or submission of test answers MUST name which of the three safeguards it provides. `/{project}:analyze` flags such specs that finalize the action on a single activation with no undo, no validation with an opportunity to correct, and no review or confirmation step.

**Source:** WCAG 2.2 §3.3.4 (Error Prevention (Legal, Financial, Data)), W3C Understanding 3.3.4 (Techniques G98, G99, G164, G168)

## FE-A11YMEDIA — Text Alternatives and Media

### FE-A11YMEDIA-001

> Every `<img>` element MUST have an `alt` attribute. Informational images MUST have descriptive alt text; purely decorative images MUST have `alt=""` (empty string). SVG icons used as interactive triggers MUST give the control an accessible name by one of the means `FE-ARIA-002` lists.

**Verification:** Any spec or plan that introduces image rendering MUST commit to an alt-text policy. `/{project}:analyze` flags affected-files snippets with `<img>` elements lacking an `alt` attribute; an unnamed SVG-only control is flagged under `FE-ARIA-002`.

**Source:** WCAG 2.2 §1.1.1 (Non-text Content), W3C Technique H67 (Using null alt text and no title attribute on img elements for images that assistive technology should ignore), WebAIM "Alternative Text"

### FE-A11YMEDIA-002

> Prerecorded video with audio MUST provide synchronized captions; prerecorded audio-only content MUST provide a transcript; prerecorded video-only content MUST provide a text alternative or an audio track that conveys the same information. Captions and transcripts for content authored by the application MUST be human-edited (not solely auto-generated). Audio or video that is a media alternative for text and is clearly labeled as such MAY omit these alternatives.

**Verification:** Any spec or plan that introduces prerecorded video or audio media, other than a clearly labeled media alternative for text, MUST commit to captions/transcripts and to the editorial pipeline that produces them. `/{project}:analyze` flags specs introducing prerecorded media that omit caption/transcript commitments or that rely solely on auto-generation for application-authored content. **Not flagged:** audio or video that is a media alternative for text and is clearly labeled as such. Live video, and audio description of prerecorded synchronized media, are `FE-A11YMEDIA-003`'s.

**Source:** WCAG 2.2 §1.2.1 (Audio-only and Video-only (Prerecorded)), §1.2.2 (Captions (Prerecorded)), W3C WAI "Captions/Subtitles" ("Automatic Captions are Not Sufficient")

### FE-A11YMEDIA-003

> All prerecorded video content in synchronized media that the application's pages present — whether the application produces it, embeds it from another source, or accepts it from users — MUST have audio description of the important visual details that cannot be understood from the main soundtrack alone. All live audio content in synchronized media those pages present MUST have captions; a two-way multimedia call between individuals made through the application MAY be left without application-provided captions. A video showing only a speaker MAY meet the audio-description obligation with a static text alternative that describes the speaker's visual details (W3C technique G203). Media that is a media alternative for text and is clearly labeled as such is not synchronized media under WCAG 2.2's definition and is outside both obligations; where the existing audio already provides all of the video information, no additional audio description is necessary.

**Verification:** Any spec or plan that introduces pages presenting prerecorded video in synchronized media MUST commit to an audio-description approach for it or record which of the Statement's audio-description exceptions applies. Any spec or plan that introduces pages presenting live synchronized media broadcast to viewers, such as a live stream or web cast, MUST commit to a real-time captioning mechanism. Where media under either commitment comes from a source outside the application's control, the spec or plan MUST state how the obligation is met, such as requiring the contributor to supply it or monitoring and repairing within two business days as §5.4 describes; a statement of partial conformance records that the obligation is not met and does not meet it. `/{project}:analyze` flags media-introducing specs that omit the applicable commitment, including specs that commit to it only for the application's own media.

**Source:** WCAG 2.2 §1.2.5 (Audio Description (Prerecorded)), §1.2.3 (Audio Description or Media Alternative (Prerecorded)), §1.2.4 (Captions (Live)), the WCAG 2.2 definitions of synchronized media, audio description (Note 3), and web page (Example 1), §5.2.2 (Full pages), §5.4 (Statement of Partial Conformance - Third Party Content), W3C Understanding 1.2.5 (sufficient techniques G78, G173, G226, and G203), W3C Understanding 1.2.4, W3C Understanding Conformance ("Partial conformance claims due to third party content")

## FE-POINTER — Pointer Input and Target Size

### FE-POINTER-001

> Pointer targets MUST be at least 24×24 CSS pixels, or be spaced so that a 24 CSS pixel diameter circle centered on the bounding box of each undersized target intersects no other target and no other undersized target's circle. Targets in a sentence or whose size is otherwise constrained by the line-height of non-target text, targets whose size the browser determines and the author has not modified, targets whose function another control on the same page that meets this size also performs, and targets whose particular presentation is essential or legally required MAY be smaller.

**Verification:** Any spec or plan that introduces interactive controls (icon buttons, toolbars, pagination, close buttons, chips, table-row actions) MUST commit to a minimum target size of 24×24 CSS pixels or to the spacing alternative. `/{project}:analyze` flags affected-files snippets that size interactive elements below 24 CSS pixels in width or height — including icon buttons whose only box is a sub-24px icon with no padding — without spacing that satisfies the 24-pixel-circle test.

**Source:** WCAG 2.2 §2.5.8 (Target Size (Minimum)), W3C Understanding 2.5.8

### FE-POINTER-002

> Functionality operated by a dragging movement (drag-and-drop reordering, sliders, map panning, resizable panes) MUST also be operable by a single pointer without dragging — taps or clicks on controls such as move-up/move-down buttons, a destination picker, or a position on a slider track. Functionality operated by multipoint or path-based gestures (pinch, two-finger swipe, swipe paths) MUST also be operable by a single pointer without a path-based gesture. Dragging or a path-based gesture MAY be the only method where the movement itself is essential (freehand drawing, or a signature whose drawn mark is itself the function, not one that only acknowledges something or confirms identity), and dragging MAY also be where the browser determines the functionality and the author has not modified it.

**Verification:** Any spec or plan that introduces drag-and-drop, sliders, swipe, pinch, or drawn-path input such as a signature pad MUST name the single-pointer, non-dragging alternative for each one the Statement's MAY does not cover; the keyboard equivalent required by `FE-KBD-001` satisfies this rule only where it also provides controls a pointer can click or tap. `/{project}:analyze` flags interaction specs that describe drag, swipe, pinch, or drawn-path operations without a click or tap alternative, and flags affected-files snippets that bind drag or multi-touch handlers (`draggable`, `dragstart`, pointer-move drag or drawing logic, multi-touch checks) with no control that performs the same action. **Not flagged:** dragging or a path-based gesture the Statement's MAY covers, such as freehand drawing or a signature whose drawn mark is itself the function.

**Source:** WCAG 2.2 §2.5.7 (Dragging Movements), §2.5.1 (Pointer Gestures), W3C Understanding 2.5.7 and 2.5.1

### FE-POINTER-003

> Functionality operated by a single pointer MUST NOT execute any part of its function on the pointer's down-event (`pointerdown`, `mousedown`, `touchstart`); it completes on the up-event, as a `click` handler does. A down-event MAY begin a function that completes on the up-event when the user can abort it before completion or undo it after (a drag the user can drop back where it started), MAY trigger an outcome the up-event reverses (a preview shown only while pressed), and MAY execute the function where completing it on the down-event is essential (an on-screen keyboard or keypad).

**Verification:** Any spec or plan that introduces custom pointer handling — press-and-hold, drag, or controls built on pointer events rather than `click` — MUST name the event that completes each action. `/{project}:analyze` flags affected-files snippets that navigate, submit, delete, or change state inside a `pointerdown`, `mousedown`, or `touchstart` handler, unless the handler only begins a drag or press that the up-event completes, cancels, or reverses.

**Source:** WCAG 2.2 §2.5.2 (Pointer Cancellation), W3C Understanding 2.5.2 (Techniques G210, G212; Failure F101)

## FE-LAYOUT — Reflow, Zoom, Text Spacing, and Orientation

### FE-LAYOUT-001

> Content MUST remain usable, without loss of information or functionality, when the user enlarges it: pages MUST reflow to a viewport 320 CSS pixels wide without horizontal scrolling (content that needs two-dimensional layout for its meaning — data tables, maps, diagrams, video — excepted), and text other than captions and images of text MUST resize to 200% through browser zoom or text-size settings without assistive technology. The viewport meta tag MUST NOT disable or cap zoom (`user-scalable=no`, or a `maximum-scale` below 2).

**Verification:** Any spec or plan that introduces a page layout, a responsive breakpoint set, or the viewport meta tag MUST commit to reflow at 320 CSS pixels and to 200% text resize without loss. `/{project}:analyze` flags affected-files snippets whose viewport meta tag contains `user-scalable=no` or a `maximum-scale` below 2, and flags layout specs that set fixed widths above 320 CSS pixels on page-level containers without a narrower breakpoint.

**Source:** WCAG 2.2 §1.4.10 (Reflow), §1.4.4 (Resize Text), W3C ACT rule b4f0c3 "Meta viewport allows for zoom"

### FE-LAYOUT-002

> Text containers MUST NOT clip, truncate, or overlap content when the user overrides text spacing to a line height of 1.5×, paragraph spacing of 2×, letter spacing of 0.12×, and word spacing of 0.16× the font size. Content MUST NOT restrict its display to a single orientation (portrait or landscape) unless a specific orientation is essential (a bank-check capture, a piano keyboard).

**Verification:** Any spec or plan that introduces text containers with fixed dimensions, or that constrains orientation, MUST commit to tolerating spacing overrides and to no orientation lock without an essential reason. `/{project}:analyze` flags affected-files snippets that set a fixed pixel `height` on text containers together with `overflow: hidden`, that call `screen.orientation.lock()`, or that hide or rotate content under an `orientation` media query to force one orientation.

**Source:** WCAG 2.2 §1.4.12 (Text Spacing), §1.3.4 (Orientation), W3C Understanding 1.4.12 and 1.3.4

## FE-MOTION — Motion, Flashing, and Auto-Playing Media

### FE-MOTION-001

> Content that moves, blinks, or scrolls automatically for more than five seconds alongside other content (carousels, marquees, animated banners, background video) MUST provide a control to pause, stop, or hide it, and content that updates automatically (tickers, live feeds) MUST provide a control to pause, stop, hide it, or set its update frequency. Audio that plays automatically for more than three seconds MUST provide a control to pause or stop it, or to set its volume independently of the system volume. Movement or updating that is essential to the activity MAY omit the control, such as a loading indicator shown while no interaction is possible.

**Verification:** Any spec or plan that introduces carousels, auto-advancing slides, animated or auto-updating regions, background video, or auto-playing audio MUST name the pause, stop, or hide control. `/{project}:analyze` flags affected-files snippets with `autoplay` on `<audio>`, or on `<video>` without `muted`, that render no controls, and flags auto-advancing carousel or ticker specs that omit a pause control.

**Source:** WCAG 2.2 §2.2.2 (Pause, Stop, Hide), §1.4.2 (Audio Control), W3C Understanding 2.2.2 and 1.4.2

### FE-MOTION-002

> Content MUST NOT flash more than three times in any one-second period, unless the flashing stays below the general flash and red flash thresholds.

**Verification:** Any spec or plan that introduces animation, video, or visual effects with rapid luminance changes (strobing, flashing alerts, glitch effects) MUST commit to the three-flashes limit. `/{project}:analyze` flags specs that describe flashing or strobing effects without a flash-rate commitment, and flags affected-files snippets whose CSS animations or timers complete a full on-and-off cycle of visibility, opacity, or high-contrast colors in less than 333 ms.

**Source:** WCAG 2.2 §2.3.1 (Three Flashes or Below Threshold), WCAG 2.2 definitions of "flash" and "general flash and red flash thresholds", W3C Understanding 2.3.1

### FE-MOTION-003

> Non-essential motion triggered by interaction — parallax, zoom and slide transitions, scroll-linked animation, animated page transitions — SHOULD be removed or reduced when the user has set the `prefers-reduced-motion: reduce` media feature.

**Verification:** Any spec or plan that introduces animated transitions, parallax, or scroll-linked effects SHOULD commit to a reduced-motion variant. `/{project}:analyze` emits a warning when affected files define animations or transitions of `transform` or position, or use a JavaScript animation library, with no `prefers-reduced-motion` media query or `matchMedia` check governing them.

**Source:** WCAG 2.2 §2.3.3 (Animation from Interactions, Level AAA), W3C Understanding 2.3.3 (Techniques C39 and SCR40), W3C Media Queries Level 5 (`prefers-reduced-motion`)

## FE-TIMING — Time Limits

### FE-TIMING-001

> Every time limit the application sets (session inactivity timeout, timed form, held reservation) MUST let the user turn it off, adjust it to at least ten times its default length, or extend it — warning the user before it expires, giving at least 20 seconds to extend with a simple action, and allowing at least ten extensions. A limit that is a required part of a real-time event (an auction) where no alternative to the limit is possible, a limit that is essential and that extending would invalidate the activity (a time-limited one-time code), and a limit longer than 20 hours MAY be exempt.

**Verification:** Any spec or plan that introduces a session timeout, timed task, or held reservation MUST commit, for each limit, to one of turn-off, adjust, or warn-and-extend — naming, for warn-and-extend, the warning lead time — or name the Statement's exemption that applies to it. `/{project}:analyze` flags timeout-introducing specs that commit to none of the three and claim no exemption, or whose warn-and-extend allows less than 20 seconds to respond. A limit set for security is still one the application sets: the idle timeout `BE-AUTHN-005` requires is in scope, and so is an absolute session lifetime of 20 hours or less unless it is essential in the Statement's sense. This rule governs the warning and the chance to continue before a limit expires: a session lacking an idle or absolute timeout, or enforcing one only client-side, is `BE-AUTHN-005`'s, and a session expiry or revocation that fails silently or leaves the UI broken is `FE-AUTHN-002`'s.

**Source:** WCAG 2.2 §2.2.1 (Timing Adjustable), W3C Understanding 2.2.1

## FE-A11YTEST — Accessibility Testing

### FE-A11YTEST-001

> A project with a web frontend MUST check its rendered pages or components with an automated accessibility rule engine (axe-core, Pa11y, Lighthouse's accessibility audit, or an equivalent WCAG-mapped checker) before each release, and a release MUST NOT ship with a new violation at or above the project's documented severity threshold.

**Verification:** `specs/system.md` or the spec that introduces the frontend MUST name the automated accessibility tool, the pages or components it covers, when it runs before release, and the release-blocking severity threshold; any spec or plan that adds a page, route, or shared component MUST extend that coverage. `/{project}:analyze` flags frontend projects with no automated accessibility-check commitment, and UI-introducing specs whose plan adds no coverage for the new surface. **Not flagged:** the engine running wherever the project chooses — CI, a pre-commit hook, a component test suite, or a release script; a check that does not also run on every change, which `FE-A11YTEST-003` owns; and the absence of manual keyboard and screen-reader testing, which `FE-A11YTEST-002` owns.

**Source:** W3C WAI "Selecting Web Accessibility Evaluation Tools", Deque axe-core rule documentation

### FE-A11YTEST-002

> New custom widgets, page templates, and interaction patterns SHOULD be tested manually before release with the keyboard alone and with at least one screen reader and browser pairing (NVDA or JAWS with Chrome or Firefox on Windows, VoiceOver with Safari on macOS or iOS, TalkBack with Chrome on Android).

**Verification:** Any spec or plan that introduces a custom widget, a new page template, or a novel interaction pattern SHOULD include a manual keyboard and screen-reader test step naming the screen-reader and browser pairing. `/{project}:analyze` emits a warning when such a spec's plan or tasks contain no manual assistive-technology test step.

**Source:** W3C WCAG Evaluation Methodology (WCAG-EM) 2.0 (W3C Group Note, 23 July 2026; Methodology Requirement 1.3), W3C WAI "Evaluating Web Accessibility Overview"

### FE-A11YTEST-003

> The automated accessibility check of `FE-A11YTEST-001` SHOULD also run automatically on every change to the pages or components it covers, failing the change on a new violation at or above the project's documented severity threshold.

**Verification:** Any spec or plan covering frontend build, test, or CI configuration SHOULD include the automated accessibility check on each change. `/{project}:analyze` emits a warning when such a spec omits it. **Not flagged:** a per-change check run by any mechanism, such as CI, a pre-commit hook, or a component test suite.

**Source:** W3C WAI "Selecting Web Accessibility Evaluation Tools", Deque axe-core rule documentation
