//! `fetch-archive` — download an archive and, optionally, verify its
//! sha256 against a sidecar file.
//!
//! The procedural use case is the `/ductus` bootstrap installer
//! (scenario `ductus-bootstrap` on spec 022). The bootstrap fetches
//! GitHub's auto-generated source tarball at the ref its run resolved
//! (`codeload.github.com/<owner>/<repo>/tar.gz/{archive-ref}`, spec 061),
//! which has no companion sha256 sidecar on any ref. Other procedural
//! callers (release-asset installers, future runtime auto-update) do
//! have sidecars and want verification.
//!
//! Behavior:
//!
//! - `sha256_url = Some(_)`: download the archive, fetch the sidecar,
//!   parse its leading hex token, compare against the computed sha. A
//!   mismatch raises [`PrimitiveError::ChecksumMismatch`] *before* the
//!   archive is written — nothing lands on disk. The error's `path`
//!   names the destination the archive would have been written to
//!   (kept for message compatibility and retry context); that file is
//!   never created.
//! - `sha256_url = None`: download the archive, compute the sha256,
//!   return it in the result with `verified: false`. The host can
//!   compare against a known-good digest out-of-band if it cares.
//!
//! Either way the result includes the computed digest and a
//! `verified: bool` so the host knows whether to trust it.
//!
//! Response bodies are capped at `MAX_FETCH_BYTES`. A body that exceeds
//! the cap raises an operational error naming the cap and the archive is
//! not written — never a silent truncation, which would otherwise carry
//! a self-consistent computed sha and only surface as corruption later
//! in `extract-archive`. A body of exactly the cap size succeeds.
//!
//! The primitive is blocking — it spins reqwest's blocking client per
//! call rather than dragging an async runtime through the primitive
//! surface. Network failures map to [`PrimitiveError::Http`] (transport)
//! or [`PrimitiveError::HttpStatus`] (non-2xx).
//!
//! Outbound requests are guarded against SSRF before a connection is
//! opened ([`validate_fetch_url`]): the scheme is allowlisted to `https`
//! (which also satisfies the transport-encryption rule), and the host is
//! resolved with `std::net` so every candidate address can be screened —
//! any URL resolving to a loopback, link-local (including the
//! `169.254.169.254` cloud-metadata endpoint), RFC-1918 private,
//! unique-local, or unspecified address is rejected. IPv4-mapped IPv6
//! addresses are unwrapped so a `::ffff:127.0.0.1` literal cannot smuggle
//! an internal target past the v4 screen.
//!
//! The HTTP client reads no environment variable per fetch (`CFG-ENV-001`;
//! spec 048, scenario `fetch-archive-reads-its-proxy-once`). The proxy
//! variables are captured once at startup ([`init_proxy_env`]) and applied as
//! explicit proxies, which turns off reqwest's own per-build read of them. On
//! Linux and other non-Apple Unix, where the certificate verifier would read
//! `SSL_CERT_FILE` and `SSL_CERT_DIR` on every build, the system roots are
//! loaded once, at the process's first fetch, and handed to every build.
//!
//! **Through a proxy, the operator's proxy is the trust boundary.** The
//! screen above still runs on the first URL and every redirect hop, so
//! nothing it refuses is fetched through a proxy. A direct connection — no
//! proxy set, or a host `NO_PROXY` names — stays pinned to the screened
//! addresses. A proxied one is not: reqwest sends the hostname in its
//! `CONNECT`, the proxy resolves it, and what it resolves and reaches is the
//! policy of the operator who configured it.
//!
//! The sidecar format matches `shasum -a 256` output: one or more lines
//! shaped `<hex>  <filename>`. Only the first hex digest is consulted;
//! filename column is informational.

use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use sha2::{Digest, Sha256};

use crate::primitives::{PrimitiveError, Result, resolve_path, write_atomic_bytes};
use crate::schema::primitives::{FetchArchiveArgs, FetchArchiveResult};

/// Execute the `fetch-archive` primitive.
///
/// # Errors
///
/// - [`PrimitiveError::Http`] / [`PrimitiveError::HttpStatus`] on network failure.
/// - [`PrimitiveError::Io`] on local filesystem failures writing the archive,
///   when the response body exceeds the `MAX_FETCH_BYTES` fetch cap (the
///   error message names the cap; the archive is not written), or when the
///   URL is refused by the SSRF/transport guard ([`validate_fetch_url`] —
///   non-`https` scheme or a host resolving to an internal address).
/// - [`PrimitiveError::MalformedSidecar`] when the sidecar's first hex token isn't a valid 64-char sha256.
/// - [`PrimitiveError::ChecksumMismatch`] when the computed sha differs from
///   the sidecar. Raised before the archive is written; the error's `path`
///   is the intended destination, which is never created.
pub fn run(args: &FetchArchiveArgs, repo: &Path) -> Result<FetchArchiveResult> {
    let dest = resolve_path(repo, &args.archive);

    let body = fetch_bytes(&args.url)?;
    let computed = sha256_hex(&body);

    let verified = match &args.sha256_url {
        Some(sidecar_url) => {
            let sidecar = fetch_text(sidecar_url)?;
            let expected = parse_sidecar_hex(&sidecar, sidecar_url)?;
            if computed != expected {
                // Fail before write: `path` carries the *intended*
                // destination (kept for error-message compatibility and
                // retry context), but no file is created there.
                return Err(PrimitiveError::ChecksumMismatch {
                    path: dest,
                    expected,
                    actual: computed,
                });
            }
            true
        }
        None => false,
    };

    let bytes = u64::try_from(body.len()).unwrap_or(u64::MAX);
    write_atomic_bytes(&dest, &body)?;

    Ok(FetchArchiveResult {
        path: dest.to_string_lossy().into_owned(),
        sha256: computed,
        verified,
        bytes,
    })
}

