# Estuda UVA Direct Peer Synchronization Implementation Plan

> **Status: user-approved requirement; implementation pending.** The user explicitly approved decentralized synchronization among Estuda UVA installations, in addition to local SQLite–Markdown–JSON synchronization.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let two or more opted-in Estuda UVA installations directly synchronize academic records and eligible Markdown/JSON material while each computer keeps its own SQLite database and remains usable offline.

**Architecture:** Reuse the workspace `SyncEngine` and its record IDs, projections, conflicts, and tombstones. Each installation has a machine-local device identity and causal version vectors; peers pair directly over an explicitly opened LAN/VPN TCP connection with an expiring one-time code, mutually authenticated TLS, and pinned public keys. A bounded manifest exchange transfers only missing eligible text projections and logical record changes; the peer never copies a live SQLite database.

**Tech Stack:** Rust 1.90, Tauri 2.12.1, Tokio with `net`/`io-util`, `rustls` 0.23.45, `tokio-rustls` 0.26.6, `rcgen` 0.14.10, `zeroize` 1.8.2, existing SQLite/Serde/sync modules, React 19.3, TypeScript 7. No central service, relay, or Node runtime is used for peer sync.

**Spec:** `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md` and `docs/superpowers/specs/2026-10-09-catalogo-academico-sincronizacao-design.md`

## Global Constraints

- Pairing and synchronization are user-started. A listener exists only during a short pairing window or an active transfer.
- Connections are direct to an address supplied/displayed by a paired device on the same LAN or a reachable VPN/network; there is no account, central server, cloud relay, or public port mapping.
- Pair with an expiring one-time code and exchange/pin both device public-key fingerprints. Require mutually authenticated TLS and reject unknown/revoked peers.
- Exchange logical records and eligible `.md`/`.json` contents only. Never copy `.estuda-uva.sqlite3`, `-wal`, `-shm`, credentials, cookies, browser state, machine-specific settings, temporary PDFs, or excluded raw originals.
- Reuse the shared `SyncEngine`; do not add a second merge, ID, projection, or conflict implementation.
- Version vectors distinguish causally ordered from concurrent peer revisions. Causally newer content may advance; concurrent edits merge only when field/body adapters prove they are disjoint, otherwise preserve both versions as a visible conflict.
- Validate every received relative path beneath the receiving archive root, reject symlink escapes, and use atomic writes. A different existing file is preserved and reported as a conflict.
- Bound each transferred file to 64 MiB and each explicit sync batch to 256 MiB; expose count/size limit results rather than silently omitting entries.
- Peer state and private keys stay in machine-local app data, are excluded from Git, and are not included in Markdown/JSON projections.
- Do not stage, commit, push, publish, initialize Git, or alter branches automatically.

## Review Focus

- An unauthenticated device guesses/replays a code or uses a revoked certificate — reject before manifest exchange and transfer no data.
- Two devices edit the same record while offline — preserve both concurrent versions as a conflict and keep the common ancestor.
- A remote path contains `..`, an absolute path, or a symlink escape — reject it without modifying files outside the archive.
- A peer advertises an oversized item or batch — stop at the fixed limits and report the affected record without exhausting memory/disk.
- A connection drops during an item write — leave the old file intact and resume/retry from the hash-verified manifest on the next user-started sync.

---

## File Structure

