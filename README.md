# experience_schema

Shared `ExperienceSchema` contracts for WorldBuilder clients and backend services.

## Scope

- Defines serializable schema primitives consumed by engine and service crates.
- Provides a single package source-of-truth for cross-repository schema reuse.

## Anonymous 2D Browse/Play v1 Contract

`experience_schema` now defines the canonical v1 anonymous browse/play response contracts used by:
- `backend-data-center`
- `backend-gateway`
- `client`

Rust module:
- `experience_schema::shared::anonymous_browse_play_v1`

Primary contract constant:
- `ANONYMOUS_BROWSE_PLAY_V1` (`experience_schema.anonymous_browse_play.v1`)

Detailed required-field tables and canonical JSON payloads:
- In the wiki, `anonymous_browse_play_v1.md`
- `examples/anonymous_browse_play_v1/`

## Release Policy

- Semver is used with pre-1.0 constraints (`0.x`).
- Any schema shape or field removal that can break serialization compatibility requires a breaking version bump.
- Every release should pass:
  - `cargo test`
  - `cargo package`

## Publishing

Dry run:

```powershell
cargo publish --dry-run
```

Publish to crates.io:

```powershell
cargo publish
```