fn fetch_bytes(url: &str) -> Result<Vec<u8>> {
    fetch_bytes_with(url, PROXY_ENV.get().unwrap_or(&ProxyEnv::NONE))
}

/// [`fetch_bytes`] through the proxy configuration `proxies` — the seam a
/// test drives with a configuration of its own, since the startup capture
/// cannot be set from inside a test process.
fn fetch_bytes_with(url: &str, proxies: &ProxyEnv) -> Result<Vec<u8>> {
    let screen = validate_fetch_url(url)?;
    // The default reqwest client follows up to 10 redirects with no
    // re-validation, so a `302 Location:` pointing at `http://` or an
    // internal address would defeat the guard applied to the initial URL.
    // A custom policy re-runs `validate_fetch_url` on every hop's target
    // (and re-caps the redirect count the custom policy would otherwise
    // uncap). Redirects are still followed — GitHub release assets redirect
    // to codeload/S3 — but only to targets that pass the same screen.
    let mut builder =
        reqwest::blocking::Client::builder().redirect(reqwest::redirect::Policy::custom(
            |attempt| match redirect_verdict(attempt.url().as_str(), attempt.previous().len()) {
                RedirectVerdict::Follow => attempt.follow(),
                RedirectVerdict::Refuse(reason) => attempt.error(reason),
            },
        ));
    // Pin the connection to the exact addresses `validate_fetch_url` already
    // screened, so reqwest connects to one of those rather than re-resolving
    // the host independently at connect time. Without the pin, a host that
    // resolved to a public address during validation and an internal one at
    // connect (DNS rebinding) would slip past the SSRF screen — the TOCTOU
    // between resolve-then-block and connect. An operator-allowlisted insecure
    // host is left to resolve normally.
    if let FetchScreen::Pinned { host, addrs } = &screen {
        builder = builder.resolve_to_addrs(host, addrs);
    }
    builder = apply_proxies(builder, proxies);
    if VERIFIER_READS_CERT_ENV {
        let roots = system_roots().map_err(|reason| fetch_refused(url, reason))?;
        builder = builder.tls_certs_only(roots.iter().cloned());
    }
    let client = builder.build().map_err(|source| PrimitiveError::Http {
        url: url.into(),
        source,
    })?;
    let response = client
        .get(url)
        .send()
        .map_err(|source| PrimitiveError::Http {
            url: url.into(),
            source,
        })?;
    let status = response.status();
    if !status.is_success() {
        return Err(PrimitiveError::HttpStatus {
            url: url.into(),
            status: status.as_u16(),
        });
    }
    read_capped(response, MAX_FETCH_BYTES, url)
}

/// Outcome of screening one redirect hop's target URL.
enum RedirectVerdict {
    Follow,
    Refuse(String),
}

/// Decide whether a redirect hop may be followed. Rejects once the hop
/// count reaches [`MAX_FETCH_REDIRECTS`], then re-applies the full
/// [`validate_fetch_url`] SSRF/scheme screen to the hop's target — so a
/// `3xx` pointing at `http://` or an internal address is refused mid-chain,
/// not just at the initial URL.
fn redirect_verdict(url: &str, prior_hops: usize) -> RedirectVerdict {
    if prior_hops >= MAX_FETCH_REDIRECTS {
        return RedirectVerdict::Refuse(format!("exceeded {MAX_FETCH_REDIRECTS} redirects"));
    }
    match validate_fetch_url(url) {
        Ok(_) => RedirectVerdict::Follow,
        Err(err) => RedirectVerdict::Refuse(err.to_string()),
    }
}

/// The screened connection target for a validated fetch URL.
///
/// A screened host carries the resolved, internal-range-screened socket
/// addresses so the caller can *pin* the connection to them — closing the
/// DNS-rebinding TOCTOU between [`validate_fetch_url`]'s resolution and
/// reqwest's own connect-time resolution.
#[derive(Debug)]
enum FetchScreen {
    /// An operator-allowlisted insecure host (`DUCTUS_FETCH_ALLOW_INSECURE_HOSTS`):
    /// resolve and connect normally, no pinning.
    Unpinned,
    /// A screened host: pin the connection to `addrs`. Every address already
    /// passed the internal-range screen, so a re-resolution to an internal
    /// address cannot occur.
    Pinned {
        host: String,
        addrs: Vec<SocketAddr>,
    },
}

