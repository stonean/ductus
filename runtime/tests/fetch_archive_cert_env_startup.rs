//! `fetch-archive` reads `SSL_CERT_FILE` and `SSL_CERT_DIR` once, at startup
//! (`CFG-ENV-001`; spec 048, `fetch-archive-reads-its-proxy-once`), where the
//! certificate verifier reads them. Removing `init_proxy_env`'s capture leaves
//! `native_certs` falling back to `load_native_certs`, which reads the same two
//! variables at the first fetch, so only a test that changes them between the
//! startup and the fetch can tell the two apart. This is that test, in a
//! process of its own: the captures are process-global `OnceLock`s and the
//! environment is changed in place.
//!
//! Compiled only where `VERIFIER_READS_CERT_ENV` holds (Linux and other
//! non-Apple Unix), the one place the variables are read.

#![cfg(all(unix, not(target_vendor = "apple"), not(target_os = "android")))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use ductus::primitives::fetch_archive::{init_insecure_hosts, init_proxy_env, run};
use ductus::schema::primitives::FetchArchiveArgs;

/// Standard-alphabet base64 with padding: a PEM body's encoding, small enough
/// to write here rather than add a crate for.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0_u32, |n, (i, byte)| n | (u32::from(*byte) << (16 - 8 * i)));
        for (i, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            if i <= chunk.len() {
                out.push(char::from(ALPHABET[((n >> shift) & 63) as usize]));
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[test]
fn the_certificate_variables_are_the_ones_the_process_started_with() {
    // A bundle with one real root, so the startup capture's roots load.
    let root = webpki_root_certs::TLS_SERVER_ROOT_CERTS
        .first()
        .expect("webpki-root-certs holds roots");
    let body = base64(root.as_ref());
    let mut pem = String::from("-----BEGIN CERTIFICATE-----\n");
    for line in body.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(line).unwrap());
        pem.push('\n');
    }
    pem.push_str("-----END CERTIFICATE-----\n");

    let dir = tempfile::tempdir().unwrap();
    let bundle = dir.path().join("bundle.pem");
    std::fs::write(&bundle, pem).unwrap();
    let missing = dir.path().join("no-such-bundle.pem");

    // A port nothing listens on: bound, read, dropped.
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();

    let staging = tempfile::tempdir().unwrap();
    let args = FetchArchiveArgs {
        url: format!("https://127.0.0.1:{port}/x.tar.gz"),
        sha256_url: None,
        archive: staging
            .path()
            .join("x.tar.gz")
            .to_string_lossy()
            .into_owned(),
    };
    let project = tempfile::tempdir().unwrap();

    let result = temp_env::with_vars(
        [
            ("SSL_CERT_FILE", Some(bundle.as_os_str())),
            ("SSL_CERT_DIR", None),
            (
                "DUCTUS_FETCH_ALLOW_INSECURE_HOSTS",
                Some(std::ffi::OsStr::new("127.0.0.1")),
            ),
            ("ALL_PROXY", None),
            ("all_proxy", None),
            ("HTTP_PROXY", None),
            ("http_proxy", None),
            ("HTTPS_PROXY", None),
            ("https_proxy", None),
            ("NO_PROXY", None),
            ("no_proxy", None),
            ("REQUEST_METHOD", None),
        ],
        || {
            init_proxy_env();
            init_insecure_hosts();
            // After startup the variable names a path that does not exist: a
            // fetch that reads it now finds no certificate to trust.
            temp_env::with_var("SSL_CERT_FILE", Some(missing.as_os_str()), || {
                run(&args, project.path())
            })
        },
    );

    let error = result.expect_err("nothing listens on the port").to_string();
    // With the startup capture the roots come from the bundle and the fetch
    // gets as far as connecting; without it the first fetch reads the missing
    // path and refuses for want of a certificate.
    assert!(
        !error.contains("no CA certificate could be loaded"),
        "the variable was read at the fetch, not at startup: {error}"
    );
    assert!(error.starts_with("HTTP error on "), "{error}");
}
