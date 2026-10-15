//! Procedure walker and JSON-over-stdio protocol I/O.
//!
//! [`Walker`] is the synchronous engine that consumes a parsed
//! [`Procedure`] step by step. For each step it either:
//!
//! - Dispatches to a primitive's pure-Rust function, merges its structured
//!   result into the walker context (so a later step's payload builder or a
//!   later primitive can read prior results), and emits a `progress`
//!   envelope (`Step::Primitive`). See [`Walker::merge_primitive_result`]
//!   for the merge policy.
//! - Emits an `llm-request` envelope and reads a matching
//!   `llm-response` from stdin (`Step::Extension`). An `assessSpecQuality`
//!   step emits one per loaded rule of its tier, each carrying one rule
//!   (spec 060), and an `askClarifyQuestion` step one per open question;
//!   every other extension step emits exactly one.
//! - Blocks on a confirmation gate: emits a `gate-confirm` envelope and
//!   reads a `gate-response` back. A denied gate is a clean `complete`
//!   (per §partial-failure-semantics), never an error.
//! - Otherwise no-op (`Step::Prose`).
//!
//! # Gate convention
//!
//! Two step shapes gate, and step type decides which rule applies:
//!
//! - A `Step::Primitive` whose name is `gate-confirm` IS a blocking gate
//!   by virtue of the primitive — phrase or no phrase (prune.md step 6's
//!   shape). The walker emits the `gate-confirm` envelope itself and
//!   awaits the `gate-response`; it does not dispatch through
//!   [`dispatch_primitive`].
//! - A `Step::Prose` whose text contains the phrase "ask the user to
//!   approve" (case-insensitive, [`GATE_TRIGGER`]) is a fallback gate for
//!   procedures that gate without the primitive (plan.md / specify.md's
//!   shape).
//!
//! Dispatch wins over the phrase: a non-gate `Step::Primitive` or a
//! `Step::Extension` whose prose happens to contain the phrase dispatches
//! normally — a step is never silently converted into a gate that drops
//! its primitive or extension dispatch.
//!
//! A confirmed gate binds `apply: true` into the arguments of every primitive
//! dispatched after it, because a gate is what authorizes the writes that
//! follow. A primitive with an `apply` argument previews before the gate and
//! writes after it, so the step after prune.md's gate performs the reduction
//! the operator confirmed. The binding is an argument, never a context key,
//! so no extension payload carries it. A denied gate ends the walk.
//!
//! At the end of the procedure the walker emits `complete`. Operational
//! errors halt the walk and emit an `error` envelope before returning.
//! Step ordering and message emission are deterministic given the same
//! procedure + inputs. While suspended awaiting an `llm-response` or
//! `gate-response`, any other inbound line — a wrong-type envelope, a
//! response with a mismatched request-id, malformed JSON, or a blank
//! keepalive — is logged to stderr and skipped, and the walker keeps
//! waiting (data-model §JSON-over-stdio ignore-and-continue rule). Only
//! stdin EOF while awaiting a response is an operational error.

#![allow(clippy::module_name_repetitions)]

mod analyze_tally;
pub mod payload;

use std::collections::HashSet;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use crate::io::{read_envelope, write_envelope};
use crate::primitives;
use crate::schema::extensions::{
    self, AssessSpecQualityRule, PerformReviewResponse, ValidationError, WriteCodeResponse,
};
use crate::schema::primitives::{
    AppendInboxArgs, AppendQuestionArgs, AppendTaskArgs, ApplyManifestArgs, CheckArtifactSizeArgs,
    CheckArtifactsArgs, CheckCommandFlagsArgs, CheckCorpusLinksArgs, CheckOrphanedReferencesArgs,
    CheckPromotionCoverageArgs, CheckReviewGateArgs, CheckRuleIdsArgs, CheckStepReferencesArgs,
    CheckStuckArgs, CheckUnfoldedSpecsArgs, ComputeReviewScopeArgs, CreateFeatureArgs,
    CreatePlanArtifactsArgs, CreateScenarioArgs, DashboardArgs, DeriveBoundaryArgs,
    DeriveDependenciesArgs, DeriveReferencesArgs, DeriveRoutingCandidatesArgs, DiffCrossSpecArgs,
    DiscoverRuleFilesArgs, EnforceManifestArgs, ExtractArchiveArgs, FetchArchiveArgs,
    InvalidateReviewArgs, LabelCriteriaArgs, LintMarkdownArgs, MarkCriterionArgs, MarkTaskArgs,
    MergeManagedBlockArgs, MergePermissionsArgs, MigrateSessionFileArgs, ProcessDecisionsArgs,
    ProcessWaiversArgs, PrunePlanArgs, PruneTasksArgs, ReadSpecArgs, ReadTasksArgs,
    RelocateAuditRecordsArgs, RemoveInboxItemArgs, ResolveAnchorArgs, ResolveConstitutionsArgs,
    ResolveFeatureArgs, ResolveReferencesArgs, ResolveSessionArgs, RetargetSessionsArgs,
    RetireFeatureArgs, RewriteSpecLinksArgs, RunGeneratorArgs, SetStatusArgs, TraverseDepsArgs,
    ValidateFrontmatterArgs, WriteAnalysisArgs, WriteReviewArgs, WriteSessionArgs,
};
use crate::schema::procedure::{Procedure, Step, StepNumber};
use crate::schema::protocol::{ErrorLocation, ProtocolMessage};
use crate::schema::severity::RuleSeverity;

const GATE_TRIGGER: &str = "ask the user to approve";

/// The extension point `/analyze` steps 11 and 12 ask per rule through.
const ASSESS_SPEC_QUALITY: &str = "assessSpecQuality";

/// The extension point `/clarify` step 6 asks per open question through.
const ASK_CLARIFY_QUESTION: &str = "askClarifyQuestion";

/// One run of the walker. The caller owns the procedure, repo path, and
/// reader/writer streams; the walker borrows them for its lifetime.
pub struct Walker<'a, R: BufRead, W: Write> {
    procedure: &'a Procedure,
    repo: PathBuf,
    context: Map<String, Value>,
    /// Keys present in `context` at construction — the session-seeded
    /// bindings (e.g. `feature`, `write-boundary`). A primitive result may
    /// never overwrite one of these; see [`Walker::merge_primitive_result`].
    seeded_keys: HashSet<String>,
    reader: &'a mut R,
    writer: &'a mut W,
    request_counter: u64,
    /// The tier counts an `/analyze` walk's detection steps produce, bound to
    /// its `write-analysis` step; `None` for every other command.
    analyze_tally: Option<analyze_tally::AnalyzeTally>,
    /// The rules `assessSpecQuality` steps ask about, loaded from
    /// `rule-files` at the walk's first such step; `None` until then.
    rules: Option<payload::LoadedRules>,
    /// Whether a gate has been confirmed. Every primitive dispatched after it
    /// is bound `apply: true` — in its arguments, never the context, so an
    /// extension payload built from the context does not carry it.
    gate_confirmed: bool,
}

/// Top-level outcome of [`Walker::run`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WalkOutcome {
    /// Walker emitted `complete` and exited cleanly.
    Complete,
    /// Walker emitted `error` and halted.
    Errored {
        /// Machine-readable error code.
        code: String,
        /// Human-readable description.
        message: String,
    },
}

impl<'a, R: BufRead, W: Write> Walker<'a, R, W> {
    /// Build a walker against `procedure`, rooted at `repo`. `context`
    /// carries CLI-supplied bindings (e.g., `feature`) that primitives
    /// deserialize their args from. An `/analyze` walk also binds the list
    /// seeds a session cannot carry (see [`derive_analyze_seeds`]).
    pub fn new(
        procedure: &'a Procedure,
        repo: PathBuf,
        mut context: Map<String, Value>,
        reader: &'a mut R,
        writer: &'a mut W,
    ) -> Self {
        let analyze = procedure.command == "analyze";
        if analyze {
            derive_analyze_seeds(&mut context, &repo);
        }
        let seeded_keys = context.keys().cloned().collect();
        Self {
            procedure,
            repo,
            context,
            seeded_keys,
            reader,
            writer,
            request_counter: 0,
            analyze_tally: analyze.then(analyze_tally::AnalyzeTally::new),
            rules: None,
            gate_confirmed: false,
        }
    }

    /// Walk the procedure to completion. Emits envelopes as a side effect.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if writing to `writer` fails or reading from
    /// `reader` fails. Operational errors from primitives are surfaced as
    /// `error` envelopes (not propagated as I/O errors); the walker
    /// returns `Ok(WalkOutcome::Errored)` in that case.
    pub fn run(&mut self) -> std::io::Result<WalkOutcome> {
        let steps = self.procedure.steps.clone();
        for step in &steps {
            if let Some(outcome) = self.handle_step(step)? {
                return Ok(outcome);
            }
        }
        self.emit_complete()?;
        Ok(WalkOutcome::Complete)
    }

    fn handle_step(&mut self, step: &Step) -> std::io::Result<Option<WalkOutcome>> {
        // Gate convention (see the module docs): a `gate-confirm` primitive
        // step blocks by virtue of the primitive; the prose phrase trigger
        // is a fallback for steps with no dispatch. Primitive/extension
        // dispatch wins over the phrase, so a dispatching step is never
        // silently converted into a gate.
        match step {
            Step::Primitive {
                number,
                name,
                prose,
                location,
            } => {
                if name == "gate-confirm" {
                    return self.handle_gate(number, prose);
                }
                self.handle_primitive(number, name, *location)
            }
            Step::Extension {
                number,
                identifier,
                prose,
                location: _,
            } => {
                let outcome = self.handle_extension(number, identifier, prose)?;
                Ok(outcome)
            }
            Step::Prose { number, text, .. } => {
                if text.to_lowercase().contains(GATE_TRIGGER) {
                    return self.handle_gate(number, text);
                }
                Ok(None)
            }
        }
    }

    fn handle_primitive(
        &mut self,
        number: &StepNumber,
        name: &str,
        location: crate::schema::procedure::SourceRange,
    ) -> std::io::Result<Option<WalkOutcome>> {
        let step_label = format_step_number(number);
        self.emit_progress(
            format!("dispatching primitive `{name}`"),
            Some(step_label.clone()),
            Some(name.into()),
        )?;
        // An `/analyze` walk's record carries the tier counts its own
        // detection steps produced (spec 058, AC26); the walk has no host to
        // supply them.
        let tally = match (&self.analyze_tally, name) {
            (Some(tally), "write-analysis") => Some(tally),
            _ => None,
        };
        let dispatched = if tally.is_some() || self.gate_confirmed {
            let mut bindings = self.context.clone();
            if let Some(tally) = tally {
                tally.bind(&mut bindings);
            }
            if self.gate_confirmed {
                bindings.insert("apply".into(), Value::Bool(true));
            }
            dispatch_primitive(name, &bindings, &self.repo)
        } else {
            dispatch_primitive(name, &self.context, &self.repo)
        };
        match dispatched {
            Ok(result) => {
                if let Some(tally) = &mut self.analyze_tally {
                    tally.record_primitive(name, &result, &self.context);
                }
                for line in session_report_lines(name, &result) {
                    self.emit_progress(line, Some(step_label.clone()), Some(name.into()))?;
                }
                self.merge_primitive_result(name, result);
                Ok(None)
            }
            Err(err) => {
                let code = match &err {
                    DispatchError::UnknownPrimitive(_) => "unknown-primitive".to_string(),
                    DispatchError::BadArgs(_) => "primitive-args-mismatch".to_string(),
                    DispatchError::Primitive(_) => "primitive-failure".to_string(),
                };
                let message = err.to_string();
                self.emit_error(
                    code.clone(),
                    message.clone(),
                    Some(ErrorLocation {
                        file: self.procedure.command.clone(),
                        line: location.start_line,
                        col: location.start_col,
                    }),
                )?;
                Ok(Some(WalkOutcome::Errored { code, message }))
            }
        }
    }

