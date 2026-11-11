//! `merge-permissions` — idempotently install or update a canonical
//! permission allow/deny set into a JSON file, removing exact-match
//! duplicates from each array. The destination path is host-supplied
//! (typically the bootstrap-substituted `{cli-config-dir}/settings.local.json`,
//! e.g. `.claude/settings.local.json` on Claude Code); no default — `path` is required.
//!
//! The primitive is the deterministic surface `/configure` calls; see
//! spec 022's `framework-list-dedup` scenario for the contract.
//!
//! Behavior summary:
//!
//! - **File does not exist** → write
//!   `{ "permissions": { "allow": [...canonical], "deny": [...canonical] } }`
//!   and emit `created`.
//! - **File exists, parses as JSON** → dedup exact-match entries in
//!   `permissions.allow` and `permissions.deny`, remove any `revoke`
//!   entry from `permissions.allow`, then ensure every canonical entry
//!   is present (append at end, preserving prior order). If the
//!   post-merge value equals the pre-merge value structurally, emit
//!   `unchanged` and skip the write (preserves mtime for build-tool
//!   idempotency).
//! - **File exists, malformed JSON** → return
//!   [`PrimitiveError::Json`]; do not write.
//! - **`permissions.allow` / `permissions.deny` field exists but is
//!   not an array** → return [`PrimitiveError::JsonSchema`]; do not
//!   write.
//!
//! Atomic writes use the project-wide tempfile + rename helper. Field
//! order under `permissions` follows insertion order via
//! `serde_json`'s `preserve_order` feature.
//!
//! # Retirement (`revoke`)
//!
//! `merge-permissions` installs and dedups; on its own it never removes
//! a non-canonical entry an adopter owns. That is deliberate, and it is
//! also why an entry the framework *once shipped* and has since retired
//! survives in every adopter tree indefinitely — the removal has no
//! owner. `revoke` gives it one: the caller passes the explicit list of
//! formerly-canonical entries, and only exact matches from that list are
//! removed. Shape-matching is not offered, so an adopter who authored
//! their own copy of a retired pattern keeps it.
//!
//! Retirement is **allow-side only**, enforced by the absence of a
//! deny-side counterpart rather than by a documented convention: an
//! over-broad deny entry refuses more rather than approving more, so
//! sweeping both arrays under one argument would invite narrowing the
//! deny set into holes. See spec 027's `retired-permission-entry-cleanup`
//! scenario.
//!
//! # Ceiling
//!
//! An `allow` entry outside Claude's canonical allow set is refused before
//! the settings file is read ([`canonical_allow`]), so the primitive grants nothing
//! `/configure` does not (BE-AUTHZ-004; spec 022 task 11, the owner's answer
//! (a)). A host auto-approves the runtime's tools, and the agent calling one
//! could otherwise write any rule into the file its own writes are prompted
//! for. `deny` and `revoke` need no ceiling: each only takes a grant away.
//!
//! # Canonical sets (`canonical`)
//!
//! With `canonical` set, the call merges Claude's canonical allow, deny and
//! retired sets itself ([`canonical_allow_ordered`], [`CANONICAL_DENY`],
//! [`RETIRED_ALLOW`]), beside any entries the caller passes, and ensures
//! `permissions.additionalDirectories` holds the spec root and the project's
//! command directory as absolute paths. `/{project}:configure` on Claude Code
//! then types no entry and writes the file through no host tool (spec 080,
//! scenario `configure-types-no-permission-entries`). Steps 2–4 of
//! `framework/bootstrap/configure/claude.md` stay the reviewed record of the
//! three sets, and tests hold each constant equal to its step.

#![allow(clippy::expect_used)]

use std::path::Path;

use serde_json::{Map, Value, json};

use crate::host::Host;
use crate::primitives::{PrimitiveError, Result, read_text, write_atomic};
use crate::schema::paths::Paths;
use crate::schema::primitives::{MergePermissionsArgs, MergePermissionsResult};

/// Execute the `merge-permissions` primitive.
///
/// # Errors
///
/// - [`PrimitiveError::InvalidArgument`] when an `allow` entry is outside
///   Claude's canonical allow set ([`canonical_allow`], the ceiling).
///   Checked before the settings file is read or written, after the
///   `revoke` overlap check below, so a retired entry passed in `allow` is named as the overlap.
/// - [`PrimitiveError::InvalidPath`] when `path` is absolute, empty, or
///   contains a parent-directory component, or when a symlink on it
///   resolves outside the project — the BE-INPUT-004 check a primitive
///   applies to a path the host supplies, before any filesystem operation. `path` is repo-relative (the
///   bootstrap-substituted `{cli-config-dir}/settings.local.json`); no
///   caller needs an out-of-repo destination.
/// - [`PrimitiveError::Io`] on local filesystem failures.
/// - Under `canonical`, [`PrimitiveError::Toml`] when the config or session
///   file `Host::load` and `Paths::load` read does not parse, and
///   [`PrimitiveError::InvalidSpecsRoot`] for an invalid `[paths] specs-root`,
///   each naming the file, before the settings file is read.
/// - [`PrimitiveError::Json`] when an existing file fails JSON parse.
/// - [`PrimitiveError::JsonSchema`] when `permissions.allow` /
///   `permissions.deny`, or under `canonical` `permissions.additionalDirectories`,
///   exists but is not an array (e.g., null, object, string).
/// - [`PrimitiveError::ConflictingRevoke`] when an entry appears in
///   both `allow` and `revoke`, the canonical sets included under
///   `canonical`. Checked before the settings file is read or written, so a
///   contradictory call never leaves a partial write.
pub fn run(args: &MergePermissionsArgs, repo: &Path) -> Result<MergePermissionsResult> {
    super::validate_contained(repo, &args.path)?;
    let target_path = repo.join(&args.path);
    super::host_config::check_no_protected(repo, &target_path, &args.path)?;
    let (allow, deny, revoke, directories) = if args.canonical {
        (
            joined(&canonical_allow_ordered(), &args.allow),
            joined(&to_strings(CANONICAL_DENY), &args.deny),
            joined(&to_strings(RETIRED_ALLOW), &args.revoke),
            canonical_directories(repo, &args.path)?,
        )
    } else {
        (
            args.allow.clone(),
            args.deny.clone(),
            args.revoke.clone(),
            Vec::new(),
        )
    };
    validate_revoke_disjoint(&allow, &revoke)?;
    refuse_beyond_ceiling(&args.allow)?;

    let existing = match target_path.try_exists() {
        Ok(true) => Some(read_text(&target_path)?),
        Ok(false) => None,
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: target_path.clone(),
                source,
            });
        }
    };

    let MergeOutcome {
        post_value,
        action,
        allow_added,
        allow_deduped,
        allow_revoked,
        revoked,
        deny_added,
        deny_deduped,
        directories_added,
    } = compute_merge(
        existing.as_deref(),
        &Sets {
            allow: &allow,
            deny: &deny,
            revoke: &revoke,
            directories: &directories,
        },
        &target_path,
    )?;

    if action != "unchanged" {
        let serialized = serialize_pretty(&post_value);
        write_atomic(&target_path, &serialized)?;
    }

    Ok(MergePermissionsResult {
        path: target_path.to_string_lossy().into_owned(),
        action: action.into(),
        allow_added,
        allow_deduped,
        allow_revoked,
        revoked,
        deny_added,
        deny_deduped,
        directories_added,
    })
}