/// SSRF/transport guard applied to every outbound fetch before a socket is
/// opened. Enforces (a) an `https`-only scheme allowlist — rejecting
/// `http`, `file`, `ftp`, and everything else — and (b) an internal-range
/// denial: the host is resolved via `std::net` and *every* candidate
/// address is screened, so the URL is refused if any resolved address is
/// loopback, link-local, RFC-1918 private, IPv6 unique-local, or
/// unspecified (covering the `169.254.169.254` cloud-metadata endpoint,
/// which is link-local). A rejection is an operational error naming the
/// reason; no request is made.
///
/// On success it returns the [`FetchScreen`] the caller pins the connection
/// to: the resolve-then-block screen and the actual connect must agree on the
/// address, or a host that rebinds (public at validation, internal at connect)
/// would slip past. The screened addresses are exactly those pinned.
fn validate_fetch_url(url: &str) -> Result<FetchScreen> {
    let parsed = reqwest::Url::parse(url)
        .map_err(|err| fetch_refused(url, &format!("is not a valid URL: {err}")))?;
    // Explicit opt-in escape hatch for internal mirrors and local testing.
    // The SSRF rule (BE-INPUT-007) denies internal ranges *by default*; a
    // host named in `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` (comma-separated) is
    // exempted from both the https-only and internal-address screens. Empty
    // by default, so the secure posture holds unless an operator opts in.
    // Only a URL that actually carries a host can be allowlisted; hostless
    // URLs (e.g. `file://`) fall through to the scheme rejection below.
    if let Some(host) = parsed.host_str()
        && host_is_insecure_allowed(host)
    {
        return Ok(FetchScreen::Unpinned);
    }
    if parsed.scheme() != "https" {
        return Err(fetch_refused(
            url,
            &format!(
                "scheme `{}` is not permitted; only https is allowed",
                parsed.scheme()
            ),
        ));
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| fetch_refused(url, "has no host to resolve"))?;
    // `url` returns IPv6 hosts in bracketed form (`[::1]`); `ToSocketAddrs`
    // parses the bare literal, so strip a matched `[` … `]` pair.
    let host = host
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .unwrap_or(host);
    let port = parsed.port_or_known_default().unwrap_or(443);
    let resolved = (host, port)
        .to_socket_addrs()
        .map_err(|err| fetch_refused(url, &format!("host `{host}` did not resolve: {err}")))?;
    let mut addrs = Vec::new();
    for addr in resolved {
        if is_internal_ip(addr.ip()) {
            return Err(fetch_refused(
                url,
                &format!(
                    "resolves to internal address {} (loopback / link-local / private / metadata ranges are denied)",
                    addr.ip()
                ),
            ));
        }
        addrs.push(addr);
    }
    if addrs.is_empty() {
        return Err(fetch_refused(
            url,
            &format!("host `{host}` resolved to no addresses"),
        ));
    }
    Ok(FetchScreen::Pinned {
        host: host.to_string(),
        addrs,
    })
}

/// Name of the environment variable holding the comma-separated
/// insecure-host allowlist consulted by [`validate_fetch_url`].
///
/// Renamed from `GVRN_*` in the 049 sweep, which missed it: the constant's
/// *name* carried no project token, so a grep for the old project name over
/// identifiers passed straight over the string literal. The old name is not
/// read as a fallback, deliberately. Honouring it would leave a live
/// reference to the retired project name — the thing 049's AC1 forbids — and
/// the failure mode of dropping it is safe and loud rather than silent: a
/// host that was exempt under the old variable is now screened, so the fetch
/// is *refused* with an error naming the scheme or the internal address,
/// never quietly allowed. The variable only ever loosens the SSRF guard, so
/// losing it fails closed.
const FETCH_ALLOW_INSECURE_HOSTS_ENV: &str = "DUCTUS_FETCH_ALLOW_INSECURE_HOSTS";

/// The allowlist as this process's environment held it at startup, captured
/// once by [`init_insecure_hosts`] (`CFG-ENV-001`: read once at startup and
/// cached; listed in `docs/runtime.md`'s environment-variable inventory).
static INSECURE_HOSTS: OnceLock<Option<String>> = OnceLock::new();

/// Capture [`FETCH_ALLOW_INSECURE_HOSTS_ENV`] from this process's
/// environment — once, at startup. The binary's `main` calls this before it
/// dispatches anything; a later call is a no-op.
pub fn init_insecure_hosts() {
    INSECURE_HOSTS.get_or_init(|| std::env::var(FETCH_ALLOW_INSECURE_HOSTS_ENV).ok());
}

/// Whether `host` is exempted from the SSRF/scheme screens via the
/// `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` allowlist captured at startup.
/// Matching is exact against each comma-separated, whitespace-trimmed entry.
/// An unset or empty variable exempts nothing (the secure default), and so
/// does a process that never captured it, such as an in-process test.
fn host_is_insecure_allowed(host: &str) -> bool {
    INSECURE_HOSTS
        .get()
        .and_then(Option::as_deref)
        .is_some_and(|list| list.split(',').map(str::trim).any(|entry| entry == host))
}

