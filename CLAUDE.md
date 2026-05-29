# CLAUDE.md — markc/amp

Guidance for Claude Code sessions working in `~/.amp/`.

## What this repo is

The Agent Mesh Protocol library family. Three crates: `cosmix-lib-amp` (wire format), `cosmix-lib-client` (broker WebSocket client, native + wasm32), `cosmix-lib-props-core` (SPEC 07 property read surface).

amp is the *protocol layer* — every byte that travels between AMP peers is defined here. It deliberately holds no substrate (storage, TLS, auto-resolve, config-file loaders). Anything that needs files, sockets beyond the broker WebSocket, or persistent state belongs in [cos](https://github.com/markc/cos), not here.

## Layout

```
~/.amp/src/
├── Cargo.toml                          workspace (3 members)
└── crates/
    ├── cosmix-lib-amp/                 AMP wire format
    ├── cosmix-lib-client/              broker WebSocket client
    └── cosmix-lib-props-core/          SPEC 07 read surface
```

## Build / test / lint

amp builds standalone — no sibling repos required.

```sh
cd ~/.amp/src
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The zero-warning baseline is enforced: any new clippy warning is a regression.

## Internal dep graph

- `cosmix-lib-amp` → no internal deps.
- `cosmix-lib-client` → `cosmix-lib-amp` (sibling path).
- `cosmix-lib-props-core` → `cosmix-lib-amp` (sibling path, optional under the `amp` feature).

External consumers (mix, cos, third-party agents) path-dep or version-dep these three crates; amp never depends back.

## What goes here, what doesn't

✅ **Belongs in amp:**
- AMP wire format additions (new message kinds, new field codecs).
- Broker client primitives (`NodedClient` surface, reconnect strategy, request/reply correlation).
- SPEC 07 read-surface types and the AMP-wire dispatcher.
- Standalone unit tests + doctests; manual broker-acceptance examples under `examples/`.

❌ **Doesn't belong in amp:**
- Storage backends, audit, persistence — those live in cos's `cosmix-lib-props-store`.
- TLS, ACME, SNI, certificate machinery — cos's `cosmix-lib-daemon` (tls feature).
- TOML / config-file loaders, broker URL auto-resolution from `node.toml` — cos's `cosmix-lib-config` (`client_helpers` feature).
- Anything that needs a `cosmix-noded` binary at build time (it's runtime-only; the protocol library compiles without one).

If a contribution would force a dep on cos, mix, or any sibling repo outside this workspace, it's in the wrong repo.

## Versioning

Each crate carries its own `version` in its `Cargo.toml`. Path-dep consumers (mix, cos) follow whatever's on `main`; version bumps become load-bearing once the crates publish to crates.io.

## License

MIT.
