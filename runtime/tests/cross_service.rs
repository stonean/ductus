//! Cross-service reference resolution: golden records + a recorded capture.
//!
//! Spec 030's deterministic resolution work — resolve each `references:`
//! entry against the project config's `[services]` registry — resolved
//! through `schema::paths`'s three-tier `CONFIG_CHAIN`; the fixture below
//! deliberately uses the legacy root `.govern.toml` tier — read the linked
//! spec's `status` from its local checkout, classify the outcome — runs
//! through the `resolve-references` runtime primitive, with or without the
//! MCP server. The classification is fully deterministic (no prose read for
//! intent), so its records can be pinned byte for byte.
//!
//! This test pins those records two ways against the `cross-service-basic`
//! fixture, whose consumer spec exercises every outcome:
//!
//!   1. `resolve_references::run` serialized one
//!      `ResolutionRecord` per line equals `golden/cross-service-basic.jsonl`.
//!   2. Byte-identity: that golden equals the capture recorded under
//!      `parity/cross-service/expected.txt` when spec 030 shipped (`3f635df3`).
//!      No host produces those records by hand any more (spec 079), so this
//!      now guards only the bless: a re-blessed golden without an updated
//!      capture, or vice versa, fails the check.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::PathBuf;

use ductus::primitives::resolve_references;
use ductus::schema::primitives::ResolveReferencesArgs;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn cross_service_basic_records_match_golden_and_recorded_capture() {
    let repo = manifest_dir().join("tests/fixtures/cross-service-basic");
    let args = ResolveReferencesArgs {
        feature: "001-consumer".into(),
    };
    let result = resolve_references::run(&args, &repo).expect("resolve-references runs clean");

    // One ResolutionRecord per line, in the consumer's `references:` order.
    let mut actual = String::new();
    for record in &result.references {
        actual.push_str(&serde_json::to_string(record).expect("serialize record"));
        actual.push('\n');
    }

    let golden = fs::read_to_string(manifest_dir().join("tests/golden/cross-service-basic.jsonl"))
        .expect("read golden");
    assert_eq!(
        actual, golden,
        "runtime resolution records must match golden/cross-service-basic.jsonl — \
         re-bless by overwriting the golden with the captured records"
    );

    // The recorded capture must be byte-identical to the golden, so a re-bless
    // has to move both; the module doc says why that is all it proves now.
    let capture =
        fs::read_to_string(manifest_dir().join("tests/parity/cross-service/expected.txt"))
            .expect("read recorded capture");
    assert_eq!(
        golden, capture,
        "recorded capture (parity/cross-service/expected.txt) must be byte-identical \
         to the golden — re-bless both together"
    );
}