/// The proxy variables as this process's environment held them at startup —
/// the fields hyper-util's matcher reads on every client build when reqwest's
/// system proxy is left on (hyper-util 0.1.20, `client/proxy/matcher.rs`,
/// `Builder::from_env`), captured once here instead. An empty string is
/// unset, as it is there. Deliberately without `Debug`: a proxy URL can
/// carry credentials, which hyper-util's own `Debug` output omits.
struct ProxyEnv {
    /// `REQUEST_METHOD` was set: a CGI environment, where the convention
    /// ignores every proxy variable.
    cgi: bool,
    /// `ALL_PROXY` / `all_proxy`: the fallback for both schemes.
    all: String,
    /// `HTTP_PROXY` / `http_proxy`.
    http: String,
    /// `HTTPS_PROXY` / `https_proxy`.
    https: String,
    /// `NO_PROXY` / `no_proxy`: hosts reached directly.
    no: String,
}

impl ProxyEnv {
    /// No proxy: what a process that never captured its environment — an
    /// in-process test — fetches with.
    const NONE: Self = Self {
        cgi: false,
        all: String::new(),
        http: String::new(),
        https: String::new(),
        no: String::new(),
    };

    /// Build from a variable `lookup` and whether `REQUEST_METHOD` is set.
    /// Each variable is its uppercase name if set — even to an empty value —
    /// else its lowercase one, which is the matcher's `get_first_env` order.
    fn capture(lookup: impl Fn(&str) -> Option<String>, cgi: bool) -> Self {
        let first = |names: [&str; 2]| names.iter().find_map(|name| lookup(name));
        Self {
            cgi,
            all: first(["ALL_PROXY", "all_proxy"]).unwrap_or_default(),
            http: first(["HTTP_PROXY", "http_proxy"]).unwrap_or_default(),
            https: first(["HTTPS_PROXY", "https_proxy"]).unwrap_or_default(),
            no: first(["NO_PROXY", "no_proxy"]).unwrap_or_default(),
        }
    }

    /// The proxies a fetch applies, in the order reqwest tries them, each as
    /// `(scheme it serves, proxy value)`. Empty when `REQUEST_METHOD` is set
    /// or no proxy variable is. `http` then `https` then `all` keeps the
    /// matcher's selection: a URL takes its own scheme's proxy, else
    /// `ALL_PROXY`.
    fn proxies(&self) -> Vec<(ProxyScheme, &str)> {
        if self.cgi {
            return Vec::new();
        }
        [
            (ProxyScheme::Http, self.http.as_str()),
            (ProxyScheme::Https, self.https.as_str()),
            (ProxyScheme::All, self.all.as_str()),
        ]
        .into_iter()
        .filter(|(_, value)| !value.is_empty())
        .collect()
    }
}

/// Which destinations a captured proxy serves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProxyScheme {
    Http,
    Https,
    All,
}

/// The proxy variables captured at startup by [`init_proxy_env`].
static PROXY_ENV: OnceLock<ProxyEnv> = OnceLock::new();

/// Capture the proxy variables and whether `REQUEST_METHOD` is set from this
/// process's environment — once, at startup (`CFG-ENV-001`). The binary's
/// `main` calls this before it dispatches anything; a later call is a no-op.
pub fn init_proxy_env() {
    PROXY_ENV.get_or_init(|| {
        ProxyEnv::capture(
            |name| std::env::var(name).ok(),
            std::env::var_os("REQUEST_METHOD").is_some(),
        )
    });
}

/// Apply `env` to a client under construction: every proxy it names, each
/// with its `NO_PROXY` exemptions, or none. Either way reqwest's system proxy
/// is off — `no_proxy` and `proxy` both clear its `auto_sys_proxy` (reqwest
/// 0.13.4, `async_impl/client.rs`) — so the build reads no environment
/// variable. `Proxy::http`, `https` and `all` compile to the matcher the
/// system proxy would have built (`proxy.rs`, `into_matcher`); a value reqwest
/// cannot parse is skipped, as the matcher skips one it cannot.
fn apply_proxies(
    builder: reqwest::blocking::ClientBuilder,
    env: &ProxyEnv,
) -> reqwest::blocking::ClientBuilder {
    let mut builder = builder.no_proxy();
    for (scheme, value) in env.proxies() {
        let proxy = match scheme {
            ProxyScheme::Http => reqwest::Proxy::http(value),
            ProxyScheme::Https => reqwest::Proxy::https(value),
            ProxyScheme::All => reqwest::Proxy::all(value),
        };
        if let Ok(proxy) = proxy {
            builder = builder.proxy(proxy.no_proxy(reqwest::NoProxy::from_string(&env.no)));
        }
    }
    builder
}

/// Whether this platform's certificate verifier reads `SSL_CERT_FILE` and
/// `SSL_CERT_DIR` on every client build. On Linux and other non-Apple Unix,
/// `rustls-platform-verifier` loads the system store through
/// `rustls_native_certs::load_native_certs` in its constructor
/// (rustls-platform-verifier 0.7.0, `verification/others.rs`). Apple
/// platforms and Windows verify through the operating system and read no
/// certificate variable, so they keep the platform verifier.
const VERIFIER_READS_CERT_ENV: bool = cfg!(all(
    unix,
    not(target_vendor = "apple"),
    not(target_os = "android")
));

/// The system roots, loaded at the process's first fetch and kept.
static SYSTEM_ROOTS: OnceLock<std::result::Result<Vec<reqwest::Certificate>, String>> =
    OnceLock::new();