- `app/src-tauri/src/peer/mod.rs`: pairing state, device identity, service facade, and typed statuses.
- `app/src-tauri/src/peer/protocol.rs`: versioned messages, bounded manifest and transfer framing.
- `app/src-tauri/src/peer/transport.rs`: Tokio TCP plus mutual TLS and certificate pinning.
- `app/src-tauri/src/peer/apply.rs`: eligibility, path validation, atomic receive, and shared-engine handoff.
- `app/src-tauri/src/storage.rs`: peer registry, per-peer cursors, and local device revision counters in machine-local/app database tables.
- `app/src-tauri/src/sync.rs`: `DeviceId` and per-record causal version vector used by local and peer reconciliation.
- `app/src-tauri/src/lib.rs`: pairing/sync Tauri commands and state lifecycle.
- `app/src-tauri/Cargo.toml` and `Cargo.lock`: pinned `rustls`/`tokio-rustls` dependencies and required Tokio I/O/network features.
- `app/src/ui/PeerSync.tsx`, `app/src/ui/peerSyncTypes.ts`, `App.tsx`, and `styles.css`: pair/connect, peer list/revoke, progress, result, and conflict entry points.
- `.gitignore`: machine-local peer identity/key/session artifacts.

### Task 1: Extend the shared revision contract for decentralized changes

**Files:**
- Modify: `app/src-tauri/src/sync.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Test: inline sync tests in `app/src-tauri/src/sync.rs`

**Interfaces:**
- `DeviceId(String)` is generated once per installation and stored in app data.
- `RecordId` remains the stable string ID already used by the shared `SyncEngine`; peer sync does not define another identity format.
- `Fingerprint(String)` is the lowercase hex SHA-256 digest of a peer's DER-encoded self-signed certificate.
- `PairedPeer { device_id: DeviceId, display_name: String, address: SocketAddr, fingerprint: Fingerprint, paired_at_unix_ms: u64, revoked_at_unix_ms: Option<u64> }` contains no private key or pairing code.
- `CausalRelation` is `Before | After | Equal | Concurrent`. `ConflictRef { record_id: String, conflict_id: String }` and `RejectedItem { record_id: Option<String>, reason: String }` are bounded, content-free UI/report references.
- `VersionVector(BTreeMap<DeviceId, u64>)` is carried by each synchronized record version; every local edit increments only the local device counter.
- `VersionVector::relation(&self, other: &Self) -> CausalRelation` returns `Before`, `After`, `Equal`, or `Concurrent` by component-wise comparison.
- `SyncEngine::merge_peer_version(local: RecordVersion, remote: RecordVersion, common: Option<RecordVersion>) -> Result<MergeOutcome, SyncError>` reuses existing record adapters; concurrent incompatible edits return a preserved conflict.
- Extend the schema migration without reinterpreting current `revision` values. Existing records start with the current local device at revision `1` and retain their last common hashes.

- [ ] **Step 1: Add failing tests** `version_vectors_order_causal_edits`, `version_vectors_detect_concurrent_edits`, `peer_newer_version_advances_without_conflict`, `concurrent_disjoint_fields_merge`, and `concurrent_same_field_preserves_conflict`.
- [ ] **Step 2: Run the focused sync tests and confirm causal metadata is not implemented.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib sync::tests`

- [ ] **Step 3: Add the version-vector and local-device schema migration** in `sync.rs`/`storage.rs`; keep device ID/private-key files out of the synchronized database and file projections.
- [ ] **Step 4: Implement `VersionVector::relation` and route local/peer record resolution through `SyncEngine::merge_peer_version`.**
- [ ] **Step 5: Run the focused sync tests.**

  Expected: causal revisions advance; concurrent non-overlapping changes merge deterministically; conflicting values remain intact until explicit user resolution.

### Task 2: Implement secure, expiring direct pairing

**Files:**
- Create: `app/src-tauri/src/peer/mod.rs`
- Create: `app/src-tauri/src/peer/transport.rs`
- Create: `app/src-tauri/src/peer/protocol.rs`
- Modify: `app/src-tauri/Cargo.toml`
- Modify: `app/src-tauri/Cargo.lock`
- Modify: `app/src-tauri/src/lib.rs`
- Test: inline pairing/protocol tests in `peer/mod.rs` and `peer/protocol.rs`

