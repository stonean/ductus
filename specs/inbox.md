# Inbox

Capture queue for the todos you record by hand with `/log` — known issues,
ideas, and chores not yet assigned to a feature spec. Items are migrated to
their proper home by `/groom` (see the constitution, §brownfield-inbox).

- [ ] Decide whether FE-STORAGE-006's per-user Cache API partition must also be purged at session expiry and revocation, not only at sign-out (FE-PII-004's Clear-Site-Data is a SHOULD); raised by 008's review of 2026-11-20 (008's review.md Summary)
- [ ] Decide whether BE-API-015's Verification should flag a WebSocket opening handshake with no Origin header on a connection authenticated by a credential the browser attaches automatically, such as a cookie, which its Statement's (a) refuses but no flag names; the gap predates 0.60.0; raised by 008's review of 2026-11-20 (008's review.md Summary)
- [ ] Reword accessibility-frontend.md's garbled FE-A11YFORM-002 Not-flagged clause and FE-KBD-001's flagged case written inside its Not-flagged clause, keeping the file at or under 069's 51,200-byte cap (51,182 bytes at bba6502c); raised by 069's second review of 2026-11-20 and waived there as outside its window (069's review.md)