/// The system certificate roots, loaded once per process. There the platform
/// verifier is `WebPKI` over exactly these roots, and `tls_certs_only` builds
/// the same `WebPKI` verifier over the roots it is given (reqwest 0.13.4,
/// `async_impl/client.rs`, the `with_root_certificates` branch), so handing
/// them over changes what is trusted in no way — only how often the store is
/// read.
///
/// # Errors
///
/// When the store yields no usable certificate, naming that, as the platform
/// verifier refuses an empty store.
fn system_roots() -> std::result::Result<&'static [reqwest::Certificate], &'static str> {
    load_once(&SYSTEM_ROOTS, || {
        roots_from(rustls_native_certs::load_native_certs().certs)
    })
}

/// `cell`'s value, computing it with `load` the first time only.
fn load_once<T>(
    cell: &'static OnceLock<std::result::Result<Vec<T>, String>>,
    load: impl FnOnce() -> std::result::Result<Vec<T>, String>,
) -> std::result::Result<&'static [T], &'static str> {
    cell.get_or_init(load).as_deref().map_err(String::as_str)
}

/// The certificates of `certs` a rustls root store accepts, as reqwest
/// certificates. The rest are skipped, as the platform verifier skips them
/// (`add_parsable_certificates`): reqwest's own root store refuses the whole
/// client on one it cannot add (`tls.rs`, `rustls_store`).
///
/// # Errors
///
/// When none is usable.
fn roots_from(
    certs: Vec<rustls::pki_types::CertificateDer<'static>>,
) -> std::result::Result<Vec<reqwest::Certificate>, String> {
    let roots: Vec<reqwest::Certificate> = certs
        .into_iter()
        .filter(|der| rustls::RootCertStore::empty().add(der.clone()).is_ok())
        // `from_der` only stores the bytes on the rustls backend, so it
        // cannot fail for a certificate the root store just accepted.
        .filter_map(|der| reqwest::Certificate::from_der(der.as_ref()).ok())
        .collect();
    if roots.is_empty() {
        return Err(
            "no CA certificate could be loaded from the system store, `SSL_CERT_FILE` or \
             `SSL_CERT_DIR`"
                .to_owned(),
        );
    }
    Ok(roots)
}

/// Build the operational error for a refused fetch URL. Reuses
/// [`PrimitiveError::Io`] (as [`read_capped`] does for its cap breach) so
/// no new error variant is needed; the `path` carries the offending URL.
fn fetch_refused(url: &str, reason: &str) -> PrimitiveError {
    PrimitiveError::Io {
        path: PathBuf::from(url),
        source: std::io::Error::other(format!("refusing to fetch `{url}`: {reason}")),
    }
}

/// Classify a resolved address as internal (must-not-be-fetched) per the
/// SSRF denial ranges. Delegates to per-family helpers; IPv4-mapped IPv6
/// addresses are unwrapped to their v4 form first so they cannot bypass
/// the v4 screen.
fn is_internal_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_internal_v4(v4),
        IpAddr::V6(v6) => is_internal_v6(v6),
    }
}

/// Internal-range test for IPv4: loopback `127.0.0.0/8`, link-local
/// `169.254.0.0/16` (which contains the `169.254.169.254` metadata
/// endpoint), RFC-1918 private (`10/8`, `172.16/12`, `192.168/16`),
/// broadcast, and the unspecified `0.0.0.0`.
fn is_internal_v4(ip: Ipv4Addr) -> bool {
    let [a, b, ..] = ip.octets();
    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_unspecified()
        // 0.0.0.0/8 "this network" (`is_unspecified` covers only 0.0.0.0).
        || a == 0
        // 100.64.0.0/10 carrier-grade NAT (`Ipv4Addr::is_shared` is unstable).
        || (a == 100 && (64..=127).contains(&b))
}

/// Internal-range test for IPv6: loopback `::1`, unspecified `::`,
/// unique-local `fc00::/7`, and link-local `fe80::/10`. `is_unique_local`
/// and `is_unicast_link_local` are unstable on the pinned toolchain, so
/// the two ranges are matched by prefix directly. IPv4-mapped addresses
/// are folded back to the v4 screen.
fn is_internal_v6(ip: Ipv6Addr) -> bool {
    if let Some(v4) = ip.to_ipv4_mapped() {
        return is_internal_v4(v4);
    }
    ip.is_loopback() || ip.is_unspecified() || is_unique_local_v6(ip) || is_link_local_v6(ip)
}

/// `fc00::/7` — the top 7 bits are `1111110`, i.e. the first byte is
/// `0xfc` or `0xfd`.
fn is_unique_local_v6(ip: Ipv6Addr) -> bool {
    (ip.octets()[0] & 0xfe) == 0xfc
}

/// `fe80::/10` — the top 10 bits are `1111111010`.
fn is_link_local_v6(ip: Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xffc0) == 0xfe80
}

/// Read at most `cap` bytes from `reader`, erroring when the stream holds
/// more. Reads `cap + 1` bytes so an over-cap body is *detected* and
/// rejected with an error naming the cap, rather than silently truncated
/// (a truncated archive would carry a self-consistent computed sha and
/// only surface as corruption later). A stream of exactly `cap` bytes
/// succeeds.
fn read_capped(reader: impl Read, cap: u64, url: &str) -> Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    reader
        .take(cap.saturating_add(1))
        .read_to_end(&mut buf)
        .map_err(|source| PrimitiveError::Io {
            path: PathBuf::from(url),
            source,
        })?;
    if u64::try_from(buf.len()).unwrap_or(u64::MAX) > cap {
        return Err(PrimitiveError::Io {
            path: PathBuf::from(url),
            source: std::io::Error::other(format!(
                "response body exceeds the fetch cap of {cap} bytes; archive not written"
            )),
        });
    }
    Ok(buf)
}