**Interfaces:**
- `PeerSyncService::create_pairing_offer(bind: IpAddr, ttl: Duration) -> Result<PairingOffer, PeerError>` opens an explicit, time-limited listener and returns a QR/manual payload containing address, ephemeral code, and public-key fingerprint. The service tracks an internal monotonic `Instant` deadline as well as the UI timestamp and closes the listener on expiry or cancellation.
- `PeerSyncService::accept_pairing(address: SocketAddr, code: PairingCode, expected_fingerprint: Fingerprint) -> Result<PairedPeer, PeerError>` verifies the code and pinned key before storing the paired peer.
- `PairingCode(Zeroizing<String>)` has redacted `Debug` output and is consumed once; UI progress and errors never echo it.
- `PairingOffer { device_id: DeviceId, display_name: String, address: SocketAddr, fingerprint: Fingerprint, pairing_code: PairingCode, expires_at_unix_ms: u64 }` contains no academic content; its pairing code is shown once, zeroized when consumed or expired, and never persisted/logged.
- `PairingCode` has a custom one-time `Serialize` implementation for the Tauri response, no `Deserialize` implementation, and a redacted `Debug`; the accepting command takes a plain UI string and immediately wraps it in `Zeroizing<String>`.
- `PeerSyncStatus` distinguishes disabled, pairing, paired, connecting, syncing, offline, revoked, and error states; errors never include the one-time code or private key.
- Store machine identity, private key, paired public keys, and revocation state only in machine-local app data. Bind only during user-started pairing/transfer and close the listener after expiry or cancellation.