    /// Merge a primitive's structured result into the walker context so a
    /// later step can read prior results — e.g. `compute-review-scope`'s
    /// `scope`/`diff-base` and `discover-rule-files`'s `selected`/`rules-dir`
    /// feed `build_perform_review_request`, and `write-review` reads
    /// `diff-base` plus the accumulated `findings`.
    ///
    /// Merge policy (kept deliberately explicit):
    /// - Only an object result merges; a non-object result (array, scalar,
    ///   null) is ignored — there are no top-level keys to thread.
    /// - Each top-level key of the result is inserted into the context,
    ///   **except** a session-seeded key (one present at construction, such
    ///   as `write-boundary` or `feature`), which is load-bearing and is
    ///   never overwritten by a primitive result. Targeted exceptions (see
    ///   the body): `create-feature`/`resolve-feature` retarget the seeded
    ///   `feature`/`path`, and `derive-boundary` unions its `boundary` into
    ///   `write-boundary`.
    /// - Among keys first introduced by primitives, last-write-wins.
    ///
    /// Results merge at the top level rather than under a per-primitive
    /// namespace because the payload builders and primitive arg binders read
    /// prior results by their bare key (`scope`, `selected`, `rules-dir`,
    /// `diff-base`); namespacing would hide them.
    fn merge_primitive_result(&mut self, name: &str, result: Value) {
        let Value::Object(map) = result else {
            return;
        };
        // Targeted merge exception (mirrors the `process-waivers`→`fired`
        // special case in `dispatch_primitive`): a successful `create-feature`
        // result carries the freshly created feature's slug and directory. On
        // `/ductus:specify` against a repo whose session already targets a
        // feature, `feature` and `path` are session-seeded keys the general
        // policy protects — but retargeting the session to the just-created
        // feature is the entire point of the command, so the later
        // `write-session` step must bind the NEW target rather than the stale
        // seed. A `created: true` create-feature result therefore overrides
        // exactly `feature` and `path`; no other primitive and no other key
        // escapes the seeded-key guard.
        // A second retarget exception, symmetric with create-feature: on
        // `/ductus:target` against a repo whose session already targets a
        // feature, switching the target is the entire point of the command.
        // Step 3's `resolve-feature` returns the resolved `feature`/`path`,
        // which the seeded-key guard would otherwise block from reaching the
        // later `write-session` step — leaving it to rewrite the stale seed.
        // Scoped to the `target` command and a `resolved` outcome so no other
        // command's `resolve-feature` call (e.g. `/ductus:analyze`'s identifier
        // resolution) escapes the guard.
        let retargets_session = (name == "create-feature"
            && map.get("created") == Some(&Value::Bool(true)))
            || (name == "resolve-feature"
                && self.procedure.command == "target"
                && map.get("outcome") == Some(&Value::String("resolved".into())));
        // A third targeted exception (scenario writecode-boundary-derivation):
        // a `derive-boundary` result must feed the writeCode validator's
        // `write-boundary` key, which the general policy would leave to the
        // session seed alone (the result emits under `boundary`, and seeded
        // keys are never overwritten). The merge is a UNION, never an
        // overwrite: a seeded boundary is a deliberate host/user grant the
        // derivation must not revoke — and on a fresh feature, whose
        // derivation holds only the spec-dir glob, the seed is what admits
        // the first out-of-spec edit. Sorted for deterministic payloads.
        if name == "derive-boundary"
            && let Some(Value::Array(derived)) = map.get("boundary")
        {
            let mut merged: std::collections::BTreeSet<String> = self
                .context
                .get("write-boundary")
                .and_then(|v| v.as_array())
                .into_iter()
                .flatten()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect();
            merged.extend(
                derived
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_string)),
            );
            self.context.insert(
                "write-boundary".into(),
                Value::Array(merged.into_iter().map(Value::String).collect()),
            );
        }
        for (key, value) in map {
            let seeded = self.seeded_keys.contains(&key);
            let overridable_target = retargets_session && (key == "feature" || key == "path");
            if seeded && !overridable_target {
                continue;
            }
            self.context.insert(key, value);
        }
        // Once a retargeting primitive has established the session target,
        // PIN `feature` and `path` for the rest of the walk.
        //
        // Those two keys are what the later `write-session` step binds, and
        // `path` there means the spec *directory*. Nearly every spec-reading
        // primitive that can run in between — `read-spec`, `label-criteria`,
        // `mark-criterion` — reports its own `path`, the spec *file*. When
        // the session file already carries a target those keys are seeded and
        // the guard above protects them; on a repo with **no** session file
        // they are unseeded, so the general merge policy let the file path
        // win and the walk wrote a session target pointing at `spec.md`.
        // That is not hypothetical: `/ductus:specify` gained a `label-criteria`
        // step between `create-feature` and `write-session` (spec 013), and
        // `/ductus:target` has read-spec sitting in the same gap.
        //
        // Pinning is the same narrow-exception idiom as the override above,
        // one step later: the retargeting primitives themselves stay exempt
        // (`overridable_target`), so a second retarget within one walk still
        // takes effect.
        if retargets_session {
            self.seeded_keys.insert("feature".into());
            self.seeded_keys.insert("path".into());
        }
    }

    fn handle_extension(
        &mut self,
        number: &StepNumber,
        identifier: &str,
        prose: &str,
    ) -> std::io::Result<Option<WalkOutcome>> {
        if identifier == ASSESS_SPEC_QUALITY {
            return self.handle_assessments(number, prose);
        }
        if identifier == ASK_CLARIFY_QUESTION
            && let Some(Value::Array(questions)) = self.context.get("open-questions").cloned()
        {
            return self.handle_clarify_questions(number, prose, questions);
        }
        let response = match self.exchange(identifier, prose)? {
            Ok(response) => response,
            Err(outcome) => return Ok(Some(outcome)),
        };
        self.accept_response(number, identifier, response)?;
        Ok(None)
    }

    /// `/analyze` steps 11 and 12: ask the host about each loaded rule of
    /// the step's tier, one `assessSpecQuality` request per rule, so every
    /// rule is asked about once and in the tier its own Statement carries
    /// (spec 060). The step's prose selects the tier; each request carries
    /// the rule's.
    ///
    /// A step that asks about nothing says so in the stream. With no rule
    /// loaded at all, or with prose naming no MUST or SHOULD tier, it also
    /// records itself unexamined: it examined nothing, and the record must
    /// not read as clean. A tier that is merely empty while the other has
    /// rules records nothing, because every loaded rule was asked about.
    fn handle_assessments(
        &mut self,
        number: &StepNumber,
        prose: &str,
    ) -> std::io::Result<Option<WalkOutcome>> {
        let step = Some(format_step_number(number));
        self.load_rules_once();
        let tier = payload::severity_from_step_prose(prose);
        let (loaded_none, asked): (bool, Vec<AssessSpecQualityRule>) = match &self.rules {
            Some(rules) => (
                rules.is_empty(),
                rules
                    .assessable
                    .iter()
                    .filter(|rule| rule.severity == tier)
                    .cloned()
                    .collect(),
            ),
            None => (true, Vec::new()),
        };
        if loaded_none || !matches!(tier, RuleSeverity::Must | RuleSeverity::Should) {
            if let Some(tally) = &mut self.analyze_tally {
                tally.record_step_asking_nothing();
            }
            let message = if loaded_none {
                "no rule loaded to assess; recorded under `rule-assessments-not-checked`"
            } else {
                "step names no MUST or SHOULD tier to select rules by; recorded under `rule-assessments-not-checked`"
            };
            self.emit_progress(message.into(), step, None)?;
            return Ok(None);
        }
        if asked.is_empty() {
            self.emit_progress(
                format!(
                    "no {}-tier rule loaded to assess",
                    tier.as_str().to_uppercase()
                ),
                step,
                None,
            )?;
            return Ok(None);
        }
        for rule in asked {
            let asked_tier = rule.severity;
            let seeded = serde_json::to_value(&rule).map_err(std::io::Error::other)?;
            self.context
                .insert(payload::ASSESSED_RULE_KEY.into(), seeded);
            let exchanged = self.exchange(ASSESS_SPEC_QUALITY, prose);
            self.context.remove(payload::ASSESSED_RULE_KEY);
            let response = match exchanged? {
                Ok(response) => response,
                Err(outcome) => return Ok(Some(outcome)),
            };
            if let Some(tally) = &mut self.analyze_tally {
                tally.record_assessment(asked_tier, &response);
            }
            self.accept_response(number, ASSESS_SPEC_QUALITY, response)?;
        }
        Ok(None)
    }

    /// `/clarify` step 6: one `askClarifyQuestion` round trip per open
    /// question, in `read-spec`'s order (spec 022, scenario
    /// `exec-clarify-asks-each-open-question`). Each question is seeded as
    /// the request's `question` and removed after its round trip; a `question`
    /// seeded before the step is restored afterwards. With no question left
    /// there is nothing to ask, and the step says so.
    fn handle_clarify_questions(
        &mut self,
        number: &StepNumber,
        prose: &str,
        questions: Vec<Value>,
    ) -> std::io::Result<Option<WalkOutcome>> {
        if questions.is_empty() {
            self.emit_progress(
                "no open question to ask".into(),
                Some(format_step_number(number)),
                None,
            )?;
            return Ok(None);
        }
        let seeded = self.context.remove(payload::CLARIFY_QUESTION_KEY);
        for question in questions {
            self.context
                .insert(payload::CLARIFY_QUESTION_KEY.into(), question);
            let exchanged = self.exchange(ASK_CLARIFY_QUESTION, prose);
            self.context.remove(payload::CLARIFY_QUESTION_KEY);
            let response = match exchanged? {
                Ok(response) => response,
                Err(outcome) => return Ok(Some(outcome)),
            };
            self.accept_response(number, ASK_CLARIFY_QUESTION, response)?;
        }
        if let Some(seeded) = seeded {
            self.context
                .insert(payload::CLARIFY_QUESTION_KEY.into(), seeded);
        }
        Ok(None)
    }

    /// Load the walk's rule set from its `rule-files` the first time an
    /// assessment step needs it, recording what it could not bring to an
    /// assessment then — once per walk, not once per step.
    fn load_rules_once(&mut self) {
        if self.rules.is_some() {
            return;
        }
        let loaded = payload::load_rules(&self.context, &self.repo);
        if let Some(tally) = &mut self.analyze_tally {
            tally.record_rule_set(loaded.unreadable_files, loaded.unassessable);
        }
        self.rules = Some(loaded);
    }

    /// Build one extension request from the walker context, send it, and
    /// await and validate its response. `Ok(Err(outcome))` when the walk
    /// halts: the request could not be built, or the response failed
    /// validation.
    fn exchange(
        &mut self,
        identifier: &str,
        prose: &str,
    ) -> std::io::Result<Result<Value, WalkOutcome>> {
        let request_id = self.fresh_request_id();
        let request = match payload::build_extension_request(
            identifier,
            &self.context,
            &self.repo,
            &self.procedure.command,
            prose,
        ) {
            Ok(value) => value,
            Err(err) => {
                let code = err.code().to_string();
                let message = err.to_string();
                self.emit_error(code.clone(), message.clone(), None)?;
                return Ok(Err(WalkOutcome::Errored { code, message }));
            }
        };
        self.emit_llm_request(identifier, &request_id, request)?;
        let response = self.await_llm_response(&request_id)?;
        if let Some(outcome) = self.validate_llm_response(identifier, &response)? {
            return Ok(Err(outcome));
        }
        Ok(Ok(response))
    }

    /// Thread a validated response into the walk: accumulate a
    /// `performReview` pass's outputs, keep the response under
    /// `llm:{identifier}`, and report it received.
    fn accept_response(
        &mut self,
        number: &StepNumber,
        identifier: &str,
        response: Value,
    ) -> std::io::Result<()> {
        // `performReview` runs once per pass; accumulate each pass's
        // findings and observations into the shared context keys so a later
        // `write-review` step consumes the union across all passes. An
        // observation is a pass output the reviewer judged real but that
        // matched no loaded rule, and it reaches `write-review` by this same
        // route — in `PassObservation`'s shape, text and path alone, since
        // a pass has no disposition to give. Both are taken from the typed
        // response, so nothing a pass volunteers beyond that shape is
        // carried. The walker has no operator to disposition an observation,
        // so it is recorded undispositioned (spec 058, AC26).
        if identifier == "performReview" {
            let pass = serde_json::to_value(reparse::<PerformReviewResponse>(&response)?)
                .map_err(std::io::Error::other)?;
            for key in payload::PERFORM_REVIEW_ACCUMULATORS {
                let Some(Value::Array(items)) = pass.get(*key) else {
                    continue;
                };
                match self.context.get_mut(*key) {
                    Some(Value::Array(existing)) => existing.extend(items.iter().cloned()),
                    _ => {
                        self.context
                            .insert((*key).to_string(), Value::Array(items.clone()));
                    }
                }
            }
        }
        self.context.insert(format!("llm:{identifier}"), response);
        self.emit_progress(
            format!("received llm-response for `{identifier}`"),
            Some(format_step_number(number)),
            None,
        )?;
        Ok(())
    }

    /// Block until the host delivers an `llm-response` matching
    /// `request_id`. Any other inbound envelope — a wrong type, or an
    /// `llm-response` for a superseded request-id — is logged to stderr
    /// and skipped per the protocol's ignore-and-continue rule; the
    /// framing layer ([`read_envelope`]) already skips malformed and
    /// blank lines the same way.
    fn await_llm_response(&mut self, request_id: &str) -> std::io::Result<Value> {
        loop {
            match self.read_envelope()? {
                ProtocolMessage::LlmResponse {
                    request_id: response_id,
                    response,
                } if response_id == request_id => return Ok(response),
                ProtocolMessage::LlmResponse {
                    request_id: other, ..
                } => {
                    eprintln!(
                        "runtime: ignoring llm-response for request-id `{other}` while awaiting `{request_id}`"
                    );
                }
                other => {
                    eprintln!(
                        "runtime: ignoring {} envelope while awaiting llm-response `{request_id}`",
                        envelope_kind(&other)
                    );
                }
            }
        }
    }

    /// Block until the host delivers a `gate-response` matching
    /// `request_id`; returns the user's decision. Same ignore-and-continue
    /// rule as [`Walker::await_llm_response`].
    fn await_gate_response(&mut self, request_id: &str) -> std::io::Result<bool> {
        loop {
            match self.read_envelope()? {
                ProtocolMessage::GateResponse {
                    request_id: response_id,
                    confirmed,
                } if response_id == request_id => return Ok(confirmed),
                ProtocolMessage::GateResponse {
                    request_id: other, ..
                } => {
                    eprintln!(
                        "runtime: ignoring gate-response for request-id `{other}` while awaiting `{request_id}`"
                    );
                }
                other => {
                    eprintln!(
                        "runtime: ignoring {} envelope while awaiting gate-response `{request_id}`",
                        envelope_kind(&other)
                    );
                }
            }
        }
    }

    fn handle_gate(
        &mut self,
        number: &StepNumber,
        prose: &str,
    ) -> std::io::Result<Option<WalkOutcome>> {
        let request_id = self.fresh_request_id();
        let gate = format!("step-{}", format_step_number(number));
        self.emit_gate_confirm(&gate, &request_id, prose)?;
        let confirmed = self.await_gate_response(&request_id)?;
        self.emit_progress(
            format!(
                "gate `{gate}` {}",
                if confirmed { "confirmed" } else { "denied" }
            ),
            Some(format_step_number(number)),
            None,
        )?;
        if confirmed {
            // A gate exists to authorize the writes after it, so a confirmed
            // one binds `apply` for every later primitive. Without it, a
            // primitive that previews unless told to apply — `prune-tasks` at
            // prune.md step 7 — would preview a second time, and the walk
            // would complete having written nothing.
            self.gate_confirmed = true;
            Ok(None)
        } else {
            // Denial is a clean exit per §partial-failure-semantics.
            self.emit_complete_with(serde_json::json!({ "confirmed": false, "gate": gate }))?;
            Ok(Some(WalkOutcome::Complete))
        }
    }

    fn fresh_request_id(&mut self) -> String {
        self.request_counter += 1;
        format!("req-{}", self.request_counter)
    }

    fn emit(&mut self, message: &ProtocolMessage) -> std::io::Result<()> {
        write_envelope(self.writer, message)
    }

    fn emit_progress(
        &mut self,
        message: String,
        step: Option<String>,
        primitive: Option<String>,
    ) -> std::io::Result<()> {
        self.emit(&ProtocolMessage::Progress {
            message,
            step,
            primitive,
        })
    }

    fn emit_llm_request(
        &mut self,
        extension_point: &str,
        request_id: &str,
        request: Value,
    ) -> std::io::Result<()> {
        self.emit(&ProtocolMessage::LlmRequest {
            extension_point: extension_point.into(),
            request_id: request_id.into(),
            request,
        })
    }

    fn emit_gate_confirm(
        &mut self,
        gate: &str,
        request_id: &str,
        prompt: &str,
    ) -> std::io::Result<()> {
        self.emit(&ProtocolMessage::GateConfirm {
            gate: gate.into(),
            request_id: request_id.into(),
            prompt: prompt.trim().into(),
        })
    }

    fn emit_complete(&mut self) -> std::io::Result<()> {
        self.emit_complete_with(Value::Object(Map::new()))
    }

    fn emit_complete_with(&mut self, result: Value) -> std::io::Result<()> {
        self.emit(&ProtocolMessage::Complete {
            result,
            runtime_version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn emit_error(
        &mut self,
        code: String,
        message: String,
        location: Option<ErrorLocation>,
    ) -> std::io::Result<()> {
        self.emit(&ProtocolMessage::Error {
            code,
            message,
            runtime_version: env!("CARGO_PKG_VERSION").into(),
            location,
        })
    }

    fn read_envelope(&mut self) -> std::io::Result<ProtocolMessage> {
        read_envelope(self.reader)
    }

    /// Validate an incoming `llm-response` payload against the schema for
    /// the extension point that emitted the request, and (for `writeCode`)
    /// reject edits whose path escapes the write boundary. Returns
    /// `Ok(Some(Errored))` when validation fails (the caller emits the
    /// error envelope and halts); `Ok(None)` when the response is well-formed.
    fn validate_llm_response(
        &mut self,
        identifier: &str,
        response: &Value,
    ) -> std::io::Result<Option<WalkOutcome>> {
        if let Err(err) = extensions::validate_response(identifier, response) {
            let (code, message) = match &err {
                ValidationError::UnknownExtension(_) => {
                    ("unknown-extension".to_string(), err.to_string())
                }
                ValidationError::Schema { .. } => ("schema-mismatch".to_string(), err.to_string()),
                ValidationError::OutOfBoundary { .. } => {
                    // Reachable only through validate_write_code_boundary;
                    // validate_response never produces it.
                    ("out-of-boundary-edit".to_string(), err.to_string())
                }
                ValidationError::EditContent { .. } => {
                    ("invalid-edit".to_string(), err.to_string())
                }
            };
            self.emit_error(code.clone(), message.clone(), None)?;
            return Ok(Some(WalkOutcome::Errored { code, message }));
        }
        if identifier == "writeCode" {
            let parsed = reparse::<WriteCodeResponse>(response)?;
            let boundary = self.write_boundary();
            if let Err(err) = extensions::validate_write_code_boundary(&parsed, &boundary) {
                let message = err.to_string();
                self.emit_error("out-of-boundary-edit".into(), message.clone(), None)?;
                return Ok(Some(WalkOutcome::Errored {
                    code: "out-of-boundary-edit".into(),
                    message,
                }));
            }
        }
        Ok(None)
    }

    /// Read the write boundary from the walker's context. The
    /// `write-boundary` context key is expected to be a `Vec<String>` of
    /// glob patterns; absent or malformed values yield an empty boundary,
    /// which rejects every path.
    fn write_boundary(&self) -> Vec<String> {
        self.context
            .get("write-boundary")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// The typed form of an `llm-response` that [`extensions::validate_response`]
/// already accepted. Validation parsed the same shape, so this cannot fail on a
/// validated payload; a failure would mean the two disagree, and it surfaces as
/// an I/O error rather than being dropped.
fn reparse<T: serde::de::DeserializeOwned>(response: &Value) -> std::io::Result<T> {
    serde_json::from_value(response.clone()).map_err(|err| {
        std::io::Error::other(format!("re-parse of validated payload failed: {err}"))
    })
}

/// The envelope's `type` discriminator, for stderr ignore-and-continue logs.
fn envelope_kind(message: &ProtocolMessage) -> &'static str {
    match message {
        ProtocolMessage::LlmRequest { .. } => "llm-request",
        ProtocolMessage::LlmResponse { .. } => "llm-response",
        ProtocolMessage::GateConfirm { .. } => "gate-confirm",
        ProtocolMessage::GateResponse { .. } => "gate-response",
        ProtocolMessage::Progress { .. } => "progress",
        ProtocolMessage::Complete { .. } => "complete",
        ProtocolMessage::Error { .. } => "error",
    }
}

/// What a session primitive's result has to tell the operator, one line
/// each: its notices, the other sessions sharing the feature it wrote, the
/// sessions its sweep removed, and the session files it could not check
/// (spec 062). A host displays these; an exec
/// walk has no host, so the walker emits them as progress lines, or the
/// notices its `resolve-session` step consumes would never be seen (AC19,
/// AC21).
fn session_report_lines(name: &str, result: &Value) -> Vec<String> {
    if !matches!(name, "resolve-session" | "write-session") {
        return Vec::new();
    }
    let list = |key: &str| {
        result
            .get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
    };
    let notices = list("notices")
        .filter_map(|notice| notice.get("message")?.as_str())
        .map(|message| format!("session notice: {message}"));
    let peers = list("peers").filter_map(|peer| {
        let session = peer.get("session")?.as_str()?;
        let mut target = peer.get("feature")?.as_str()?.to_owned();
        if let Some(scenario) = peer.get("scenario").and_then(Value::as_str) {
            target = format!("{target}/{scenario}");
        }
        Some(match peer.get("last-used").and_then(Value::as_str) {
            Some(when) => {
                format!("session peer: {session} also targets {target} (last used {when})")
            }
            None => format!("session peer: {session} also targets {target}"),
        })
    });
    let expired = list("expired").filter_map(Value::as_str).map(|label| {
        let days = crate::session::IDLE_EXPIRY.as_secs() / 86_400;
        format!("expired session: {label} was idle past {days} days and removed")
    });
    let unreadable = list("unreadable")
        .filter_map(Value::as_str)
        .map(|path| format!("unreadable session file: {path}"));
    notices
        .chain(peers)
        .chain(expired)
        .chain(unreadable)
        .collect()
}

fn format_step_number(number: &StepNumber) -> String {
    number
        .0
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

#[derive(Debug, thiserror::Error)]
enum DispatchError {
    #[error("unknown primitive `{0}`")]
    UnknownPrimitive(String),
    #[error("failed to bind args for primitive: {0}")]
    BadArgs(serde_json::Error),
    #[error("{0}")]
    Primitive(#[from] primitives::PrimitiveError),
}

/// Dispatch a primitive by name. Args are deserialized from `context`
/// — any keys it doesn't need are ignored, so callers can pass a single
/// merged binding map. Returns the primitive's result as a JSON value.
// A flat dispatch match with one arm per primitive — the exec-path
// counterpart of `main`'s CLI match, which carries the same allow for the
// same reason. It grows by one arm with each primitive and is mechanical,
// so the line-count lint is not meaningful here; splitting it would put
// half the registry somewhere a reader would not think to look.
#[allow(clippy::too_many_lines)]
fn dispatch_primitive(
    name: &str,
    context: &Map<String, Value>,
    repo: &Path,
) -> Result<Value, DispatchError> {
    let mut bindings = context.clone();
    // Exec-path binding for `process-waivers`: the `performReview` passes
    // accumulate their findings under the walker's `findings` context key,
    // and the primitive classifies waivers against exactly that set via
    // its `fired` argument. Bind `findings` → `fired` unless the caller
    // seeded `fired` explicitly — `FiredFinding` deserialization keeps the
    // `(rule, file)` anchor and ignores the extra severity/range keys.
    if name == "process-waivers"
        && !bindings.contains_key("fired")
        && let Some(findings @ Value::Array(_)) = bindings.get("findings").cloned()
    {
        bindings.insert("fired".into(), findings);
    }
    // Exec-path binding for `write-analysis`: its `findings` argument is the
    // host's itemized dispositions, which the walker never produces — the
    // fix-and-route step is host responsibility and no-ops here, so an exec
    // run itemizes nothing, and against the tier counts the walker tallied
    // (see `analyze_tally`) the writer counts every live finding as
    // undispositioned (spec 058, AC26). The context's `findings` key belongs
    // to other primitives (`validate-frontmatter`, `check-artifacts`,
    // `check-orphaned-references`), whose entries are not analyze findings and
    // would fail to bind, so it never reaches this primitive.
    if name == "write-analysis" {
        bindings.remove("findings");
    }
    // Exec-path binding for the spec-reading primitives `/analyze` dispatches
    // against "the spec path": a session target's `path` is the spec
    // *directory*, as `write-session` records it, so it is bound to the
    // directory's `spec.md`. Before this, an exec `/analyze` over a real
    // session failed at its first such step reading a directory.
    if matches!(
        name,
        "validate-frontmatter" | "resolve-anchor" | "check-rule-ids"
    ) && let Some(Value::String(path)) = bindings.get("path")
    {
        let spec = spec_file(repo, path);
        bindings.insert("path".into(), Value::String(spec));
    }
    // Exec-path binding for `resolve-anchor`: `/analyze` step 4 resolves the
    // spec's `§` references against the constitution's markers, not the
    // spec's own, which would report every reference unresolved. Bound to the
    // project's constitution when the context names no markers file.
    if name == "resolve-anchor"
        && !bindings.contains_key("markers-path")
        && let Some(constitution) = crate::schema::paths::constitution_path(repo)
    {
        bindings.insert("markers-path".into(), Value::String(constitution.into()));
    }
    // Exec-path binding for `mark-criterion`: `/ductus:implement`'s completion
    // gate seeds `criterion-index`/`checked: true`, but the checkbox flip
    // must honor the `verifyCriteria` verdict the host returned first — only
    // a criterion the LLM affirmatively confirmed `met: true` may be checked
    // (data-model §verifyCriteria). When a `llm:verifyCriteria` response is in
    // context, rebind `checked` to that verdict for the seeded index: a
    // `met: false` or absent verdict rebinds `checked` to `false`, so the
    // dispatch is a no-op (an already-unchecked criterion stays unchecked) and
    // an unconfirmed criterion is never marked. With no verifyCriteria response
    // present the seeded `checked` stands, so direct MCP/CLI calls and other
    // commands are unaffected.
    if name == "mark-criterion"
        && let Some(met) = bindings
            .get("llm:verifyCriteria")
            .map(|verify| criterion_verified_met(verify, bindings.get("criterion-index")))
    {
        bindings.insert("checked".into(), Value::Bool(met));
    }
    let value = Value::Object(bindings);
    macro_rules! call {
        ($args:ty, $module:ident) => {{
            let args: $args = serde_json::from_value(value).map_err(DispatchError::BadArgs)?;
            let result = primitives::$module::run(&args, repo)?;
            Ok(serde_json::to_value(result).unwrap_or(Value::Null))
        }};
    }
    match name {
        "read-spec" => call!(ReadSpecArgs, read_spec),
        "read-tasks" => call!(ReadTasksArgs, read_tasks),
        "mark-task" => call!(MarkTaskArgs, mark_task),
        "mark-criterion" => call!(MarkCriterionArgs, mark_criterion),
        "set-status" => call!(SetStatusArgs, set_status),
        "derive-boundary" => call!(DeriveBoundaryArgs, derive_boundary),
        "diff-cross-spec" => call!(DiffCrossSpecArgs, diff_cross_spec),
        "discover-rule-files" => call!(DiscoverRuleFilesArgs, discover_rule_files),
        "process-waivers" => call!(ProcessWaiversArgs, process_waivers),
        "process-decisions" => call!(ProcessDecisionsArgs, process_decisions),
        "compute-review-scope" => call!(ComputeReviewScopeArgs, compute_review_scope),
        "write-review" => call!(WriteReviewArgs, write_review),
        "write-analysis" => call!(WriteAnalysisArgs, write_analysis),
        "check-stuck" => call!(CheckStuckArgs, check_stuck),
        "validate-frontmatter" => call!(ValidateFrontmatterArgs, validate_frontmatter),
        "resolve-anchor" => call!(ResolveAnchorArgs, resolve_anchor),
        "resolve-feature" => call!(ResolveFeatureArgs, resolve_feature),
        "resolve-references" => call!(ResolveReferencesArgs, resolve_references),
        "resolve-constitutions" => call!(ResolveConstitutionsArgs, resolve_constitutions),
        "traverse-deps" => call!(TraverseDepsArgs, traverse_deps),
        "check-rule-ids" => call!(CheckRuleIdsArgs, check_rule_ids),
        "check-promotion-coverage" => {
            call!(CheckPromotionCoverageArgs, check_promotion_coverage)
        }
        "run-generator" => call!(RunGeneratorArgs, run_generator),
        "lint-markdown" => call!(LintMarkdownArgs, lint_markdown),
        "fetch-archive" => call!(FetchArchiveArgs, fetch_archive),
        "extract-archive" => call!(ExtractArchiveArgs, extract_archive),
        "apply-manifest" => call!(ApplyManifestArgs, apply_manifest),
        "enforce-manifest" => call!(EnforceManifestArgs, enforce_manifest),
        "merge-managed-block" => call!(MergeManagedBlockArgs, merge_managed_block),
        "merge-permissions" => call!(MergePermissionsArgs, merge_permissions),
        "migrate-session-file" => call!(MigrateSessionFileArgs, migrate_session_file),
        "relocate-audit-records" => call!(RelocateAuditRecordsArgs, relocate_audit_records),
        "create-scenario" => call!(CreateScenarioArgs, create_scenario),
        "label-criteria" => call!(LabelCriteriaArgs, label_criteria),
        "create-feature" => call!(CreateFeatureArgs, create_feature),
        "create-plan-artifacts" => call!(CreatePlanArtifactsArgs, create_plan_artifacts),
        "check-review-gate" => call!(CheckReviewGateArgs, check_review_gate),
        "append-question" => call!(AppendQuestionArgs, append_question),
        "append-task" => call!(AppendTaskArgs, append_task),
        "append-inbox" => call!(AppendInboxArgs, append_inbox),
        "remove-inbox-item" => call!(RemoveInboxItemArgs, remove_inbox_item),
        "derive-routing-candidates" => {
            call!(DeriveRoutingCandidatesArgs, derive_routing_candidates)
        }
        "check-corpus-links" => {
            call!(CheckCorpusLinksArgs, check_corpus_links)
        }
        "check-orphaned-references" => {
            call!(CheckOrphanedReferencesArgs, check_orphaned_references)
        }
        "check-step-references" => {
            call!(CheckStepReferencesArgs, check_step_references)
        }
        "check-command-flags" => {
            call!(CheckCommandFlagsArgs, check_command_flags)
        }
        "check-unfolded-specs" => call!(CheckUnfoldedSpecsArgs, check_unfolded_specs),
        "rewrite-spec-links" => call!(RewriteSpecLinksArgs, rewrite_spec_links),
        "retire-feature" => call!(RetireFeatureArgs, retire_feature),
        "invalidate-review" => call!(InvalidateReviewArgs, invalidate_review),
        "derive-dependencies" => {
            call!(DeriveDependenciesArgs, derive_dependencies)
        }
        "derive-references" => {
            call!(DeriveReferencesArgs, derive_references)
        }
        "check-artifacts" => call!(CheckArtifactsArgs, check_artifacts),
        "check-artifact-size" => call!(CheckArtifactSizeArgs, check_artifact_size),
        "prune-tasks" => call!(PruneTasksArgs, prune_tasks),
        "prune-plan" => call!(PrunePlanArgs, prune_plan),
        "dashboard" => call!(DashboardArgs, dashboard),
        "write-session" => call!(WriteSessionArgs, write_session),
        "resolve-session" => call!(ResolveSessionArgs, resolve_session),
        "retarget-sessions" => call!(RetargetSessionsArgs, retarget_sessions),
        other => Err(DispatchError::UnknownPrimitive(other.into())),
    }
}

/// The spec file a context `path` names. A session target's `path` is the spec
/// *directory*, as `write-session` records it, and what `/analyze` reads as
/// "the spec path" is that directory's `spec.md`; a `path` naming anything
/// else is the spec file already and is returned as given.
fn spec_file(repo: &Path, path: &str) -> String {
    if repo.join(path).is_dir() {
        format!("{}/spec.md", path.trim_end_matches('/'))
    } else {
        path.to_string()
    }
}

/// Bind the list seeds an exec `/analyze` walk needs and no session carries
/// (spec 022 scenario `exec-analyze-derives-its-list-seeds`). A session
/// `write-session` writes names the target alone, and a `key=value` argument
/// binds only a string, so without these no command line could complete the
/// walk. A seeded value is used as given.
///
/// - `rule-files` — every rule file in the project's rule-file directory,
///   resolved as `discover-rule-files` resolves it. `/analyze` loads all of
///   them whatever the project's surfaces, because a citation may name a rule
///   on any surface. A directory that is absent, empty, or unreadable binds
///   none, and step 5's `examined: 0` then records each citation unexamined
///   rather than missing.
/// - `paths` — the feature directory's markdown files, step 7's lint subject,
///   as the glob `check-review-gate` lints. A scenario pair in the session
///   does not narrow it. Unbound without a `feature` to name the directory.
///
/// `analyzed-at` and `analyzed-against` are not derived: the caller supplies
/// them, as `/analyze`'s host does, and a missing one fails by name at
/// `write-analysis`.
fn derive_analyze_seeds(context: &mut Map<String, Value>, repo: &Path) {
    if !context.contains_key("rule-files") {
        let rule_files = match primitives::discover_rule_files::resolve_rules_dir(repo) {
            (Some(dir), rel) => primitives::discover_rule_files::list_rule_files(&dir)
                .unwrap_or_default()
                .into_iter()
                .map(|name| Value::String(format!("{rel}/{name}")))
                .collect(),
            (None, _) => Vec::new(),
        };
        context.insert("rule-files".into(), Value::Array(rule_files));
    }
    if !context.contains_key("paths")
        && let Some(Value::String(feature)) = context.get("feature")
        && primitives::validate_no_traversal(feature).is_ok()
    {
        let root = crate::schema::paths::Paths::load(repo).specs_root;
        let glob = primitives::lint_markdown::feature_markdown_glob(&format!("{root}/{feature}"));
        context.insert("paths".into(), Value::Array(vec![Value::String(glob)]));
    }
}

/// Whether a `verifyCriteria` response affirmatively confirms the criterion
/// at `criterion_index` as `met: true`. A missing/non-numeric index, a
/// missing `results` array, an absent verdict for the index, or an explicit
/// `met: false` all yield `false` — the completion gate flips only criteria
/// the response confirms (data-model §verifyCriteria).
fn criterion_verified_met(verify: &Value, criterion_index: Option<&Value>) -> bool {
    let Some(index) = criterion_index.and_then(Value::as_u64) else {
        return false;
    };
    let Some(results) = verify.get("results").and_then(Value::as_array) else {
        return false;
    };
    results
        .iter()
        .find(|entry| entry.get("index").and_then(Value::as_u64) == Some(index))
        .and_then(|entry| entry.get("met").and_then(Value::as_bool))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::schema::procedure::SourceRange;
    use std::io::Cursor;

    /// Spec 062, AC19/AC21: an exec walk has no host to display a session
    /// primitive's notices, so the walker turns each into a progress line.
    #[test]
    fn a_session_result_reports_everything_a_host_would_display() {
        let result = serde_json::json!({
            "notices": [{"kind": "folded", "message": "Target 1.1-a was folded into 055-a."}],
            "peers": [
                {
                    "session": "review",
                    "feature": "055-a",
                    "scenario": null,
                    "last-used": "2026-09-29T12:00:00Z"
                },
                { "session": "x", "feature": "055-a", "scenario": "edge" }
            ],
            "expired": ["old"],
            "unreadable": [".ductus/sessions/broken.toml"],
        });
        let days = crate::session::IDLE_EXPIRY.as_secs() / 86_400;
        assert_eq!(
            session_report_lines("write-session", &result),
            [
                "session notice: Target 1.1-a was folded into 055-a.".to_owned(),
                "session peer: review also targets 055-a (last used 2026-09-29T12:00:00Z)"
                    .to_owned(),
                "session peer: x also targets 055-a/edge".to_owned(),
                format!("expired session: old was idle past {days} days and removed"),
                "unreadable session file: .ductus/sessions/broken.toml".to_owned(),
            ]
        );
        let waivers = serde_json::json!({ "notices": ["waiver expired: x"] });
        assert_eq!(
            session_report_lines("process-waivers", &waivers),
            [] as [String; 0]
        );
    }

    fn loc() -> SourceRange {
        SourceRange {
            start_line: 1,
            start_col: 1,
            end_line: 1,
            end_col: 1,
        }
    }

    fn ctx_with_feature(feature: &str) -> Map<String, Value> {
        let mut m = Map::new();
        m.insert("feature".into(), Value::String(feature.into()));
        m
    }

    fn fixture_repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/primitives/sample-repo")
    }

    /// Registry pin: `dispatch_primitive` must handle every name in the
    /// canonical [`crate::schema::registry::PRIMITIVE_REGISTRY`]. The
    /// dispatch match is hand-written, so a primitive added to the registry
    /// (and thereby to `PRIMITIVE_NAMES` / `TOOL_NAMES`) without a dispatch
    /// arm would parse in command files but fail at execution with
    /// `unknown-primitive`. Empty args against a scratch tempdir: any
    /// outcome is acceptable except the unknown-primitive variant.
    #[test]
    fn dispatch_handles_every_registry_primitive() {
        let tmp = tempfile::tempdir().unwrap();
        // `gate-confirm` never reaches dispatch: `handle_step` intercepts its
        // step and blocks in `handle_gate`, because the step is the gate.
        for name in crate::schema::registry::PRIMITIVE_REGISTRY
            .iter()
            .filter(|name| **name != "gate-confirm")
        {
            let result = dispatch_primitive(name, &Map::new(), tmp.path());
            assert!(
                !matches!(result, Err(DispatchError::UnknownPrimitive(_))),
                "interpreter dispatch has no arm for registry primitive `{name}`"
            );
        }
    }

    /// Task 65 (scenarios/mcp-arg-unknown-field-strictness.md), exec surface:
    /// the interpreter binds each primitive's args from a clone of the whole
    /// walker context (a deliberate superset), so an unknown key — here the
    /// `snake_case` misspelling `include_body` alongside the real `feature` —
    /// is ignored, not rejected. Counterpart to the MCP-surface rejection in
    /// `tests/mcp.rs`: strictness lives only at the MCP boundary, so the exec
    /// path's superset-context binding keeps working.
    #[test]
    fn exec_path_ignores_unknown_argument_key() {
        let mut context = ctx_with_feature("001-basic");
        context.insert("include_body".into(), Value::Bool(false));
        let repo = fixture_repo();
        let result = dispatch_primitive("read-spec", &context, &repo);
        assert!(
            result.is_ok(),
            "exec path must ignore an unknown context key, got: {result:?}"
        );
    }

    #[test]
    fn empty_procedure_emits_complete_only() {
        let procedure = Procedure {
            command: "noop".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<&str> = std::str::from_utf8(&writer).unwrap().lines().collect();
        assert_eq!(lines.len(), 1);
        let value: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(value["type"], "complete");
    }

    #[test]
    fn primitive_step_dispatches_and_emits_progress() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Primitive {
                number: StepNumber(vec![1]),
                name: "read-spec".into(),
                prose: String::new(),
                location: loc(),
            }],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            ctx_with_feature("001-basic"),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0]["type"], "progress");
        assert_eq!(lines[0]["primitive"], "read-spec");
        assert_eq!(lines[1]["type"], "complete");
    }

    #[test]
    fn primitive_failure_emits_error_and_halts() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![
                Step::Primitive {
                    number: StepNumber(vec![1]),
                    name: "read-spec".into(),
                    prose: String::new(),
                    location: loc(),
                },
                Step::Primitive {
                    number: StepNumber(vec![2]),
                    name: "read-tasks".into(),
                    prose: String::new(),
                    location: loc(),
                },
            ],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            ctx_with_feature("999-nonexistent"),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        match outcome {
            WalkOutcome::Errored { code, .. } => assert_eq!(code, "primitive-failure"),
            WalkOutcome::Complete => panic!("expected Errored, got Complete"),
        }
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        // progress(read-spec), error — second primitive never runs.
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0]["type"], "progress");
        assert_eq!(lines[1]["type"], "error");
    }

    /// A single-request extension step: one request out, its response in.
    /// (`assessSpecQuality` fans out per rule and has its own tests.)
    #[test]
    fn extension_step_emits_llm_request_and_consumes_response() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Extension {
                number: StepNumber(vec![1]),
                identifier: "askClarifyQuestion".into(),
                prose: String::new(),
                location: loc(),
            }],
        };
        let response = "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"answer\":\"yes\"}}\n";
        let mut reader = Cursor::new(response.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        // llm-request, progress(received), complete
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0]["type"], "llm-request");
        assert_eq!(lines[0]["extension-point"], "askClarifyQuestion");
        assert_eq!(lines[0]["request-id"], "req-1");
        assert_eq!(lines[1]["type"], "progress");
        assert_eq!(lines[2]["type"], "complete");
    }

    #[test]
    fn perform_review_emits_one_llm_request_per_pass_step() {
        // One `performReview` step per pass → one llm-request each. A skipped
        // pass is simply an absent step (this procedure carries three of the
        // five), so no request is emitted for it.
        let step = |n: u32| Step::Extension {
            number: StepNumber(vec![n]),
            identifier: "performReview".into(),
            prose: String::new(),
            location: loc(),
        };
        let procedure = Procedure {
            command: "review".into(),
            steps: vec![step(1), step(2), step(3)],
        };
        let responses = concat!(
            "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[]}}\n",
            "{\"type\":\"llm-response\",\"request-id\":\"req-2\",\"response\":{\"findings\":[]}}\n",
            "{\"type\":\"llm-response\",\"request-id\":\"req-3\",\"response\":{\"findings\":[]}}\n",
        );
        let mut reader = Cursor::new(responses.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let requests: Vec<&Value> = lines
            .iter()
            .filter(|l| l["type"] == "llm-request")
            .collect();
        assert_eq!(requests.len(), 3);
        for request in requests {
            assert_eq!(request["extension-point"], "performReview");
        }
    }

    #[test]
    fn perform_review_findings_flow_into_write_review() {
        // Two pass steps then a write-review step: each pass's findings
        // accumulate into `context["findings"]`, which write-review consumes.
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();

        let procedure = Procedure {
            command: "review".into(),
            steps: vec![
                Step::Extension {
                    number: StepNumber(vec![1]),
                    identifier: "performReview".into(),
                    prose: String::new(),
                    location: loc(),
                },
                Step::Extension {
                    number: StepNumber(vec![2]),
                    identifier: "performReview".into(),
                    prose: String::new(),
                    location: loc(),
                },
                Step::Primitive {
                    number: StepNumber(vec![3]),
                    name: "write-review".into(),
                    prose: String::new(),
                    location: loc(),
                },
            ],
        };
        let responses = concat!(
            "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[{\"rule\":\"SEC-BE-001\",\"severity\":\"must\",\"file\":\"runtime/src/a.rs\",\"line-range\":\"1-5\",\"confidence\":\"high\"}]}}\n",
            "{\"type\":\"llm-response\",\"request-id\":\"req-2\",\"response\":{\"findings\":[{\"rule\":\"QUAL-002\",\"severity\":\"should\",\"file\":\"runtime/src/b.rs\",\"line-range\":\"1-5\",\"confidence\":\"high\"}]}}\n",
        );
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert(
            "reviewed-at".into(),
            Value::String("2026-07-04T00:00:00Z".into()),
        );
        context.insert("reviewed-against".into(), Value::String("abc1234".into()));
        context.insert("diff-base".into(), Value::String("def5678".into()));

        let mut reader = Cursor::new(responses.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            tmp.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);

        // write-review rendered the union of both passes' findings.
        let review = std::fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(review.contains("must-violations: 1"));
        assert!(review.contains("should-violations: 1"));
        assert!(review.contains("### MUST: SEC-BE-001"));
        assert!(review.contains("### SHOULD: QUAL-002"));
        // Blocking flowed into the record, which lives in review.md (spec 057).
        assert!(review.contains("blocking: true"), "{review}");
        assert!(
            !std::fs::read_to_string(dir.join("spec.md"))
                .unwrap()
                .contains("blocking:"),
            "the spec carries no review record"
        );
    }

    /// An exec run has no operator to disposition an observation, and a pass
    /// has no disposition to give, so the walker carries an observation's text
    /// and path alone: one the reviewer's response marks `fixed` is recorded
    /// undispositioned (spec 058, AC26) — never as `fixed` without a gate —
    /// and a malformed disposition no longer fails the run over a field the
    /// walker would not use.
    #[test]
    fn perform_review_dispositions_are_not_carried_on_the_exec_path() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();
        let procedure = Procedure {
            command: "review".into(),
            steps: vec![
                Step::Extension {
                    number: StepNumber(vec![1]),
                    identifier: "performReview".into(),
                    prose: String::new(),
                    location: loc(),
                },
                Step::Primitive {
                    number: StepNumber(vec![2]),
                    name: "write-review".into(),
                    prose: String::new(),
                    location: loc(),
                },
            ],
        };
        let responses = "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"findings\":[],\"observations\":[{\"text\":\"perf: slow\",\"path\":\"a.rs\",\"disposition\":{\"outcome\":\"fixed\"},\"decision-key\":\"perf: slow — `a.rs`\"},{\"text\":\"bug: odd\",\"disposition\":{\"outcome\":\"wontfix\"}}]}}\n";
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert(
            "reviewed-at".into(),
            Value::String("2026-07-04T00:00:00Z".into()),
        );
        context.insert("reviewed-against".into(), Value::String("abc1234".into()));
        context.insert("diff-base".into(), Value::String("def5678".into()));
        let mut reader = Cursor::new(responses.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            tmp.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
        let review = std::fs::read_to_string(dir.join("review.md")).unwrap();
        assert!(
            review.contains(
                "dispositions:\n  fixed: 0\n  routed: 0\n  discarded: 0\n  undispositioned: 2\n"
            ),
            "{review}"
        );
    }

    /// An exec `/analyze` records the tier counts its own detection produced
    /// and every live finding undispositioned (spec 058, AC26). It once bound
    /// no counts, so a spec with two blocking findings was recorded 0/0/0,
    /// clean, and fully examined. The session's `path` is the spec directory,
    /// as `write-session` records it, and the spec-reading steps bind its
    /// `spec.md`.
    #[test]
    fn an_exec_analyze_records_the_tiers_its_detection_found() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: planned\ndependencies: []\n---\n\n# x\n\n## Acceptance Criteria\n\n- [ ] it works\n",
        )
        .unwrap();
        let step = |n: u32, name: &str| Step::Primitive {
            number: StepNumber(vec![n]),
            name: name.into(),
            prose: String::new(),
            location: loc(),
        };
        let procedure = Procedure {
            command: "analyze".into(),
            steps: vec![
                step(2, "validate-frontmatter"),
                step(8, "check-artifacts"),
                step(21, "write-analysis"),
            ],
        };
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("path".into(), Value::String("specs/001-x".into()));
        context.insert(
            "analyzed-at".into(),
            Value::String("2026-09-25T00:00:00Z".into()),
        );
        context.insert("analyzed-against".into(), Value::String("abc1234".into()));
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            tmp.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
        let analysis = std::fs::read_to_string(dir.join("analysis.md")).unwrap();
        // `plan.md` and `tasks.md` are both required at `planned`.
        assert!(analysis.contains("\nblocking-findings: 2\n"), "{analysis}");
        assert!(analysis.contains("\nblocking: true\n"), "{analysis}");
        assert!(analysis.contains("  undispositioned: 2\n"), "{analysis}");
    }

    /// An exec `/analyze` records an oversized artifact step 17 found as an
    /// advisory finding, undispositioned (spec 063, AC1): the walker binds
    /// the feature from the session and counts the result.
    #[test]
    fn an_exec_analyze_records_an_oversized_artifact_as_advisory() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: clarified\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[artifacts]\nread-size-bytes = 100\n",
        )
        .unwrap();
        std::fs::write(dir.join("plan.md"), "x".repeat(250)).unwrap();
        let step = |n: u32, name: &str| Step::Primitive {
            number: StepNumber(vec![n]),
            name: name.into(),
            prose: String::new(),
            location: loc(),
        };
        let procedure = Procedure {
            command: "analyze".into(),
            steps: vec![step(17, "check-artifact-size"), step(21, "write-analysis")],
        };
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("path".into(), Value::String("specs/001-x".into()));
        context.insert(
            "analyzed-at".into(),
            Value::String("2026-10-04T00:00:00Z".into()),
        );
        context.insert("analyzed-against".into(), Value::String("abc1234".into()));
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            tmp.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        );
        assert_eq!(walker.run().unwrap(), WalkOutcome::Complete);
        let analysis = std::fs::read_to_string(dir.join("analysis.md")).unwrap();
        assert!(analysis.contains("\nadvisory: 1\n"), "{analysis}");
        assert!(analysis.contains("\nblocking: false\n"), "{analysis}");
        assert!(analysis.contains("  undispositioned: 1\n"), "{analysis}");
    }

    /// A tempdir holding a `clarified` spec at `specs/001-x/spec.md` and, when
    /// `rules` is given, `framework/rules/quality-cross.md` carrying it.
    fn analyze_repo(rules: Option<&str>) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: clarified\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();
        if let Some(rules) = rules {
            let rules_dir = tmp.path().join("framework/rules");
            std::fs::create_dir_all(&rules_dir).unwrap();
            std::fs::write(rules_dir.join("quality-cross.md"), rules).unwrap();
        }
        tmp
    }

    /// An `assessSpecQuality` step, as `/analyze` steps 11 and 12 parse.
    fn assess_step(n: u32, prose: &str) -> Step {
        Step::Extension {
            number: StepNumber(vec![n]),
            identifier: "assessSpecQuality".into(),
            prose: prose.into(),
            location: loc(),
        }
    }

    fn assess_tier(n: u32, tier: &str) -> Step {
        assess_step(
            n,
            &format!("For every loaded {tier}-tier rule, request a semantic assessment."),
        )
    }

    /// An `/analyze` procedure of `steps` closed by `write-analysis`.
    fn analyze_procedure(mut steps: Vec<Step>) -> Procedure {
        steps.push(Step::Primitive {
            number: StepNumber(vec![20]),
            name: "write-analysis".into(),
            prose: String::new(),
            location: loc(),
        });
        Procedure {
            command: "analyze".into(),
            steps,
        }
    }

    /// Walk `procedure` over `repo` targeting `001-x`, answering from
    /// `responses`; returns the outcome and every envelope emitted.
    fn walk_analyze(
        procedure: &Procedure,
        repo: &Path,
        rule_files: Option<&[&str]>,
        responses: &str,
    ) -> (WalkOutcome, Vec<Value>) {
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("path".into(), Value::String("specs/001-x".into()));
        context.insert(
            "analyzed-at".into(),
            Value::String("2026-09-27T00:00:00Z".into()),
        );
        context.insert("analyzed-against".into(), Value::String("abc1234".into()));
        if let Some(files) = rule_files {
            context.insert(
                "rule-files".into(),
                Value::Array(files.iter().map(|f| Value::String((*f).into())).collect()),
            );
        }
        let mut reader = Cursor::new(responses.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let outcome = Walker::new(
            procedure,
            repo.to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        )
        .run()
        .unwrap();
        let envelopes = String::from_utf8(writer)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        (outcome, envelopes)
    }

    /// One scripted `llm-response` line.
    fn llm_response(n: u32, response: &str) -> String {
        format!(
            "{{\"type\":\"llm-response\",\"request-id\":\"req-{n}\",\"response\":{response}}}\n"
        )
    }

    /// A failed assessment carrying a finding of `severity`.
    fn failed_with(severity: &str, rule: &str) -> String {
        format!(
            "{{\"passed\":false,\"finding\":{{\"severity\":\"{severity}\",\"rule-id\":\"{rule}\",\"location\":{{\"section\":\"x\",\"line\":1}},\"message\":\"m\"}}}}"
        )
    }

    fn requests(envelopes: &[Value]) -> Vec<&Value> {
        envelopes
            .iter()
            .filter(|envelope| envelope["type"] == "llm-request")
            .collect()
    }

    fn progress_at<'a>(envelopes: &'a [Value], step: &str) -> Vec<&'a str> {
        envelopes
            .iter()
            .filter(|envelope| envelope["type"] == "progress" && envelope["step"] == step)
            .filter_map(|envelope| envelope["message"].as_str())
            .collect()
    }

    /// A rule file exercising every way a rule is read: one of each tier, one
    /// carrying both keywords, one carrying neither, and one with no
    /// Verification.
    const EVERY_KIND_OF_RULE: &str = "# Quality rules\n\n## TS-A — Test rules\n\n\
        ### TS-MUST-001\n\n> Specs MUST name their constants.\n\n**Verification:** the spec names its constants.\n\n\
        ### TS-SHOULD-001\n\n> Specs SHOULD name an owner.\n\n**Verification:** the spec names an owner.\n\n\
        ### TS-BOTH-001\n\n> Lists MUST paginate and SHOULD use cursors.\n\n**Verification:** the spec paginates its lists.\n\n\
        ### TS-NONE-001\n\n> Constants live in one module.\n\n**Verification:** the spec centralizes constants.\n\n\
        ### TS-NOVER-001\n\n> Rules MUST carry a Verification.\n\n**Rationale:** none given.\n";

    /// Steps 11 and 12 ask about each loaded rule once, in the tier its
    /// Statement carries (spec 060, AC1, AC2, AC4, AC5, AC7). The MUST rule
    /// and the rule carrying both keywords are asked at step 11, the SHOULD
    /// rule at step 12, and nothing twice. A finding counts in its own tier,
    /// so the mixed rule's SHOULD-clause failure is advisory, and a failure
    /// with no finding counts in the asked rule's tier. The rule with no
    /// keyword, the rule with no Verification, and the rule file that does
    /// not exist are recorded unexamined, never asked about as nothing.
    #[test]
    fn an_exec_analyze_asks_about_each_loaded_rule_once_in_its_own_tier() {
        let repo = analyze_repo(Some(EVERY_KIND_OF_RULE));
        let procedure = analyze_procedure(vec![assess_tier(11, "MUST"), assess_tier(12, "SHOULD")]);
        let responses = [
            llm_response(1, &failed_with("must", "TS-MUST-001")),
            llm_response(2, &failed_with("should", "TS-BOTH-001")),
            llm_response(3, "{\"passed\":false}"),
        ]
        .concat();
        let (outcome, envelopes) = walk_analyze(
            &procedure,
            repo.path(),
            Some(&[
                "framework/rules/quality-cross.md",
                "framework/rules/missing.md",
            ]),
            &responses,
        );
        assert_eq!(outcome, WalkOutcome::Complete);
        let asked: Vec<(&str, &str)> = requests(&envelopes)
            .iter()
            .map(|envelope| {
                (
                    envelope["request"]["rule"]["id"].as_str().unwrap(),
                    envelope["request"]["rule"]["severity"].as_str().unwrap(),
                )
            })
            .collect();
        assert_eq!(
            asked,
            [
                ("TS-MUST-001", "must"),
                ("TS-BOTH-001", "must"),
                ("TS-SHOULD-001", "should"),
            ]
        );
        for request in requests(&envelopes) {
            assert_eq!(request["request"]["spec-path"], "specs/001-x/spec.md");
            assert_ne!(
                request["request"]["rule"]["verification"].as_str().unwrap(),
                ""
            );
        }
        let analysis =
            std::fs::read_to_string(repo.path().join("specs/001-x/analysis.md")).unwrap();
        assert!(analysis.contains("\nblocking-findings: 1\n"), "{analysis}");
        assert!(analysis.contains("\nadvisory: 2\n"), "{analysis}");
        assert!(
            analysis.contains("  rule-assessments-not-checked: 2\n"),
            "{analysis}"
        );
        assert!(
            analysis.contains("  rule-file-unreadable: 1\n"),
            "{analysis}"
        );
    }

    /// A response that fails validation partway through a step's rules halts
    /// the walk before `write-analysis`, so no record is written — never a
    /// partial one counting only the rules answered so far.
    #[test]
    fn a_malformed_response_partway_through_the_rules_writes_no_record() {
        let repo = analyze_repo(Some(
            "### TS-MUST-001\n\n> A MUST.\n\n**Verification:** one.\n\n\
             ### TS-MUST-002\n\n> Another MUST.\n\n**Verification:** two.\n",
        ));
        let procedure = analyze_procedure(vec![assess_tier(11, "MUST")]);
        let responses = [
            llm_response(1, "{\"passed\":true}"),
            llm_response(2, "{\"passed\":\"no\"}"),
        ]
        .concat();
        let (outcome, _) = walk_analyze(&procedure, repo.path(), None, &responses);
        assert!(
            matches!(&outcome, WalkOutcome::Errored { code, .. } if code == "schema-mismatch"),
            "{outcome:?}"
        );
        assert!(!repo.path().join("specs/001-x/analysis.md").exists());
    }

    /// With no rule loaded, steps 11 and 12 send no request and each records
    /// one target unexamined, and the stream says so at each step rather than
    /// passing over it in silence.
    #[test]
    fn with_no_rule_loaded_each_step_asks_nothing_and_records_itself() {
        let repo = analyze_repo(None);
        let procedure = analyze_procedure(vec![assess_tier(11, "MUST"), assess_tier(12, "SHOULD")]);
        let (outcome, envelopes) = walk_analyze(&procedure, repo.path(), None, "");
        assert_eq!(outcome, WalkOutcome::Complete);
        assert!(requests(&envelopes).is_empty(), "{envelopes:?}");
        for step in ["11", "12"] {
            assert!(
                progress_at(&envelopes, step)
                    .iter()
                    .any(|message| message.starts_with("no rule loaded")),
                "step {step}: {envelopes:?}"
            );
        }
        let analysis =
            std::fs::read_to_string(repo.path().join("specs/001-x/analysis.md")).unwrap();
        assert!(
            analysis.contains("  rule-assessments-not-checked: 2\n"),
            "{analysis}"
        );
    }

    /// A tier with no loaded rule, beside one that has rules, records nothing:
    /// every loaded rule was asked about. A step whose prose names no MUST or
    /// SHOULD tier cannot know which rules it covers, so it asks about none
    /// and records itself unexamined.
    #[test]
    fn an_empty_tier_records_nothing_and_a_step_naming_no_tier_is_unexamined() {
        let repo = analyze_repo(Some(
            "### TS-MUST-001\n\n> A MUST.\n\n**Verification:** one.\n",
        ));
        let procedure = analyze_procedure(vec![
            assess_tier(11, "MUST"),
            assess_tier(12, "SHOULD"),
            assess_step(13, "For every loaded rule, request a semantic assessment."),
        ]);
        let (outcome, envelopes) = walk_analyze(
            &procedure,
            repo.path(),
            None,
            &llm_response(1, "{\"passed\":true}"),
        );
        assert_eq!(outcome, WalkOutcome::Complete);
        assert_eq!(requests(&envelopes).len(), 1, "{envelopes:?}");
        assert!(
            progress_at(&envelopes, "12")
                .iter()
                .any(|message| message.starts_with("no SHOULD-tier rule")),
            "{envelopes:?}"
        );
        assert!(!progress_at(&envelopes, "13").is_empty(), "{envelopes:?}");
        let analysis =
            std::fs::read_to_string(repo.path().join("specs/001-x/analysis.md")).unwrap();
        assert!(
            analysis.contains("  rule-assessments-not-checked: 1\n"),
            "{analysis}"
        );
        assert!(analysis.contains("\nblocking-findings: 0\n"), "{analysis}");
    }

    /// Walk one `askClarifyQuestion` step over the `analyze_repo` spec with
    /// `open-questions` seeded as given (absent when `None`), answering from
    /// `responses`.
    fn walk_clarify(questions: Option<Value>, responses: &str) -> (WalkOutcome, Vec<Value>) {
        let repo = analyze_repo(None);
        let procedure = Procedure {
            command: "clarify".into(),
            steps: vec![Step::Extension {
                number: StepNumber(vec![6]),
                identifier: "askClarifyQuestion".into(),
                prose: "Resolve open questions one at a time.".into(),
                location: loc(),
            }],
        };
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("path".into(), Value::String("specs/001-x".into()));
        if let Some(questions) = questions {
            context.insert("open-questions".into(), questions);
        }
        let mut reader = Cursor::new(responses.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let outcome = Walker::new(
            &procedure,
            repo.path().to_path_buf(),
            context,
            &mut reader,
            &mut writer,
        )
        .run()
        .unwrap();
        let envelopes = String::from_utf8(writer)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        (outcome, envelopes)
    }

    /// Clarify step 6 is one round trip per open question (spec 022,
    /// scenario `exec-clarify-asks-each-open-question`): each question
    /// `read-spec` returned is asked, in its order, never only the first.
    #[test]
    fn an_exec_clarify_asks_each_open_question_in_order() {
        let responses = [
            llm_response(1, "{\"answer\":\"first\"}"),
            llm_response(2, "{\"answer\":\"second\"}"),
        ]
        .concat();
        let (outcome, envelopes) = walk_clarify(
            Some(serde_json::json!([{ "text": "Which store?" }, { "text": "What limit?" }])),
            &responses,
        );
        assert_eq!(outcome, WalkOutcome::Complete);
        let asked: Vec<&str> = requests(&envelopes)
            .iter()
            .map(|envelope| envelope["request"]["question"]["text"].as_str().unwrap())
            .collect();
        assert_eq!(asked, ["Which store?", "What limit?"]);
        assert_eq!(progress_at(&envelopes, "6").len(), 2, "{envelopes:?}");
    }

    /// With every question resolved there is nothing to ask: the step sends
    /// no request and says so, rather than asking about an empty question.
    #[test]
    fn an_exec_clarify_with_no_open_question_asks_nothing() {
        let (outcome, envelopes) = walk_clarify(Some(serde_json::json!([])), "");
        assert_eq!(outcome, WalkOutcome::Complete);
        assert!(requests(&envelopes).is_empty(), "{envelopes:?}");
        assert!(
            progress_at(&envelopes, "6")
                .iter()
                .any(|message| message.starts_with("no open question")),
            "{envelopes:?}"
        );
    }

    /// With no question list in the context the walker cannot know the
    /// questions, so it sends the single request the builder has always
    /// built rather than asking nothing.
    #[test]
    fn an_exec_clarify_with_no_question_list_sends_the_single_request() {
        let (outcome, envelopes) = walk_clarify(None, &llm_response(1, "{\"answer\":\"a\"}"));
        assert_eq!(outcome, WalkOutcome::Complete);
        let sent = requests(&envelopes);
        assert_eq!(sent.len(), 1, "{envelopes:?}");
        assert_eq!(sent[0]["request"]["question"]["text"], "");
    }

    /// A session `write-session` wrote names the target alone, so an analyze
    /// walk binds the two list seeds its steps need: every rule file in the
    /// rule-file directory, whatever the project's surfaces, and the feature
    /// directory's markdown files.
    #[test]
    fn an_analyze_walk_derives_the_list_seeds_a_session_cannot_carry() {
        let tmp = tempfile::tempdir().unwrap();
        let rules = tmp.path().join("framework/rules");
        std::fs::create_dir_all(&rules).unwrap();
        for name in ["security-frontend.md", "quality-cross.md", "notes.txt"] {
            std::fs::write(rules.join(name), "# rules\n").unwrap();
        }
        std::fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[rules]\nsurfaces = [\"backend\"]\n",
        )
        .unwrap();
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("path".into(), Value::String("specs/001-x".into()));
        derive_analyze_seeds(&mut context, tmp.path());
        assert_eq!(
            context["rule-files"],
            serde_json::json!([
                "framework/rules/quality-cross.md",
                "framework/rules/security-frontend.md"
            ])
        );
        assert_eq!(context["paths"], serde_json::json!(["specs/001-x/**/*.md"]));
    }

    /// A seeded list is the caller's, so it is used as given — which is why the
    /// `analyze-basic` fixture, whose session seeds both, walks unchanged.
    #[test]
    fn seeded_analyze_lists_are_used_as_given() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
        std::fs::write(tmp.path().join("framework/rules/a.md"), "# a\n").unwrap();
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert("rule-files".into(), serde_json::json!(["mine.md"]));
        context.insert("paths".into(), serde_json::json!(["specs/001-x/spec.md"]));
        let seeded = context.clone();
        derive_analyze_seeds(&mut context, tmp.path());
        assert_eq!(context, seeded);
    }

    /// An absent or empty rule-file directory binds an empty list, so step 5
    /// reads no rule file and records the spec's citations unexamined.
    #[test]
    fn an_absent_or_empty_rule_directory_binds_no_rule_files() {
        let tmp = tempfile::tempdir().unwrap();
        let mut absent = Map::new();
        derive_analyze_seeds(&mut absent, tmp.path());
        assert_eq!(absent["rule-files"], serde_json::json!([]));

        std::fs::create_dir_all(tmp.path().join("framework/rules")).unwrap();
        let mut empty = Map::new();
        derive_analyze_seeds(&mut empty, tmp.path());
        assert_eq!(empty["rule-files"], serde_json::json!([]));
    }

    /// The lint subject is a directory inside the spec root; a `feature` that
    /// climbs out of it names none, so none is bound.
    #[test]
    fn a_feature_outside_the_spec_root_binds_no_lint_subject() {
        let tmp = tempfile::tempdir().unwrap();
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("../elsewhere".into()));
        derive_analyze_seeds(&mut context, tmp.path());
        assert!(!context.contains_key("paths"), "{context:?}");
    }

    /// `write-analysis` binds no `findings` from the walker context: that key
    /// belongs to `validate-frontmatter`, `check-artifacts` and
    /// `check-orphaned-references`, whose entries are not analyze findings.
    /// Before this, one orphaned reference made an exec `/analyze` fail at the
    /// record step (spec 058).
    #[test]
    fn write_analysis_binds_no_other_primitives_findings() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/001-x");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# x\n",
        )
        .unwrap();
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("001-x".into()));
        context.insert(
            "analyzed-at".into(),
            Value::String("2026-09-25T00:00:00Z".into()),
        );
        context.insert("analyzed-against".into(), Value::String("abc1234".into()));
        context.insert(
            "findings".into(),
            serde_json::json!([{
                "referrer": "CLAUDE.md",
                "target": ".ductus/missing.md",
                "line": 3,
                "migration": ""
            }]),
        );
        dispatch_primitive("write-analysis", &context, tmp.path()).unwrap();
        assert!(dir.join("analysis.md").is_file());
    }

    #[test]
    fn malformed_llm_response_emits_schema_mismatch_error() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Extension {
                number: StepNumber(vec![1]),
                identifier: "askClarifyQuestion".into(),
                prose: String::new(),
                location: loc(),
            }],
        };
        // Missing required `answer` field.
        let response =
            "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"reply\":null}}\n";
        let mut reader = Cursor::new(response.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        match outcome {
            WalkOutcome::Errored { code, .. } => assert_eq!(code, "schema-mismatch"),
            WalkOutcome::Complete => panic!("expected Errored, got Complete"),
        }
        let envelopes: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        // llm-request then error — no progress.
        assert_eq!(envelopes.len(), 2);
        assert_eq!(envelopes[0]["type"], "llm-request");
        assert_eq!(envelopes[1]["type"], "error");
        assert_eq!(envelopes[1]["code"], "schema-mismatch");
    }

    #[test]
    fn out_of_boundary_write_code_edit_emits_error() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Extension {
                number: StepNumber(vec![1]),
                identifier: "writeCode".into(),
                prose: String::new(),
                location: loc(),
            }],
        };
        // Schema-valid writeCode response with an edit outside the boundary.
        let response = "{\"type\":\"llm-response\",\"request-id\":\"req-1\",\"response\":{\"edits\":[{\"path\":\"framework/constitution.md\",\"action\":\"edit\",\"content\":\"malicious\"}],\"summary\":\"x\"}}\n";
        let mut reader = Cursor::new(response.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut context = Map::new();
        context.insert(
            "write-boundary".into(),
            Value::Array(vec![Value::String("runtime/**".into())]),
        );
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            context,
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        match outcome {
            WalkOutcome::Errored { code, .. } => assert_eq!(code, "out-of-boundary-edit"),
            WalkOutcome::Complete => panic!("expected Errored, got Complete"),
        }
        let envelopes: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(envelopes.last().unwrap()["code"], "out-of-boundary-edit");
        assert!(
            envelopes.last().unwrap()["message"]
                .as_str()
                .unwrap()
                .contains("framework/constitution.md")
        );
    }

    #[test]
    fn derive_boundary_result_unions_into_write_boundary() {
        // Scenario writecode-boundary-derivation: the derived boundary must
        // feed the enforcement key as a UNION with the seeded grant — the
        // seed is never revoked, the derived zones are added, and the
        // result is sorted for deterministic payloads.
        let procedure = Procedure {
            command: "implement".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut context = Map::new();
        context.insert(
            "write-boundary".into(),
            Value::Array(vec![Value::String("scripts/**".into())]),
        );
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            context,
            &mut reader,
            &mut writer,
        );
        walker.merge_primitive_result(
            "derive-boundary",
            serde_json::json!({
                "boundary": ["specs/004-implement/**", "runtime/src/**"],
                "first-commit": "abc",
                "current-head": "def",
            }),
        );
        assert_eq!(
            walker.context.get("write-boundary"),
            Some(&serde_json::json!([
                "runtime/src/**",
                "scripts/**",
                "specs/004-implement/**"
            ])),
            "seeded grant kept, derived zones added, sorted"
        );
        // The seeded-key guard still blocks a non-derive-boundary result
        // from touching the enforcement key.
        walker.merge_primitive_result(
            "read-spec",
            serde_json::json!({ "write-boundary": ["everything/**"] }),
        );
        assert_eq!(
            walker.context.get("write-boundary"),
            Some(&serde_json::json!([
                "runtime/src/**",
                "scripts/**",
                "specs/004-implement/**"
            ]))
        );
    }

    #[test]
    fn uncommitted_spec_dir_derivation_leaves_the_seeded_grant_intact() {
        // Scenario derive-boundary-uncommitted-spec-dir: on an uncommitted
        // spec dir the derivation carries the spec glob alone plus guidance.
        // The union must leave a seeded grant standing — that seed is the
        // documented escape hatch the guidance string points the operator at,
        // so it is what admits the walk when history cannot supply a zone.
        let procedure = Procedure {
            command: "implement".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut context = Map::new();
        context.insert(
            "write-boundary".into(),
            Value::Array(vec![Value::String("runtime/**".into())]),
        );
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            context,
            &mut reader,
            &mut writer,
        );
        walker.merge_primitive_result(
            "derive-boundary",
            serde_json::json!({
                "boundary": ["specs/004-implement/**"],
                "first-commit": "",
                "current-head": "def",
                "guidance": "commit the spec directory, or seed a write-boundary in the session",
            }),
        );
        assert_eq!(
            walker.context.get("write-boundary"),
            Some(&serde_json::json!(["runtime/**", "specs/004-implement/**"])),
            "seed survives an empty derivation"
        );

        // With no seed at all the same result is fail-closed: the feature's
        // own zone and nothing else, so the first out-of-spec edit halts.
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        walker.merge_primitive_result(
            "derive-boundary",
            serde_json::json!({
                "boundary": ["specs/004-implement/**"],
                "first-commit": "",
                "current-head": "def",
                "guidance": "commit the spec directory, or seed a write-boundary in the session",
            }),
        );
        assert_eq!(
            walker.context.get("write-boundary"),
            Some(&serde_json::json!(["specs/004-implement/**"])),
            "fail-closed without a seed"
        );
    }

    #[test]
    fn derive_boundary_result_populates_unseeded_write_boundary() {
        let procedure = Procedure {
            command: "implement".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        walker.merge_primitive_result(
            "derive-boundary",
            serde_json::json!({ "boundary": ["specs/004-implement/**"] }),
        );
        assert_eq!(
            walker.context.get("write-boundary"),
            Some(&serde_json::json!(["specs/004-implement/**"])),
            "with no seed, enforcement runs on the derivation alone"
        );
    }

    #[test]
    fn gate_trigger_in_prose_emits_gate_confirm_and_resumes_on_confirmed() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Prose {
                number: StepNumber(vec![1]),
                text: "Ask the user to approve the transition.".into(),
                location: loc(),
            }],
        };
        let response = "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":true}\n";
        let mut reader = Cursor::new(response.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        // gate-confirm, progress(confirmed), complete
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0]["type"], "gate-confirm");
        assert_eq!(lines[2]["type"], "complete");
    }

    #[test]
    fn gate_denial_exits_cleanly_with_confirmed_false() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Prose {
                number: StepNumber(vec![1]),
                text: "Ask the user to approve the destructive op.".into(),
                location: loc(),
            }],
        };
        let response =
            "{\"type\":\"gate-response\",\"request-id\":\"req-1\",\"confirmed\":false}\n";
        let mut reader = Cursor::new(response.to_string());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<Value> = std::str::from_utf8(&writer)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        // gate-confirm, progress(denied), complete(confirmed: false)
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2]["type"], "complete");
        assert_eq!(lines[2]["result"]["confirmed"], false);
    }

    #[test]
    fn prose_step_is_noop() {
        let procedure = Procedure {
            command: "test".into(),
            steps: vec![Step::Prose {
                number: StepNumber(vec![1]),
                text: "Do the thing.".into(),
                location: loc(),
            }],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = Walker::new(
            &procedure,
            fixture_repo(),
            Map::new(),
            &mut reader,
            &mut writer,
        );
        let outcome = walker.run().unwrap();
        assert_eq!(outcome, WalkOutcome::Complete);
        let lines: Vec<&str> = std::str::from_utf8(&writer).unwrap().lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("\"complete\""));
    }

    // --- FIX 1: create-feature retargets the seeded session target ----------

    fn seeded_target_walker<'a, R: BufRead, W: Write>(
        procedure: &'a Procedure,
        reader: &'a mut R,
        writer: &'a mut W,
    ) -> Walker<'a, R, W> {
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("006-specify".into()));
        context.insert("path".into(), Value::String("specs/006-specify".into()));
        Walker::new(procedure, fixture_repo(), context, reader, writer)
    }

    #[test]
    fn create_feature_success_overrides_seeded_feature_and_path() {
        let procedure = Procedure {
            command: "specify".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = seeded_target_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "create-feature",
            serde_json::json!({
                "created": true,
                "feature": "007-webhook-delivery",
                "path": "specs/007-webhook-delivery",
                "template": "specs/templates/spec.md",
            }),
        );
        // The just-created feature retargets the session so the later
        // write-session binds the NEW target, not the stale seed.
        assert_eq!(walker.context["feature"], "007-webhook-delivery");
        assert_eq!(walker.context["path"], "specs/007-webhook-delivery");
        assert_eq!(walker.context["created"], Value::Bool(true));
    }

    /// A walker with an EMPTY seed — the fresh-repo case, where
    /// `.ductus/session.toml` does not exist yet (or carries no target), so
    /// neither `feature` nor `path` is a seeded key.
    fn unseeded_walker<'a, R: BufRead, W: Write>(
        procedure: &'a Procedure,
        reader: &'a mut R,
        writer: &'a mut W,
    ) -> Walker<'a, R, W> {
        Walker::new(procedure, fixture_repo(), Map::new(), reader, writer)
    }

    #[test]
    fn a_spec_file_path_never_overwrites_the_retargeted_session_path() {
        // `/ductus:specify` runs create-feature → label-criteria → write-session.
        // create-feature's `path` is the spec DIRECTORY the session target
        // holds; label-criteria's is the spec FILE it labelled. With no
        // session file to seed the key, the general merge policy let the file
        // win and write-session recorded `specs/007-.../spec.md` as the
        // target. `/ductus:target` has read-spec sitting in the same gap.
        let procedure = Procedure {
            command: "specify".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = unseeded_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "create-feature",
            serde_json::json!({
                "created": true,
                "feature": "007-webhook-delivery",
                "path": "specs/007-webhook-delivery",
            }),
        );
        walker.merge_primitive_result(
            "label-criteria",
            serde_json::json!({
                "assigned": [],
                "next-criterion": 1,
                "path": "specs/007-webhook-delivery/spec.md",
                "changed": false,
            }),
        );

        assert_eq!(walker.context["path"], "specs/007-webhook-delivery");
        assert_eq!(walker.context["feature"], "007-webhook-delivery");
        // Everything else the primitive reported still lands — pinning is
        // scoped to the two target keys, not to the whole result.
        assert_eq!(walker.context["next-criterion"], 1);
        assert_eq!(walker.context["changed"], Value::Bool(false));
    }

    #[test]
    fn a_second_retarget_still_overrides_the_pinned_target() {
        // Pinning must not lock out the retargeting primitives themselves,
        // or a walk that resolves a feature twice would write the first one.
        let procedure = Procedure {
            command: "target".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = unseeded_walker(&procedure, &mut reader, &mut writer);
        for slug in ["006-specify", "007-webhook-delivery"] {
            walker.merge_primitive_result(
                "resolve-feature",
                serde_json::json!({
                    "outcome": "resolved",
                    "feature": slug,
                    "path": format!("specs/{slug}"),
                }),
            );
        }
        assert_eq!(walker.context["path"], "specs/007-webhook-delivery");
        assert_eq!(walker.context["feature"], "007-webhook-delivery");
    }

    #[test]
    fn create_feature_refusal_leaves_seeded_target_intact() {
        // A `created: false` refusal (directory collision) must NOT override
        // the seeded target — nothing was scaffolded to retarget to.
        let procedure = Procedure {
            command: "specify".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = seeded_target_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "create-feature",
            serde_json::json!({
                "created": false,
                "feature": "006-specify",
                "path": "specs/006-specify",
            }),
        );
        assert_eq!(walker.context["feature"], "006-specify");
        assert_eq!(walker.context["path"], "specs/006-specify");
    }

    #[test]
    fn other_primitive_result_never_overrides_seeded_target() {
        // The override is keyed on the create-feature name alone: a
        // resolve-feature result echoing `feature`/`path` must obey the
        // general seeded-key guard.
        let procedure = Procedure {
            command: "analyze".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = seeded_target_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "resolve-feature",
            serde_json::json!({
                "outcome": "resolved",
                "feature": "999-other",
                "path": "specs/999-other",
            }),
        );
        assert_eq!(walker.context["feature"], "006-specify");
        assert_eq!(walker.context["path"], "specs/006-specify");
        // A non-seeded key from the same result still merges.
        assert_eq!(walker.context["outcome"], "resolved");
    }

    #[test]
    fn target_resolve_feature_overrides_seeded_target() {
        // `/ductus:target <feature>` against a repo whose session already names
        // a different feature: resolve-feature's resolved `feature`/`path`
        // must override the seed so write-session persists the NEW target,
        // not the stale one.
        let procedure = Procedure {
            command: "target".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = seeded_target_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "resolve-feature",
            serde_json::json!({
                "outcome": "resolved",
                "feature": "999-other",
                "path": "specs/999-other",
            }),
        );
        assert_eq!(walker.context["feature"], "999-other");
        assert_eq!(walker.context["path"], "specs/999-other");
    }

    #[test]
    fn target_resolve_feature_ambiguous_keeps_seeded_target() {
        // Only a `resolved` outcome retargets; an `ambiguous`/`not-found`
        // resolve-feature result carries no feature/path and must not disturb
        // the seeded target.
        let procedure = Procedure {
            command: "target".into(),
            steps: vec![],
        };
        let mut reader = Cursor::new(String::new());
        let mut writer: Vec<u8> = Vec::new();
        let mut walker = seeded_target_walker(&procedure, &mut reader, &mut writer);
        walker.merge_primitive_result(
            "resolve-feature",
            serde_json::json!({ "outcome": "ambiguous", "candidates": ["006-a", "006-b"] }),
        );
        assert_eq!(walker.context["feature"], "006-specify");
        assert_eq!(walker.context["path"], "specs/006-specify");
    }

    // --- FIX 2: verifyCriteria verdict gates the mark-criterion flip --------

    #[test]
    fn criterion_verified_met_reads_verdict_for_seeded_index() {
        let verify = serde_json::json!({
            "results": [
                { "index": 0, "met": true },
                { "index": 1, "met": false },
            ]
        });
        let idx = |n: u64| Value::from(n);
        assert!(criterion_verified_met(&verify, Some(&idx(0))));
        assert!(!criterion_verified_met(&verify, Some(&idx(1))));
        // Absent verdict for the index → not met.
        assert!(!criterion_verified_met(&verify, Some(&idx(2))));
        // Missing index argument → not met.
        assert!(!criterion_verified_met(&verify, None));
        // No `results` array → not met.
        assert!(!criterion_verified_met(
            &serde_json::json!({}),
            Some(&idx(0))
        ));
    }

    fn spec_repo_with_one_unchecked_criterion() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("specs/feat");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.md"),
            "---\nstatus: in-progress\ndependencies: []\n---\n\n# feat\n\n## Acceptance Criteria\n\n- [ ] Only criterion.\n",
        )
        .unwrap();
        tmp
    }

    fn mark_criterion_context(verify: Option<Value>) -> Map<String, Value> {
        let mut context = Map::new();
        context.insert("feature".into(), Value::String("feat".into()));
        context.insert("criterion-index".into(), Value::from(0u64));
        context.insert("checked".into(), Value::Bool(true));
        if let Some(verify) = verify {
            context.insert("llm:verifyCriteria".into(), verify);
        }
        context
    }

    #[test]
    fn mark_criterion_skips_flip_when_verdict_not_met() {
        let tmp = spec_repo_with_one_unchecked_criterion();
        let context = mark_criterion_context(Some(serde_json::json!({
            "results": [ { "index": 0, "met": false } ]
        })));
        let result = dispatch_primitive("mark-criterion", &context, tmp.path()).unwrap();
        assert_eq!(
            result["current"],
            Value::Bool(false),
            "an unconfirmed criterion is left unchecked despite the seeded checked:true"
        );
        let on_disk = std::fs::read_to_string(tmp.path().join("specs/feat/spec.md")).unwrap();
        assert!(
            on_disk.contains("- [ ] Only criterion."),
            "checkbox stays unchecked: {on_disk}"
        );
    }

    #[test]
    fn mark_criterion_flips_when_verdict_met() {
        let tmp = spec_repo_with_one_unchecked_criterion();
        let context = mark_criterion_context(Some(serde_json::json!({
            "results": [ { "index": 0, "met": true } ]
        })));
        let result = dispatch_primitive("mark-criterion", &context, tmp.path()).unwrap();
        assert_eq!(result["current"], Value::Bool(true));
        let on_disk = std::fs::read_to_string(tmp.path().join("specs/feat/spec.md")).unwrap();
        assert!(on_disk.contains("- [x] Only criterion."), "{on_disk}");
    }

    #[test]
    fn mark_criterion_without_verify_response_honors_seeded_checked() {
        // No verifyCriteria response present → the seeded `checked` stands,
        // so direct MCP/CLI callers and other commands are unaffected.
        let tmp = spec_repo_with_one_unchecked_criterion();
        let context = mark_criterion_context(None);
        let result = dispatch_primitive("mark-criterion", &context, tmp.path()).unwrap();
        assert_eq!(result["current"], Value::Bool(true));
    }
}