fn fetch_text(url: &str) -> Result<String> {
    let bytes = fetch_bytes(url)?;
    String::from_utf8(bytes).map_err(|err| PrimitiveError::MalformedSidecar {
        url: url.into(),
        reason: format!("not valid UTF-8: {err}"),
    })
}

/// Parse the first `<hex>  <filename>` line of a `shasum -a 256` sidecar.
///
/// # Errors
///
/// Returns [`PrimitiveError::MalformedSidecar`] when the body has no parsable
/// line or the leading token isn't a 64-char lowercase-hex string.
pub(crate) fn parse_sidecar_hex(body: &str, url: &str) -> Result<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let token = trimmed.split_whitespace().next().unwrap_or("");
        if is_sha256_hex(token) {
            return Ok(token.to_ascii_lowercase());
        }
        return Err(PrimitiveError::MalformedSidecar {
            url: url.into(),
            reason: format!("leading token `{token}` is not 64-char lowercase hex"),
        });
    }
    Err(PrimitiveError::MalformedSidecar {
        url: url.into(),
        reason: "sidecar body is empty or has no non-comment line".into(),
    })
}

fn is_sha256_hex(token: &str) -> bool {
    token.len() == 64
        && token
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c) || ('A'..='F').contains(&c))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    super::hex(&hasher.finalize())
}

/// Cap the per-fetch body size to ~256 MiB. Framework release tarballs
/// are well under 50 MiB; this ceiling defends against accidentally
/// streaming an unbounded URL into memory. A body exceeding the cap is
/// an operational error (see [`read_capped`]), never a silent truncation.
const MAX_FETCH_BYTES: u64 = 256 * 1024 * 1024;