/// The non-MCP half of Claude's canonical `permissions.allow` set, step 2 of
/// `framework/bootstrap/configure/claude.md`. The MCP half is derived
/// ([`canonical_allow`]). `the_ceiling_is_configure_claudes_canonical_allow_set`
/// holds the two equal to that file, so narrowing the set there (022 task 18)
/// fails it until this list moves too.
const CANONICAL_ALLOW_STATIC: &[&str] = &[
    "Edit",
    "Write",
    "Edit(.ductus/session.toml)",
    "Edit(.ductus/config.toml)",
    "WebFetch",
    "WebSearch",
    "Bash(ls *)",
    "Bash(git add *)",
    "Bash(git commit *)",
    "Bash(git push *)",
    "Bash(git log *)",
    "Bash(git diff *)",
    "Bash(git status *)",
    "Bash(git show *)",
    "Bash(curl *)",
    "Bash(gh api *)",
    "Bash(mkdir -p *)",
    "Bash(chmod +x *)",
    "Bash(command -v *)",
    "Bash(make *)",
    "Bash(markdownlint *)",
    "Bash(markdownlint-cli2 *)",
    "Bash(npx markdownlint-cli2 *)",
    "Bash(git config core.hooksPath *)",
    "Bash(git config --get core.hooksPath)",
    "Bash(git config --unset core.hooksPath)",
    "Bash(./.githooks/pre-commit)",
    "Bash(scripts/install-hooks.sh)",
    "Bash(./scripts/install-hooks.sh)",
];

/// Claude's canonical `permissions.deny` set, step 3 of
/// `framework/bootstrap/configure/claude.md`, which
/// `the_canonical_deny_set_is_configure_claudes_step_3` holds equal to it.
pub(crate) const CANONICAL_DENY: &[&str] = &[
    "Bash(rm -rf *)",
    "Bash(rm -r *)",
    "Bash(rm -fr *)",
    "Bash(*rm -rf *)",
    "Bash(*rm -r *)",
    "Bash(*rm -fr *)",
    "Bash(git mv *)",
    "Bash(git push --force *)",
    "Bash(git push -f *)",
    "Bash(git reset --hard *)",
    "Bash(git rm *)",
    "Bash(git clean -fd *)",
    "Bash(git -C * mv *)",
    "Bash(git -C * push --force *)",
    "Bash(git -C * push -f *)",
    "Bash(git -C * reset --hard *)",
    "Bash(git -C * rm *)",
    "Bash(git -C * clean -fd *)",
    "Bash(chmod -R 777 *)",
    "Bash(> *)",
];

/// Claude's retired `permissions.allow` entries, step 4 of
/// `framework/bootstrap/configure/claude.md`, which
/// `the_retired_set_is_configure_claudes_step_4` holds equal to it. The
/// canonical call passes them as `revoke`.
pub(crate) const RETIRED_ALLOW: &[&str] = &[
    "Bash(git -C * add *)",
    "Bash(git -C * commit *)",
    "Bash(git -C * push *)",
    "Bash(git -C * log *)",
    "Bash(git -C * diff *)",
    "Bash(git -C * status *)",
    "Bash(git -C * show *)",
    "Write(.ductus/session.toml)",
    "Write(.ductus/config.toml)",
    "mcp__ductus__run-generator",
    "mcp__ductus__check-review-agreement",
    "mcp__ductus__read-supersession-pair",
    "mcp__ductus__write-supersession-annotation",
    "mcp__ductus__check-review-freshness",
];

/// [`canonical_allow`] in the order the canonical call appends missing
/// entries: [`CANONICAL_ALLOW_STATIC`], in step 2's order, then
/// `mcp__ductus__<name>` for each served primitive in registry order, which
/// is not the order of step 2's generated block.
fn canonical_allow_ordered() -> Vec<String> {
    CANONICAL_ALLOW_STATIC
        .iter()
        .map(|entry| (*entry).to_string())
        .chain(
            crate::schema::registry::served_primitives()
                .map(crate::schema::registry::claude_code_tool_name),
        )
        .collect()
}

fn to_strings(entries: &[&str]) -> Vec<String> {
    entries.iter().map(|entry| (*entry).to_string()).collect()
}

/// `first`, then each entry of `extra` not already in it.
fn joined(first: &[String], extra: &[String]) -> Vec<String> {
    let mut out = first.to_vec();
    for entry in extra {
        if !out.contains(entry) {
            out.push(entry.clone());
        }
    }
    out
}

