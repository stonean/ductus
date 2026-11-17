"""Mutation list for spec 022 task 50: `fetch-archive` lets a host named in
`DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` skip the internal-address screen over
https, and allows plain http only to a named loopback literal (BE-DATA-001;
`specs/system.md`).

Run with `python3 scripts/mutation/run.py
scripts/mutation/fetch-archive-loopback-http.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused; when the code a mutation anchors on changes,
update the anchor in the same change (`run.py --check`).

The behaviours and the tests that hold them:

  M1  A named host is fetched over plain http whatever its address, as before
      task 50. Killed by fetch_archive's
      `plain_http_reaches_only_a_named_loopback_literal`.
  M2  `is_loopback_literal` accepts any IP literal, so a named private
      address is fetched over http. Killed by the same test.
  M3  `is_loopback_literal` does not strip a bracketed IPv6 host, so `[::1]`
      is refused over http. Killed by the same test.
  M4  A named host is refused over https too, so the internal-address
      exemption reaches no mirror. Killed by the same test (022's review of
      2026-11-17).
  M5  With no NO_PROXY set, the loopback ranges are not on the proxies' list,
      so a named loopback fetch goes through a configured proxy, off this host.
      Killed by fetch_archive's
      `a_named_loopback_host_is_fetched_without_the_proxy`.
  M6  With NO_PROXY naming another host, the same. Killed by the same test.
  M7  `::1` leaves the loopback ranges, so a named `[::1]` fetch goes through
      a configured proxy. Killed by the same test's `[::1]` origin (task 52).

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a named host is fetched over plain http off loopback',
        'src/primitives/fetch_archive.rs',
        '            "http" if is_loopback_literal(host) => return Ok(FetchScreen::Unpinned),\n',
        '            "http" if !host.is_empty() => return Ok(FetchScreen::Unpinned),\n',
        'plain_http_reaches_only_a_named_loopback_literal',
    ),
    (
        'M2 is_loopback_literal accepts any IP literal',
        'src/primitives/fetch_archive.rs',
        '    bare.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())\n',
        '    bare.parse::<IpAddr>().is_ok()\n',
        'plain_http_reaches_only_a_named_loopback_literal',
    ),
    (
        'M3 is_loopback_literal does not strip the brackets of an IPv6 host',
        'src/primitives/fetch_archive.rs',
        "    let bare = host\n        .strip_prefix('[')\n        .and_then(|inner| inner.strip_suffix(']'))\n        .unwrap_or(host);\n    bare.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())\n",
        "    host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())\n",
        'plain_http_reaches_only_a_named_loopback_literal',
    ),
    (
        'M4 a named host is refused over https',
        'src/primitives/fetch_archive.rs',
        '            "https" => return Ok(FetchScreen::Unpinned),\n',
        '            "https" if host.is_empty() => return Ok(FetchScreen::Unpinned),\n',
        'plain_http_reaches_only_a_named_loopback_literal',
    ),
    (
        'M5 with no NO_PROXY, the loopback ranges are not on the list',
        'src/primitives/fetch_archive.rs',
        '            LOOPBACK_NO_PROXY.to_string()\n',
        '            String::new()\n',
        'a_named_loopback_host_is_fetched_without_the_proxy',
    ),
    (
        'M6 with NO_PROXY naming another host, the loopback ranges are not on the list',
        'src/primitives/fetch_archive.rs',
        '            format!("{},{LOOPBACK_NO_PROXY}", self.no)\n',
        '            self.no.clone()\n',
        'a_named_loopback_host_is_fetched_without_the_proxy',
    ),
    (
        'M7 ::1 leaves the loopback ranges',
        'src/primitives/fetch_archive.rs',
        'const LOOPBACK_NO_PROXY: &str = "127.0.0.0/8,::1";\n',
        'const LOOPBACK_NO_PROXY: &str = "127.0.0.0/8";\n',
        'a_named_loopback_host_is_fetched_without_the_proxy',
    ),
]