/// Upper bound on HTTP redirects followed by [`fetch_bytes`]. Matches
/// reqwest's default limit; a custom redirect policy replaces the default,
/// so the cap is re-declared here alongside the per-hop SSRF re-validation.
const MAX_FETCH_REDIRECTS: usize = 10;

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use std::io::Cursor;
    use std::net::IpAddr;

    use super::{
        FetchScreen, MAX_FETCH_REDIRECTS, ProxyEnv, ProxyScheme, RedirectVerdict, fetch_bytes_with,
        host_is_insecure_allowed, is_internal_ip, is_sha256_hex, load_once, parse_sidecar_hex,
        read_capped, redirect_verdict, roots_from, sha256_hex, validate_fetch_url,
    };

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn sha256_hex_matches_known_vector() {
        // sha256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn parse_sidecar_extracts_leading_hex() {
        let sidecar =
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  release.tar.gz\n";
        let got = parse_sidecar_hex(sidecar, "http://t").unwrap();
        assert_eq!(
            got,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn parse_sidecar_lowercases_hex() {
        let sidecar =
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD  release.tar.gz";
        let got = parse_sidecar_hex(sidecar, "http://t").unwrap();
        assert!(got.chars().all(|c| !c.is_ascii_uppercase()));
    }

    #[test]
    fn parse_sidecar_skips_comments_and_blanks() {
        let sidecar = "# this is a comment\n\nba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  release.tar.gz\n";
        let got = parse_sidecar_hex(sidecar, "http://t").unwrap();
        assert!(got.starts_with("ba7816"));
    }

    #[test]
    fn parse_sidecar_rejects_short_token() {
        let err = parse_sidecar_hex("abc123  release.tar.gz", "http://t").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("malformed"), "got: {msg}");
    }

    #[test]
    fn parse_sidecar_rejects_empty_body() {
        let err = parse_sidecar_hex("", "http://t").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("empty"), "got: {msg}");
    }

    #[test]
    fn read_capped_accepts_body_under_cap() {
        let got = read_capped(Cursor::new(vec![7u8; 4]), 8, "http://t").unwrap();
        assert_eq!(got, vec![7u8; 4]);
    }

    #[test]
    fn read_capped_accepts_body_of_exactly_cap_size() {
        let got = read_capped(Cursor::new(vec![7u8; 8]), 8, "http://t").unwrap();
        assert_eq!(got.len(), 8);
    }

    #[test]
    fn read_capped_rejects_body_exceeding_cap_naming_the_cap() {
        let err = read_capped(Cursor::new(vec![7u8; 9]), 8, "http://t").unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("fetch cap of 8 bytes"),
            "error must name the cap; got: {msg}"
        );
    }

    #[test]
    fn is_sha256_hex_recognizes_valid_digests() {
        assert!(is_sha256_hex(
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        ));
        assert!(is_sha256_hex(
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
        ));
        assert!(!is_sha256_hex("ba7816bf"));
        assert!(!is_sha256_hex(
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015agz"
        ));
    }

    // -- SSRF / scheme guard (all offline: IP literals resolve without DNS,
    //    and scheme rejection short-circuits before any resolution) --------

    #[test]
    fn validate_fetch_url_rejects_non_https_scheme() {
        // `http://` (and any non-https scheme) is refused before a socket
        // is opened. Uses an IP literal so no DNS is attempted even if the
        // scheme check were somehow skipped.
        let err = validate_fetch_url("http://10.0.0.1/main.tar.gz").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("only https is allowed"), "got: {msg}");

        let file = validate_fetch_url("file:///etc/passwd").unwrap_err();
        assert!(file.to_string().contains("only https is allowed"));
    }

    #[test]
    fn redirect_verdict_refuses_downgrade_and_internal_hops() {
        // A 3xx Location pointing at an http:// target is refused mid-chain
        // by the same scheme screen as the initial URL — the SSRF bypass a
        // default redirect-following client would allow. IP literals keep the
        // check offline (scheme is rejected before any DNS).
        match redirect_verdict("http://10.0.0.1/next.tar.gz", 1) {
            RedirectVerdict::Refuse(msg) => assert!(msg.contains("only https is allowed"), "{msg}"),
            RedirectVerdict::Follow => panic!("http redirect target must be refused"),
        }
        // An https redirect to a loopback/metadata literal is refused as an
        // internal target.
        match redirect_verdict("https://169.254.169.254/latest/meta-data/", 1) {
            RedirectVerdict::Refuse(msg) => assert!(msg.contains("internal address"), "{msg}"),
            RedirectVerdict::Follow => panic!("internal redirect target must be refused"),
        }
    }

    #[test]
    fn redirect_verdict_caps_hop_count() {
        // At the hop limit the redirect chain is refused before the target is
        // even screened (so no DNS is attempted).
        match redirect_verdict("https://example.com/loop.tar.gz", MAX_FETCH_REDIRECTS) {
            RedirectVerdict::Refuse(msg) => assert!(msg.contains("redirects"), "{msg}"),
            RedirectVerdict::Follow => panic!("hop-count cap must refuse"),
        }
    }

    #[test]
    fn insecure_host_allowlist_is_empty_by_default() {
        // The test process never captures the variable at startup, so
        // nothing is exempted — the secure default holds. The allow path is
        // exercised end-to-end by the ductus-basic parity subprocess, which
        // sets DUCTUS_FETCH_ALLOW_INSECURE_HOSTS on its own process env (no
        // in-process env mutation here, which would race sibling tests).
        assert!(!host_is_insecure_allowed("127.0.0.1"));
        assert!(!host_is_insecure_allowed("example.com"));
    }

    #[test]
    fn validate_fetch_url_rejects_https_to_loopback() {
        // An https URL whose host is a loopback literal resolves offline to
        // 127.0.0.1 and is denied as an internal target.
        let err = validate_fetch_url("https://127.0.0.1/main.tar.gz").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("internal address"), "got: {msg}");

        assert!(
            validate_fetch_url("https://[::1]/x")
                .unwrap_err()
                .to_string()
                .contains("internal address")
        );
        // The cloud-metadata endpoint (link-local) is refused.
        assert!(
            validate_fetch_url("https://169.254.169.254/latest/meta-data/")
                .unwrap_err()
                .to_string()
                .contains("internal address")
        );
    }

    #[test]
    fn validate_fetch_url_pins_the_screened_public_address() {
        // scenarios/fetch-archive-dns-rebinding.md: a validated URL yields the
        // exact screened address(es) the connection is then pinned to, so
        // reqwest connects to a screened address instead of re-resolving the
        // host to a possibly-internal one between validation and connect. A
        // public IP literal resolves offline to itself.
        match validate_fetch_url("https://93.184.216.34/main.tar.gz").unwrap() {
            FetchScreen::Pinned { host, addrs } => {
                assert_eq!(host, "93.184.216.34");
                assert!(
                    !addrs.is_empty(),
                    "a screened host must carry pin addresses"
                );
                assert!(
                    addrs.iter().all(|a| a.port() == 443),
                    "the https default port is pinned"
                );
                assert!(
                    addrs.iter().all(|a| !is_internal_ip(a.ip())),
                    "every pinned address must have passed the internal-range screen"
                );
                assert!(addrs.iter().any(|a| a.ip() == ip("93.184.216.34")));
            }
            FetchScreen::Unpinned => panic!("a non-allowlisted https host must be pinned"),
        }
    }

    #[test]
    fn validate_fetch_url_rejects_malformed_url() {
        let err = validate_fetch_url("not a url").unwrap_err();
        assert!(err.to_string().contains("not a valid URL"));
    }

    #[test]
    fn is_internal_ip_flags_each_denied_range() {
        // IPv4: loopback, RFC-1918 (all three blocks), link-local +
        // metadata, unspecified.
        assert!(is_internal_ip(ip("127.0.0.1")));
        assert!(is_internal_ip(ip("10.1.2.3")));
        assert!(is_internal_ip(ip("172.16.0.1")));
        assert!(is_internal_ip(ip("172.31.255.255")));
        assert!(is_internal_ip(ip("192.168.1.1")));
        assert!(is_internal_ip(ip("169.254.0.1")));
        assert!(is_internal_ip(ip("169.254.169.254")));
        assert!(is_internal_ip(ip("0.0.0.0")));
        // 0.0.0.0/8 "this network" (a non-zero host) and 100.64/10 CGNAT.
        assert!(is_internal_ip(ip("0.1.2.3")));
        assert!(is_internal_ip(ip("100.64.0.1")));
        assert!(is_internal_ip(ip("100.127.255.255")));
        // IPv6: loopback, unspecified, unique-local fc00::/7, link-local
        // fe80::/10, and an IPv4-mapped loopback.
        assert!(is_internal_ip(ip("::1")));
        assert!(is_internal_ip(ip("::")));
        assert!(is_internal_ip(ip("fc00::1")));
        assert!(is_internal_ip(ip("fd12:3456::1")));
        assert!(is_internal_ip(ip("fe80::1")));
        assert!(is_internal_ip(ip("::ffff:127.0.0.1")));
    }

    // -- proxy and certificate configuration read once (spec 048, scenario
    //    `fetch-archive-reads-its-proxy-once`) --------------------------------

    fn lookup(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let pairs: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.clone())
        }
    }

    #[test]
    fn a_captured_variable_is_its_uppercase_name_if_set_even_empty() {
        let env = ProxyEnv::capture(
            lookup(&[
                ("HTTPS_PROXY", "http://upper:1"),
                ("https_proxy", "http://lower:1"),
                ("HTTP_PROXY", ""),
                ("http_proxy", "http://lower:2"),
                ("all_proxy", "http://lower:3"),
                ("no_proxy", "internal.example"),
            ]),
            false,
        );
        assert_eq!(env.https, "http://upper:1", "uppercase wins");
        assert_eq!(env.http, "", "a set but empty uppercase value still wins");
        assert_eq!(
            env.all, "http://lower:3",
            "lowercase when uppercase is unset"
        );
        assert_eq!(env.no, "internal.example");
    }

    #[test]
    fn proxies_keep_the_matchers_order_and_skip_unset_values() {
        let env = ProxyEnv::capture(
            lookup(&[
                ("ALL_PROXY", "http://all:1"),
                ("HTTPS_PROXY", "http://tls:1"),
            ]),
            false,
        );
        assert_eq!(
            env.proxies(),
            [
                (ProxyScheme::Https, "http://tls:1"),
                (ProxyScheme::All, "http://all:1")
            ]
        );
        assert_eq!(ProxyEnv::NONE.proxies(), [] as [(ProxyScheme, &str); 0]);
    }

    #[test]
    fn request_method_turns_every_proxy_off() {
        let env = ProxyEnv::capture(lookup(&[("HTTPS_PROXY", "http://tls:1")]), true);
        assert!(env.proxies().is_empty(), "{:?}", env.proxies());
    }

    /// A captured `HTTPS_PROXY` routes an https fetch through that proxy:
    /// the proxy receives the `CONNECT`, naming the target. The target is a
    /// TEST-NET-1 literal, which passes the screen without DNS; the proxy
    /// answers 403, so nothing leaves this machine.
    #[test]
    fn a_captured_proxy_carries_the_fetch() {
        use std::io::{BufRead, BufReader, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let proxy = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();
            let mut line = String::new();
            while reader.read_line(&mut line).unwrap() > 2 {
                line.clear();
            }
            let mut stream = stream;
            stream
                .write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")
                .unwrap();
            tx.send(request_line).unwrap();
        });

        let env = ProxyEnv::capture(lookup(&[("HTTPS_PROXY", proxy.as_str())]), false);
        let result = fetch_bytes_with("https://192.0.2.1/main.tar.gz", &env);
        assert!(result.is_err(), "the proxy refused the tunnel");
        let request_line = rx
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("the fetch never reached the captured proxy");
        assert!(
            request_line.starts_with("CONNECT 192.0.2.1:443 "),
            "{request_line:?}"
        );
    }

    #[test]
    fn roots_are_loaded_once() {
        static CELL: std::sync::OnceLock<std::result::Result<Vec<u8>, String>> =
            std::sync::OnceLock::new();
        let loads = std::sync::atomic::AtomicUsize::new(0);
        let load = || {
            loads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(vec![1])
        };
        assert_eq!(load_once(&CELL, load).unwrap(), [1]);
        assert_eq!(load_once(&CELL, load).unwrap(), [1]);
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn roots_skip_a_certificate_the_store_refuses_and_refuse_none() {
        let good = webpki_root_certs::TLS_SERVER_ROOT_CERTS[0].clone();
        let bad = rustls::pki_types::CertificateDer::from(vec![0x30, 0x03, 0x02, 0x01, 0x00]);
        assert_eq!(roots_from(vec![bad.clone(), good]).unwrap().len(), 1);
        let err = roots_from(vec![bad]).unwrap_err();
        assert!(err.contains("no CA certificate"), "{err}");
    }

    #[test]
    fn is_internal_ip_allows_public_addresses() {
        // A public v4, a public v6 (Cloudflare DNS), and a boundary just
        // outside 172.16/12 stay allowed.
        assert!(!is_internal_ip(ip("8.8.8.8")));
        assert!(!is_internal_ip(ip("172.32.0.1")));
        assert!(!is_internal_ip(ip("172.15.255.255")));
        assert!(!is_internal_ip(ip("2606:4700:4700::1111")));
    }
}