/// The directories the canonical call ensures under
/// `permissions.additionalDirectories`, as absolute paths with no trailing
/// slash: the configured spec root, and `{config-dir}/commands/{project}`,
/// where `{config-dir}` is the settings file's own directory and `{project}`
/// the namespace [`Host::load`] resolves.
fn canonical_directories(repo: &Path, settings_path: &str) -> Result<Vec<String>> {
    let config_dir = Path::new(settings_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let commands = config_dir.join("commands").join(Host::load(repo)?.project);
    Ok([
        repo.join(Paths::load(repo)?.specs_root),
        repo.join(commands),
    ]
    .iter()
    .map(|dir| {
        dir.to_string_lossy()
            .trim_end_matches(['/', '\\'])
            .to_owned()
    })
    .collect())
}

/// Claude's canonical `permissions.allow` set: [`CANONICAL_ALLOW_STATIC`]
/// and `mcp__ductus__<name>` for every primitive the MCP server serves, the
/// block `scripts/gen-configure-mcp.sh` generates from
/// `framework/runtime-tools.txt`, which `runtime/tests/mcp.rs` holds equal to
/// the served set.
pub(crate) fn canonical_allow() -> std::collections::BTreeSet<String> {
    canonical_allow_ordered().into_iter().collect()
}

/// Refuse any `allow` entry outside [`canonical_allow`] (the ceiling).
fn refuse_beyond_ceiling(allow: &[String]) -> Result<()> {
    let canonical = canonical_allow();
    let beyond: Vec<&str> = allow
        .iter()
        .map(String::as_str)
        .filter(|entry| !canonical.contains(*entry))
        .collect();
    if beyond.is_empty() {
        return Ok(());
    }
    Err(PrimitiveError::InvalidArgument {
        primitive: "merge-permissions".into(),
        argument: "allow".into(),
        reason: format!(
            "{} {} not in Claude's canonical allow set \
             (framework/bootstrap/configure/claude.md step 2), and the primitive grants \
             nothing beyond it (spec 022 task 11): {}",
            beyond.len(),
            if beyond.len() == 1 {
                "entry is"
            } else {
                "entries are"
            },
            beyond
                .iter()
                .map(|entry| format!("`{entry}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    })
}

/// Reject a call whose `allow` and `revoke` sets intersect.
///
/// The two passes are irreconcilable for such an entry: revoke removes
/// it, the canonical-presence pass appends it back. Whichever runs last
/// wins, so the primitive would silently honour half the request and —
/// worse — could never emit `unchanged`, defeating the mtime-preserving
/// short-circuit callers rely on for idempotency. Reporting every
/// conflicting entry at once (rather than the first) means a caller
/// fixing a retired-entry list sees the whole overlap in one run.
fn validate_revoke_disjoint(allow: &[String], revoke: &[String]) -> Result<()> {
    // Deduped: a canonical set that repeats an entry would otherwise make
    // the message count one conflict twice and name it twice, reporting a
    // scale of problem the caller does not have.
    let mut conflicts: Vec<&str> = Vec::new();
    for entry in allow {
        if revoke.contains(entry) && !conflicts.contains(&entry.as_str()) {
            conflicts.push(entry.as_str());
        }
    }
    if conflicts.is_empty() {
        return Ok(());
    }
    Err(PrimitiveError::ConflictingRevoke {
        count: conflicts.len(),
        plural: if conflicts.len() == 1 { "y" } else { "ies" },
        entries: conflicts.join(", "),
    })
}

/// Pretty-print with 2-space indent and a trailing newline. The
/// `serde_json` `preserve_order` feature keeps `Map`'s insertion
/// order intact across serialization. `to_string_pretty` is
/// infallible on `serde_json::Value` (no non-string keys, no I/O),
/// so the `.expect` documents the invariant rather than handling a
/// reachable failure mode.
pub(crate) fn serialize_pretty(value: &Value) -> String {
    let mut out =
        serde_json::to_string_pretty(value).expect("serde_json::Value serializes infallibly");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

struct MergeOutcome {
    post_value: Value,
    action: &'static str,
    allow_added: u32,
    allow_deduped: u32,
    allow_revoked: u32,
    revoked: Vec<String>,
    deny_added: u32,
    deny_deduped: u32,
    directories_added: Vec<String>,
}

/// What one call merges: the entries to ensure under `allow` and `deny`, the
/// entries to retire from `allow`, and the paths to ensure under
/// `additionalDirectories`, empty unless the call is canonical.
struct Sets<'a> {
    allow: &'a [String],
    deny: &'a [String],
    revoke: &'a [String],
    directories: &'a [String],
}

fn compute_merge(existing: Option<&str>, sets: &Sets<'_>, path: &Path) -> Result<MergeOutcome> {
    match existing {
        // Nothing on disk to retire: a fresh file is written from the
        // canonical sets, which `validate_revoke_disjoint` has already
        // proven share no member with `revoke`.
        None => Ok(fresh_merge(sets)),
        Some(text) => existing_merge(text, sets, path),
    }
}

fn fresh_merge(sets: &Sets<'_>) -> MergeOutcome {
    let allow_added = u32::try_from(sets.allow.len()).unwrap_or(u32::MAX);
    let deny_added = u32::try_from(sets.deny.len()).unwrap_or(u32::MAX);
    let mut post_value = json!({
        "permissions": {
            "allow": sets.allow,
            "deny": sets.deny,
        }
    });
    if !sets.directories.is_empty() {
        post_value["permissions"]["additionalDirectories"] = json!(sets.directories);
    }
    MergeOutcome {
        post_value,
        action: "created",
        allow_added,
        allow_deduped: 0,
        allow_revoked: 0,
        revoked: Vec::new(),
        deny_added,
        deny_deduped: 0,
        directories_added: sets.directories.to_vec(),
    }
}

fn existing_merge(text: &str, sets: &Sets<'_>, path: &Path) -> Result<MergeOutcome> {
    let original: Value = serde_json::from_str(text).map_err(|source| PrimitiveError::Json {
        path: path.into(),
        source,
    })?;

    let mut post_value = original.clone();
    let permissions = ensure_permissions_object(&mut post_value, path)?;

    let ArrayOutcome {
        added: allow_added,
        deduped: allow_deduped,
        revoked: allow_revoked,
        revoked_entries: revoked,
    } = merge_array(permissions, "allow", sets.allow, sets.revoke, path)?;
    // Deny is never a retirement subject; see the module's Retirement note.
    let ArrayOutcome {
        added: deny_added,
        deduped: deny_deduped,
        ..
    } = merge_array(permissions, "deny", sets.deny, &[], path)?;
    let directories_added = ensure_directories(permissions, sets.directories, path)?;

    let action = if post_value == original {
        "unchanged"
    } else {
        "updated"
    };

    Ok(MergeOutcome {
        post_value,
        action,
        allow_added,
        allow_deduped,
        allow_revoked,
        revoked,
        deny_added,
        deny_deduped,
        directories_added,
    })
}

/// Append each of `directories` that `permissions.additionalDirectories` does
/// not hold, compared by exact string, creating the array when it is absent
/// and `directories` is not empty. Returns the paths appended, in order.
fn ensure_directories(
    permissions: &mut Map<String, Value>,
    directories: &[String],
    path: &Path,
) -> Result<Vec<String>> {
    if directories.is_empty() {
        return Ok(Vec::new());
    }
    let Some(array) = permissions
        .entry("additionalDirectories")
        .or_insert_with(|| json!([]))
        .as_array_mut()
    else {
        return Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: "`permissions.additionalDirectories` exists but is not an array".into(),
        });
    };
    let mut added = Vec::new();
    for directory in directories {
        if !array.iter().any(|held| held == directory.as_str()) {
            array.push(Value::String(directory.clone()));
            added.push(directory.clone());
        }
    }
    Ok(added)
}

/// Ensure the top-level value has a `permissions` object, returning a
/// mutable reference to its `Map`. If the top-level value is not an
/// object, return a schema error rather than silently overwriting.
fn ensure_permissions_object<'a>(
    value: &'a mut Value,
    path: &Path,
) -> Result<&'a mut Map<String, Value>> {
    let Some(root) = value.as_object_mut() else {
        return Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: "top-level value is not a JSON object".into(),
        });
    };
    let permissions = root.entry("permissions").or_insert_with(|| json!({}));
    match permissions.as_object_mut() {
        Some(map) => Ok(map),
        None => Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: "`permissions` field exists but is not a JSON object".into(),
        }),
    }
}

/// Per-array counts returned by [`merge_array`], and the distinct entries
/// the revoke pass removed, in `revoke`'s order.
struct ArrayOutcome {
    added: u32,
    deduped: u32,
    revoked: u32,
    revoked_entries: Vec<String>,
}