- [ ] **Step 1: Add failing tests** `pairing_offer_expires`, `wrong_pairing_code_rejects_without_trust`, `fingerprint_mismatch_rejects_before_manifest`, `revoked_peer_is_rejected`, and `unpaired_listener_closes_on_cancel`.
- [ ] **Step 2: Run the focused peer tests.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib peer::tests`

- [ ] **Step 3: Add pinned `rustls`/`tokio-rustls`/`rcgen` dependencies and implement TLS 1.3 certificates** with a distinct app-owned key per device. During pairing, the client pins the displayed host fingerprint, presents its own certificate, and proves the one-time code inside the encrypted channel; the host shows that client's fingerprint for explicit approval before storing either peer. Later syncs require both stored fingerprint pins before manifest exchange.
- [ ] **Step 4: Implement pairing offer/accept and durable peer revocation** with a bounded timeout and no always-on network listener.
- [ ] **Step 5: Run the focused pairing tests.**

  Expected: unknown, expired, mismatched, and revoked peers cannot complete TLS/application pairing; a valid offer succeeds only after both sides confirm fingerprints.

### Task 3: Exchange bounded manifests and apply records through SyncEngine

**Files:**
- Create: `app/src-tauri/src/peer/apply.rs`
- Modify: `app/src-tauri/src/peer/protocol.rs`
- Modify: `app/src-tauri/src/peer/mod.rs`
- Modify: `app/src-tauri/src/storage.rs`
- Modify: `app/src-tauri/src/sync.rs`
- Test: inline protocol/apply tests in `peer/protocol.rs` and `peer/apply.rs`

**Interfaces:**
- `PeerManifestEntry { id: String, surface: RecordSurface, relative_path: Option<String>, size_bytes: u64, sha256: String, version: VersionVector, tombstone: bool }` contains metadata only and uses the shared SyncEngine ID unchanged.
- `PeerSyncService::sync_now(peer_id: &str) -> Result<PeerSyncReport, PeerError>` exchanges manifests, requests missing hashes, verifies bounded payload hashes, and applies versions through `SyncEngine`.
- `ConflictRef { conflict_id: String, record_id: String }` points at the shared SyncEngine conflict record.
- `PeerSyncReport { records_received: u32, files_received: u32, bytes_received: u64, unchanged: u32, conflicts: Vec<ConflictRef>, rejected: Vec<RejectedItem>, duration_ms: u64 }` is safe to show/log without content or credentials.
- Eligibility is limited to synchronized academic logical records and `.md`/eligible `.json` projections under configured source/mirror roots. Never send local SQLite files, browser state, private keys, app settings, temporary PDFs, excluded originals, or unsupported binaries.
- Enforce 64 MiB per item and 256 MiB per explicit batch before allocation. Verify SHA-256 before writing. Use temp-file + fsync + atomic rename and register sync-origin hashes to prevent watcher echo loops.

- [ ] **Step 1: Add failing tests** `manifest_contains_hashes_not_contents`, `sync_requests_only_missing_hashes`, `oversized_item_is_rejected_before_allocation`, `batch_limit_reports_remaining_items`, `remote_traversal_path_is_rejected`, `interrupted_write_preserves_old_file`, `matching_hash_is_noop`, and `peer_divergence_uses_shared_conflict_record`.
- [ ] **Step 2: Run focused protocol/apply tests and confirm bounded peer transfer is absent.**

  Run: `cargo test --manifest-path app/src-tauri/Cargo.toml --lib peer::`

- [ ] **Step 3: Implement versioned length-delimited protocol messages** for handshake, manifest pages, hash requests, bounded content frames, acknowledgements, and cancellation.
- [ ] **Step 4: Implement eligibility/path/hash validation and atomic receiving**; route verified records/files through `SyncEngine` and normal journal/outbox recovery.
- [ ] **Step 5: Implement per-peer acknowledgements and retry cursor** so a dropped connection resumes at the first missing hash without duplicating records.
- [ ] **Step 6: Run focused protocol/apply tests.**

  Expected: only eligible missing material transfers, interrupted writes preserve the old file, and divergent revisions appear in the existing conflict list.

### Task 4: Add the user-controlled peer interface

**Files:**
- Create: `app/src/ui/peerSyncTypes.ts`
- Create: `app/src/ui/PeerSync.tsx`
- Modify: `app/src/ui/App.tsx`
- Modify: `app/src/ui/styles.css`
- Modify: `app/src-tauri/src/lib.rs`

**Interfaces:**
- Tauri commands: `peer_sync_status()`, `create_peer_pairing_offer(bind_address)`, `accept_peer_pairing(address, code, expected_fingerprint)`, `list_paired_peers()`, `revoke_paired_peer(peer_id)`, and `sync_peer_now(peer_id)`.
- `PeerSync` shows a QR/manual address and fingerprint during pairing, requires visible peer confirmation, lists paired computers and last sync, supports explicit sync/revoke, and reports transferred/skipped/conflicted/rejected counts.
- The UI explains direct reachability limits: same LAN or reachable VPN/network. Offline peers remain queued and no relay is attempted.
- The UI never displays pairing codes after expiry and never includes credentials or raw file contents in progress/errors.

- [ ] **Step 1: Implement typed command/status adapters and the peer panel** with clear disabled/offline/error states.
- [ ] **Step 2: Wire pairing, confirmation, revoke, and explicit sync actions** to the Rust service without enabling an always-on listener.
- [ ] **Step 3: Build the frontend.**

  Run: `npm run build --prefix app`

  Expected: the app shows setup, paired peers, last result, and conflict counts without blocking the local archive.

## Integration Order

Run this plan after the repository-local `AcademicStore` and shared `SyncEngine` are implemented by `docs/superpowers/plans/2026-10-09-estuda-uva-sync-git.md`. It does not depend on Canvas or UVA portal authentication. A pair of running app builds on the same LAN can verify the end-to-end flow; VPN/restricted-network reachability remains network-dependent.

## No automatic publication

Leave all working-tree changes unstaged and uncommitted. The user reviews Git status and chooses whether to stage or commit.

## External references

- `rcgen` 0.14.10 generates self-signed X.509 certificates and serializable private keys: https://docs.rs/rcgen/0.14.10/rcgen/struct.CertificateParams.html
- `tokio-rustls` supplies async TLS over Tokio I/O; use its certificate-verifier APIs with the explicitly paired fingerprint pins: https://docs.rs/tokio-rustls/0.26.6/tokio_rustls/