/// Apply the revoke + dedup + canonical-presence passes to one array
/// field on `permissions`. `field` is `"allow"` or `"deny"`; `revoke`
/// is always empty for `"deny"` (retirement is allow-side only).
///
/// Pass order is revoke-first, and it is load-bearing for the counts.
/// Deduping first would remove the second copy of a doubled retired
/// entry as a *duplicate* and the first as a *retirement*, splitting one
/// cause across two counters and crediting the dedup pass with work it
/// did not conceptually do. Revoking first attributes every copy to the
/// reason it actually went, and leaves `deduped` to mean what it says:
/// duplicates among the entries that survive.
fn merge_array(
    permissions: &mut Map<String, Value>,
    field: &str,
    canonical: &[String],
    revoke: &[String],
    path: &Path,
) -> Result<ArrayOutcome> {
    let Some(array_value) = permissions.get_mut(field) else {
        permissions.insert(
            field.into(),
            Value::Array(canonical.iter().map(|s| Value::String(s.clone())).collect()),
        );
        let added = u32::try_from(canonical.len()).unwrap_or(u32::MAX);
        // No array on disk means nothing to retire, so `revoke` has no
        // subject here even when non-empty.
        return Ok(ArrayOutcome {
            added,
            deduped: 0,
            revoked: 0,
            revoked_entries: Vec::new(),
        });
    };

    let Some(arr) = array_value.as_array_mut() else {
        return Err(PrimitiveError::JsonSchema {
            path: path.into(),
            reason: format!("`permissions.{field}` exists but is not an array"),
        });
    };

    // Revoke pass: drop every copy of a formerly-canonical entry, by
    // exact string match. Runs before dedup so a doubled retired entry
    // is attributed wholly to retirement (see the note above).
    let mut revoked = 0u32;
    let mut removed: Vec<String> = Vec::new();
    if !revoke.is_empty() {
        let mut idx = 0;
        while idx < arr.len() {
            let retired = arr[idx]
                .as_str()
                .filter(|entry| revoke.iter().any(|r| r == entry))
                .map(str::to_owned);
            if let Some(entry) = retired {
                arr.remove(idx);
                revoked = revoked.saturating_add(1);
                removed.push(entry);
                continue;
            }
            idx += 1;
        }
    }
    let mut revoked_entries: Vec<String> = Vec::new();
    for entry in revoke {
        if removed.contains(entry) && !revoked_entries.contains(entry) {
            revoked_entries.push(entry.clone());
        }
    }

    // Dedup pass: first occurrence wins; later duplicates removed in place.
    let mut seen: Vec<String> = Vec::with_capacity(arr.len());
    let mut deduped = 0u32;
    let mut idx = 0;
    while idx < arr.len() {
        if let Some(s) = arr[idx].as_str() {
            let s_owned = s.to_string();
            if seen.contains(&s_owned) {
                arr.remove(idx);
                deduped = deduped.saturating_add(1);
                continue;
            }
            seen.push(s_owned);
        }
        // Non-string entries are preserved verbatim and not considered
        // for dedup. The canonical set is string-valued; non-string
        // entries don't collide.
        idx += 1;
    }

    // Canonical-presence pass: append any canonical entry not already
    // present (by string-equality), preserving canonical-set order.
    let mut added = 0u32;
    for entry in canonical {
        if !seen.contains(entry) {
            arr.push(Value::String(entry.clone()));
            seen.push(entry.clone());
            added = added.saturating_add(1);
        }
    }

    Ok(ArrayOutcome {
        added,
        deduped,
        revoked,
        revoked_entries,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;

    fn args(path: &str, allow: &[&str], deny: &[&str]) -> MergePermissionsArgs {
        args_revoking(path, allow, deny, &[])
    }

    fn args_revoking(
        path: &str,
        allow: &[&str],
        deny: &[&str],
        revoke: &[&str],
    ) -> MergePermissionsArgs {
        MergePermissionsArgs {
            path: path.to_string(),
            allow: allow.iter().map(|s| (*s).to_string()).collect(),
            deny: deny.iter().map(|s| (*s).to_string()).collect(),
            revoke: revoke.iter().map(|s| (*s).to_string()).collect(),
            canonical: false,
        }
    }

    /// Read a settings file back as JSON.
    fn read_json(path: &std::path::Path) -> Value {
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    /// Read back `permissions.allow` as owned strings.
    fn allow_of(path: &std::path::Path) -> Vec<String> {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        v["permissions"]["allow"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e.as_str().unwrap().to_string())
            .collect()
    }

    /// The nine entries the `retired-permission-entries` migration
    /// retires: seven wildcard-before-subcommand git allows (spec 023
    /// task 21) and two inert `Write(path)` allows (spec 023's
    /// `configure-inert-write-path-entries`).
    const RETIRED: &[&str] = &[
        "Bash(git -C * add *)",
        "Bash(git -C * commit *)",
        "Bash(git -C * push *)",
        "Bash(git -C * log *)",
        "Bash(git -C * diff *)",
        "Bash(git -C * status *)",
        "Bash(git -C * show *)",
        "Write(.ductus/session.toml)",
        "Write(.ductus/config.toml)",
    ];

    #[test]
    fn creates_file_when_absent_with_canonical_set() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join(".claude/settings.local.json");
        let result = run(
            &args(
                ".claude/settings.local.json",
                &["Edit", "Bash(ls *)"],
                &["Bash(rm -rf *)"],
            ),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "created");
        assert_eq!(result.allow_added, 2);
        assert_eq!(result.deny_added, 1);
        assert_eq!(result.allow_deduped, 0);
        assert_eq!(result.deny_deduped, 0);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["allow"][1], "Bash(ls *)");
        assert_eq!(parsed["permissions"]["deny"][0], "Bash(rm -rf *)");
    }

    #[test]
    fn writes_to_host_supplied_path_for_non_claude_adopter() {
        // A host-supplied path under another config directory (Pi's settings
        // file, which carries no permissions): the primitive writes wherever
        // the caller says without baking in a Claude-shaped default.
        let tmp = tempfile::tempdir().unwrap();
        let result = run(&args(".pi/settings.json", &["Edit"], &[]), tmp.path()).unwrap();
        assert!(result.path.ends_with(".pi/settings.json"));
        assert_eq!(result.action, "created");
        assert!(tmp.path().join(".pi/settings.json").is_file());
        assert!(
            !tmp.path().join(".claude").exists(),
            "A non-Claude write must not create a Claude-shaped sibling"
        );
    }

    #[test]
    fn dedups_existing_duplicates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("settings.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["Edit","Bash(ls *)","Edit","Write","Edit"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(
            &args("settings.json", &["Edit", "Bash(ls *)"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 0);
        assert_eq!(
            result.allow_deduped, 2,
            "two extra 'Edit' entries should be removed"
        );

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow.len(), 3, "Edit + Bash(ls *) + Write after dedup");
        assert_eq!(allow[0], "Edit", "first occurrence wins");
        assert_eq!(allow[1], "Bash(ls *)");
        assert_eq!(allow[2], "Write");
    }

    #[test]
    fn dedup_includes_non_canonical_entries() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["UserAdded","UserAdded","Other","UserAdded"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &[]), tmp.path()).unwrap();
        assert_eq!(result.allow_deduped, 2);
        assert_eq!(result.allow_added, 0);
        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow.len(), 2);
        assert_eq!(allow[0], "UserAdded");
        assert_eq!(allow[1], "Other");
    }

    #[test]
    fn appends_missing_canonical_at_end_preserving_order() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["UserA","UserB"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &["WebFetch", "WebSearch"], &[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 2);
        assert_eq!(result.allow_deduped, 0);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert_eq!(
            allow
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["UserA", "UserB", "WebFetch", "WebSearch"]
        );
    }

    #[test]
    fn canonical_present_is_not_re_appended() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["WebFetch","UserA"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &["WebFetch", "WebSearch"], &[]), tmp.path()).unwrap();
        assert_eq!(result.allow_added, 1, "only WebSearch was missing");
        assert_eq!(result.allow_deduped, 0);
    }

    #[test]
    fn unchanged_when_canonical_present_and_no_duplicates() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        let original = r#"{
  "permissions": {
    "allow": [
      "Edit",
      "Bash(ls *)"
    ],
    "deny": [
      "Bash(rm -rf *)"
    ]
  }
}
"#;
        fs::write(&path, original).unwrap();
        let mtime_before = fs::metadata(&path).unwrap().modified().unwrap();

        let result = run(
            &args("s.json", &["Edit", "Bash(ls *)"], &["Bash(rm -rf *)"]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "unchanged");
        assert_eq!(result.allow_added, 0);
        assert_eq!(result.allow_deduped, 0);

        // mtime preserved (no write happened).
        assert_eq!(
            fs::metadata(&path).unwrap().modified().unwrap(),
            mtime_before
        );

        // File content untouched byte-for-byte.
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
    }

    #[test]
    fn preserves_untouched_top_level_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{
  "permissions": {
    "allow": ["UserA"],
    "deny": [],
    "additionalDirectories": ["/foo", "/bar"]
  },
  "defaultMode": "default",
  "customField": {"nested": "value"}
}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &["WebFetch"], &[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["defaultMode"], "default");
        assert_eq!(parsed["customField"]["nested"], "value");
        assert_eq!(parsed["permissions"]["additionalDirectories"][0], "/foo");
        assert_eq!(parsed["permissions"]["additionalDirectories"][1], "/bar");
    }

    #[test]
    fn missing_permissions_object_is_added() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"defaultMode": "default"}"#).unwrap();
        let result = run(&args("s.json", &["Edit"], &["Bash(rm -rf *)"]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["defaultMode"], "default");
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["deny"][0], "Bash(rm -rf *)");
    }

    #[test]
    fn missing_allow_array_is_seeded() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"permissions":{"deny":["Existing"]}}"#).unwrap();
        let result = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_added, 1);

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["permissions"]["allow"][0], "Edit");
        assert_eq!(parsed["permissions"]["deny"][0], "Existing");
    }

    #[test]
    fn absolute_path_is_rejected_before_any_write() {
        let tmp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("settings.json");
        let err = run(&args(&target.to_string_lossy(), &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(
            matches!(err, PrimitiveError::InvalidPath { .. }),
            "expected InvalidPath, got {err:?}"
        );
        assert!(!target.exists(), "nothing may be written outside the repo");
    }

    #[test]
    fn traversal_path_is_rejected_before_any_write() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("../settings.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }));
    }

    #[test]
    fn malformed_json_returns_parse_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, r#"{"permissions": {"allow": [oops}"#).unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::Json { .. }));
        // File should be unchanged.
        let body = fs::read_to_string(&path).unwrap();
        assert!(body.contains("oops"));
    }

    #[test]
    fn non_array_allow_returns_schema_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions": {"allow": "not-an-array", "deny": []}}"#,
        )
        .unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        match err {
            PrimitiveError::JsonSchema { reason, .. } => {
                assert!(reason.contains("permissions.allow"));
                assert!(reason.contains("not an array"));
            }
            other => panic!("expected JsonSchema, got {other:?}"),
        }
    }

    #[test]
    fn non_object_top_level_returns_schema_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(&path, "[]").unwrap();
        let err = run(&args("s.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::JsonSchema { .. }));
    }

    #[test]
    fn deny_array_is_independently_handled() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":[],"deny":["Bash(rm -rf *)","Bash(rm -rf *)","Other"]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &["Bash(rm -rf *)"]), tmp.path()).unwrap();
        assert_eq!(result.deny_deduped, 1);
        assert_eq!(result.deny_added, 0);
        assert_eq!(result.allow_added, 0);
        assert_eq!(result.allow_deduped, 0);
    }

    #[test]
    fn non_string_entries_preserved_and_ignored_for_dedup() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("s.json");
        fs::write(
            &path,
            r#"{"permissions":{"allow":["Edit",42,"Edit",null,"Edit"],"deny":[]}}"#,
        )
        .unwrap();
        let result = run(&args("s.json", &[], &[]), tmp.path()).unwrap();
        assert_eq!(result.allow_deduped, 2, "duplicate 'Edit' entries removed");

        let body = fs::read_to_string(&path).unwrap();
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        // After dedup: "Edit", 42, null
        assert_eq!(allow.len(), 3);
        assert_eq!(allow[0], "Edit");
        assert_eq!(allow[1], 42);
        assert_eq!(allow[2], Value::Null);
    }
    // -- retirement (`revoke`) ------------------------------------------

    #[test]
    fn revoke_removes_every_retired_entry_reported_by_the_host_linter() {
        // The exact nine entries Claude Code warns about at session start
        // in a tree configured before spec 023 task 21 and before
        // `configure-inert-write-path-entries`.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(".claude/settings.local.json");
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        let mut pre: Vec<&str> = vec!["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"];
        pre.extend_from_slice(RETIRED);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": { "allow": pre, "deny": ["Bash(rm -rf *)"] }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking(
                ".claude/settings.local.json",
                &["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"],
                &["Bash(rm -rf *)"],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_revoked, 9, "all nine retired entries removed");
        assert_eq!(result.allow_added, 0, "canonical set was already present");
        assert_eq!(result.allow_deduped, 0);
        assert_eq!(
            allow_of(&target),
            vec!["Edit", "Edit(.ductus/session.toml)", "Bash(ls *)"],
            "survivors keep their original order"
        );
    }

    #[test]
    fn revoke_never_reaches_the_deny_array() {
        // Retirement is allow-side only *by construction*: the deny array
        // is merged with an empty revoke list, so even an entry named in
        // `revoke` survives there. An over-broad deny refuses more rather
        // than approving more, so narrowing it would open a hole.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": ["Bash(git -C * rm *)"],
                    "deny": ["Bash(git -C * rm *)"],
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking(
                "settings.json",
                &[],
                &["Bash(git -C * rm *)"],
                &["Bash(git -C * rm *)"],
            ),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1, "removed from allow");
        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(
            v["permissions"]["deny"][0], "Bash(git -C * rm *)",
            "the identical string is a correct deny entry and must survive"
        );
        assert_eq!(
            v["permissions"]["allow"].as_array().unwrap().len(),
            0,
            "and must be gone from allow"
        );
    }

    #[test]
    fn revoke_matches_exactly_and_spares_adopter_authored_entries() {
        // Shape-matching is deliberately not offered. An adopter who wrote
        // their own near-miss of a retired pattern keeps it; only what the
        // framework itself shipped is removed.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let adopter = [
            "Bash(git -C * status)",         // no trailing wildcard
            "Bash(git -C ~/other status *)", // pinned path, not a wildcard
            "bash(git -C * status *)",       // different tool casing
            "Write(.ductus/session.toml )",  // stray space
        ];
        let mut pre = vec!["Bash(git -C * status *)"];
        pre.extend_from_slice(&adopter);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({ "permissions": { "allow": pre } })).unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &[], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1, "only the exact match went");
        assert_eq!(
            allow_of(&target),
            adopter.to_vec(),
            "every adopter-authored near-miss survives untouched"
        );
    }

    #[test]
    fn revoke_removes_every_copy_of_a_doubled_retired_entry() {
        // Revoke runs before dedup, so both copies are attributed to the
        // retirement rather than one being credited to the dedup pass.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": [
                        "Write(.ductus/config.toml)",
                        "Edit",
                        "Write(.ductus/config.toml)",
                    ]
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 2, "both copies count as retired");
        assert_eq!(
            result.allow_deduped, 0,
            "and neither is miscounted as a duplicate"
        );
        assert_eq!(allow_of(&target), vec!["Edit"]);
    }

    #[test]
    fn revoke_is_a_silent_no_op_on_an_already_clean_file() {
        // The second `/ductus` run, and every fresh adopter. `unchanged`
        // skips the write, preserving mtime.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let clean = json!({ "permissions": { "allow": ["Edit"], "deny": [] } });
        fs::write(&target, serialize_pretty(&clean)).unwrap();
        let before = fs::metadata(&target).unwrap().modified().unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "unchanged");
        assert_eq!(result.allow_revoked, 0);
        assert_eq!(
            fs::metadata(&target).unwrap().modified().unwrap(),
            before,
            "an unchanged merge must not rewrite the file"
        );
    }

    #[test]
    fn revoke_reaches_a_fixed_point_on_the_second_run() {
        // Idempotency is what makes the migration safe to re-run: the
        // adopter's next `/ductus` must report `unchanged`, not churn.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        let mut pre = vec!["Bash(ls *)"];
        pre.extend_from_slice(RETIRED);
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({ "permissions": { "allow": pre } })).unwrap(),
        )
        .unwrap();
        let call = args_revoking("settings.json", &["Bash(ls *)", "Edit"], &[], RETIRED);

        let first = run(&call, tmp.path()).unwrap();
        assert_eq!(first.action, "updated");
        assert_eq!(first.allow_revoked, 9);
        assert_eq!(first.allow_added, 1, "`Edit` appended");

        let second = run(&call, tmp.path()).unwrap();
        assert_eq!(second.action, "unchanged");
        assert_eq!(second.allow_revoked, 0);
        assert_eq!(second.allow_added, 0);
    }

    #[test]
    fn revoke_on_an_absent_file_writes_the_canonical_set_only() {
        let tmp = tempfile::tempdir().unwrap();
        let result = run(
            &args_revoking("settings.json", &["Edit"], &["Bash(rm -rf *)"], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.action, "created");
        assert_eq!(
            result.allow_revoked, 0,
            "a file that did not exist has nothing to retire"
        );
        assert_eq!(allow_of(&tmp.path().join("settings.json")), vec!["Edit"]);
    }

    #[test]
    fn revoke_preserves_unrelated_keys_and_sibling_permission_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "$schema": "https://example.test/schema.json",
                "model": "opus",
                "permissions": {
                    "allow": ["Write(.ductus/config.toml)", "Edit"],
                    "ask": ["Bash(curl *)"],
                    "additionalDirectories": ["/abs/specs"],
                }
            }))
            .unwrap(),
        )
        .unwrap();

        run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        assert_eq!(v["$schema"], "https://example.test/schema.json");
        assert_eq!(v["model"], "opus");
        assert_eq!(v["permissions"]["ask"][0], "Bash(curl *)");
        assert_eq!(v["permissions"]["additionalDirectories"][0], "/abs/specs");
        assert_eq!(allow_of(&target), vec!["Edit"]);
    }

    #[test]
    fn an_entry_in_both_allow_and_revoke_is_rejected_before_any_write() {
        // The two passes would fight, so the merge could never reach a
        // fixed point. Caught before the file is read, so a contradictory
        // call cannot leave a partial write behind.
        let tmp = tempfile::tempdir().unwrap();
        let err = run(
            &args_revoking(
                "settings.json",
                &["Edit", "Write(.ductus/config.toml)"],
                &[],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(
            message.contains("Write(.ductus/config.toml)"),
            "names the offending entry: {message}"
        );
        assert!(
            message.contains("share 1 entry:"),
            "singular form: {message}"
        );
        assert!(
            !tmp.path().join("settings.json").exists(),
            "a rejected call writes nothing"
        );
    }

    #[test]
    fn conflicting_revoke_reports_every_overlap_at_once() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(
            &args_revoking(
                "settings.json",
                &["Write(.ductus/config.toml)", "Write(.ductus/session.toml)"],
                &[],
                RETIRED,
            ),
            tmp.path(),
        )
        .unwrap_err();

        let message = err.to_string();
        assert!(message.contains("share 2 entries:"), "plural: {message}");
        assert!(message.contains("Write(.ductus/config.toml)"), "{message}");
        assert!(message.contains("Write(.ductus/session.toml)"), "{message}");
    }

    #[test]
    fn revoke_survives_non_string_entries_in_the_allow_array() {
        // The array is adopter-owned; a hand-edited file can hold a
        // non-string. Retirement must skip it rather than panic.
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(
            &target,
            serde_json::to_string_pretty(&json!({
                "permissions": {
                    "allow": [42, "Write(.ductus/config.toml)", Value::Null, "Edit"]
                }
            }))
            .unwrap(),
        )
        .unwrap();

        let result = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap();

        assert_eq!(result.allow_revoked, 1);
        let v: Value = serde_json::from_str(&fs::read_to_string(&target).unwrap()).unwrap();
        let allow = v["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow[0], 42, "non-strings preserved verbatim");
        assert_eq!(allow[1], Value::Null);
        assert_eq!(allow[2], "Edit");
    }

    #[test]
    fn revoke_on_a_malformed_file_refuses_without_writing() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("settings.json");
        fs::write(&target, "{ not json").unwrap();

        let err = run(
            &args_revoking("settings.json", &["Edit"], &[], RETIRED),
            tmp.path(),
        )
        .unwrap_err();

        assert!(matches!(err, PrimitiveError::Json { .. }));
        assert_eq!(
            fs::read_to_string(&target).unwrap(),
            "{ not json",
            "a hand-edited file is reported, never silently rewritten"
        );
    }

    // -- ceiling (spec 022 task 11) --------------------------------------------

    /// An allow entry outside Claude's canonical set is refused before
    /// anything is written, named in the error.
    #[test]
    fn an_allow_entry_beyond_the_canonical_set_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run(&args("s.json", &["Edit", "Bash(*)"], &[]), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, reason, .. }
                if argument == "allow" && reason.contains("`Bash(*)`") && !reason.contains("`Edit`")),
            "{err}"
        );
        assert!(!tmp.path().join("s.json").exists());
        // A deny entry needs no ceiling: it only takes a grant away.
        run(&args("s.json", &["Edit"], &["Bash(*)"]), tmp.path()).unwrap();
    }

    /// The ceiling is the canonical allow set `/configure` installs on
    /// Claude: step 2 of `framework/bootstrap/configure/claude.md`, its
    /// generated `mcp__ductus__*` block included. Narrowing that set (022
    /// task 18) fails this test until `CANONICAL_ALLOW_STATIC` moves with it.
    #[test]
    fn the_ceiling_is_configure_claudes_canonical_allow_set() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../framework/bootstrap/configure/claude.md"
        );
        let text = fs::read_to_string(path).unwrap();
        let start = text
            .find("\n2. Canonical `permissions.allow` entries")
            .expect("step 2 heading");
        let end = start
            + text[start..]
                .find("\n3. Canonical `permissions.deny` entries")
                .expect("step 3 heading");
        let documented: std::collections::BTreeSet<String> = text[start..end]
            .lines()
            .filter_map(|line| {
                let item = line.trim().strip_prefix("- `")?.strip_suffix('`')?;
                (!item.contains('`')).then(|| item.to_string())
            })
            .collect();
        assert!(
            documented.len() > 60 && documented.iter().any(|e| e.starts_with("mcp__ductus__")),
            "the parse found the bullets: {documented:?}"
        );
        assert_eq!(documented, canonical_allow());
    }

    /// The bullets of `framework/bootstrap/configure/claude.md` between the
    /// line starting `start` and the line starting `end`, each a single code
    /// span, as the ceiling test parses step 2.
    fn configure_bullets(start: &str, end: &str) -> Vec<String> {
        let text = fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../framework/bootstrap/configure/claude.md"
        ))
        .unwrap();
        let from = text.find(start).expect("start heading");
        let to = from + text[from..].find(end).expect("end heading");
        text[from..to]
            .lines()
            .filter_map(|line| {
                let item = line.trim().strip_prefix("- `")?.strip_suffix('`')?;
                (!item.contains('`')).then(|| item.to_string())
            })
            .collect()
    }

    /// The canonical call's deny set is configure's step 3, in its order.
    #[test]
    fn the_canonical_deny_set_is_configure_claudes_step_3() {
        let documented = configure_bullets(
            "\n3. Canonical `permissions.deny` entries",
            "\n4. Retired `permissions.allow` entries",
        );
        assert!(
            documented.len() > 10,
            "the parse found the bullets: {documented:?}"
        );
        assert_eq!(documented, to_strings(CANONICAL_DENY));
    }

    /// The canonical call's retired set is configure's step 4, in its order.
    #[test]
    fn the_retired_set_is_configure_claudes_step_4() {
        let documented = configure_bullets(
            "\n4. Retired `permissions.allow` entries",
            "\n5. Ensure `permissions.additionalDirectories`",
        );
        assert!(
            documented.len() > 10,
            "the parse found the bullets: {documented:?}"
        );
        assert_eq!(documented, to_strings(RETIRED_ALLOW));
    }

    /// A project named `acme`, its spec root `root` when one is given.
    fn acme(root: Option<&str>) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        let paths = root.map_or_else(String::new, |root| {
            format!("\n[paths]\nspecs-root = \"{root}\"\n")
        });
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            format!("[host]\nproject = \"acme\"\n{paths}"),
        )
        .unwrap();
        tmp
    }

    fn canonical_args(allow: &[&str]) -> MergePermissionsArgs {
        MergePermissionsArgs {
            canonical: true,
            ..args(".claude/settings.local.json", allow, &[])
        }
    }

    /// `rel` under `repo`, joined a component at a time as
    /// `canonical_directories` joins it, so the expected path carries the
    /// platform's own separator.
    fn dir(repo: &Path, rel: &str) -> String {
        rel.split('/')
            .fold(repo.to_path_buf(), |path, part| path.join(part))
            .to_string_lossy()
            .into_owned()
    }

    /// With no file, the canonical call writes the three sets and both
    /// directories, the model passing no entry.
    #[test]
    fn a_canonical_call_writes_the_canonical_sets_and_both_directories() {
        let tmp = acme(None);
        let result = run(&canonical_args(&[]), tmp.path()).unwrap();
        assert_eq!(result.action, "created");
        let value = read_json(&tmp.path().join(".claude/settings.local.json"));
        let strings = |field: &str| -> Vec<String> {
            value["permissions"][field]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect()
        };
        assert_eq!(strings("allow"), canonical_allow_ordered());
        assert_eq!(strings("deny"), to_strings(CANONICAL_DENY));
        let directories = vec![
            dir(tmp.path(), "specs"),
            dir(tmp.path(), ".claude/commands/acme"),
        ];
        assert_eq!(strings("additionalDirectories"), directories);
        assert_eq!(result.directories_added, directories);
        assert_eq!(result.allow_added as usize, canonical_allow_ordered().len());
        assert_eq!(result.deny_added as usize, CANONICAL_DENY.len());
    }

    /// On an existing file the canonical call retires every copy of a retired
    /// entry and names it once, keeps the adopter's own entries and
    /// directories where they are, and appends only the directory missing.
    #[test]
    fn a_canonical_call_retires_names_and_keeps_the_adopters_entries() {
        let tmp = acme(None);
        let specs = dir(tmp.path(), "specs");
        let before = json!({
            "permissions": {
                "allow": ["Bash(my-tool *)", "Write(.ductus/session.toml)", "Edit", "Write(.ductus/session.toml)"],
                "deny": ["Bash(my-deny *)"],
                "additionalDirectories": ["/elsewhere", specs],
            },
            "model": "opus",
        });
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        fs::write(
            tmp.path().join(".claude/settings.local.json"),
            before.to_string(),
        )
        .unwrap();
        let result = run(&canonical_args(&[]), tmp.path()).unwrap();
        assert_eq!(result.action, "updated");
        assert_eq!(result.allow_revoked, 2);
        assert_eq!(result.revoked, vec!["Write(.ductus/session.toml)"]);
        let value = read_json(&tmp.path().join(".claude/settings.local.json"));
        assert_eq!(value["model"], "opus");
        let allow = value["permissions"]["allow"].as_array().unwrap();
        assert_eq!(allow[0], "Bash(my-tool *)");
        assert_eq!(allow[1], "Edit");
        assert!(
            !allow
                .iter()
                .any(|entry| entry == "Write(.ductus/session.toml)")
        );
        assert_eq!(value["permissions"]["deny"][0], "Bash(my-deny *)");
        let commands = dir(tmp.path(), ".claude/commands/acme");
        assert_eq!(
            value["permissions"]["additionalDirectories"],
            json!(["/elsewhere", specs, commands])
        );
        assert_eq!(result.directories_added, vec![commands]);

        let again = run(&canonical_args(&[]), tmp.path()).unwrap();
        assert_eq!(again.action, "unchanged");
        assert_eq!(again.revoked, Vec::<String>::new());
        assert_eq!(again.directories_added, Vec::<String>::new());
    }

    /// The spec-root directory follows `[paths] specs-root`.
    #[test]
    fn the_canonical_spec_root_directory_follows_the_configured_root() {
        let tmp = acme(Some("design"));
        let result = run(&canonical_args(&[]), tmp.path()).unwrap();
        assert_eq!(result.directories_added[0], dir(tmp.path(), "design"));
    }

    /// Entries a canonical caller passes are still bounded: one beyond the
    /// ceiling is refused, a retired one is named as the overlap, and an
    /// `additionalDirectories` that is not an array is refused unwritten.
    #[test]
    fn a_canonical_call_still_refuses_what_an_explicit_call_refuses() {
        let tmp = acme(None);
        let beyond = run(&canonical_args(&["Bash(*)"]), tmp.path()).unwrap_err();
        assert!(
            matches!(&beyond, PrimitiveError::InvalidArgument { argument, .. } if argument == "allow"),
            "{beyond}"
        );
        let retired =
            run(&canonical_args(&["Write(.ductus/config.toml)"]), tmp.path()).unwrap_err();
        assert!(
            matches!(&retired, PrimitiveError::ConflictingRevoke { entries, .. } if entries == "Write(.ductus/config.toml)"),
            "{retired}"
        );
        assert!(!tmp.path().join(".claude/settings.local.json").exists());
        fs::create_dir_all(tmp.path().join(".claude")).unwrap();
        let text = r#"{"permissions": {"additionalDirectories": "specs"}}"#;
        fs::write(tmp.path().join(".claude/settings.local.json"), text).unwrap();
        let shape = run(&canonical_args(&[]), tmp.path()).unwrap_err();
        assert!(
            matches!(shape, PrimitiveError::JsonSchema { .. }),
            "{shape}"
        );
        assert_eq!(
            fs::read_to_string(tmp.path().join(".claude/settings.local.json")).unwrap(),
            text
        );
    }

    /// An explicit call writes no directory, as before the canonical form.
    #[test]
    fn an_explicit_call_ensures_no_directory() {
        let tmp = acme(None);
        let result = run(
            &args(".claude/settings.local.json", &["Edit"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.directories_added, Vec::<String>::new());
        let value = read_json(&tmp.path().join(".claude/settings.local.json"));
        assert!(value["permissions"].get("additionalDirectories").is_none());
    }

    /// A path in the host's protected set is refused, before any write (spec
    /// 022 task 20). A config-directory settings file stays allowed — that is
    /// `merge-permissions`' own target, covered by the tests above that write
    /// `.pi/settings.json`; this is the protected-set complement.
    #[test]
    fn a_protected_path_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        // `.git`/`.cargo`/`.npmrc` are task-20 protected paths; `.mcp.json` is
        // task 11's MCP file, which `merge-permissions` reaches only because it
        // was not routed through the check before (task 20 closes it).
        // Another tool's configuration paths, which grant an agent tools, are
        // refused outright: no agent's settings file lives there.
        for path in [
            ".git/config",
            ".cargo/config.toml",
            ".npmrc",
            ".mcp.json",
            ".augment/settings.json",
            ".agents/settings.json",
            ".opencode/settings.json",
            "opencode.json",
            "opencode.jsonc",
        ] {
            let err = run(&args(path, &["Edit"], &[]), tmp.path()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidPath { reason, .. }
                    if reason.contains("task 20") || reason.contains("task 11")),
                "{path}: {err:?}"
            );
            assert!(!tmp.path().join(path).exists(), "{path}: nothing written");
        }
    }

    /// A config-directory settings file stays allowed — `merge-permissions`'
    /// own target — since the protected-set check does not judge the config
    /// directories (spec 022 task 20, not task 11's row logic).
    #[test]
    fn a_config_dir_settings_file_stays_allowed() {
        let tmp = tempfile::tempdir().unwrap();
        let result = run(
            &args(".claude/settings.local.json", &["Edit"], &[]),
            tmp.path(),
        )
        .unwrap();
        assert_eq!(result.action, "created");
        assert!(tmp.path().join(".claude/settings.local.json").exists());
    }

    /// The working directory may itself be a protected subdirectory of a git
    /// work tree, so the protected component is above `repo`; `check_no_protected`
    /// delegates to `check_destination`, which judges from the outer work-tree
    /// root too, and refuses (spec 022 task 20, re-read M-1).
    #[test]
    fn a_protected_path_above_the_working_directory_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        git2::Repository::init(tmp.path()).unwrap();
        let cwd = tmp.path().join(".cargo");
        std::fs::create_dir_all(&cwd).unwrap();
        let err = run(&args("config.toml", &["Edit"], &[]), &cwd).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. } if reason.contains("task 20")),
            "{err:?}"
        );
        assert!(!cwd.join("config.toml").exists(), "nothing written");
    }

    /// A write to the target a root MCP/settings-file link resolves to is
    /// refused: `check_destination`'s root-file link-followback catches it
    /// (spec 022 task 20, re-read S-1).
    #[cfg(unix)]
    #[test]
    fn a_write_through_a_root_mcp_link_is_refused() {
        let tmp = tempfile::tempdir().unwrap();
        git2::Repository::init(tmp.path()).unwrap();
        std::fs::create_dir_all(tmp.path().join("shared")).unwrap();
        std::os::unix::fs::symlink("shared/m.json", tmp.path().join(".mcp.json")).unwrap();
        let err = run(&args("shared/m.json", &["Edit"], &[]), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidPath { reason, .. } if reason.contains(".mcp.json")),
            "{err:?}"
        );
    }
}
