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

## Published Experience Contract Fixtures

Canonical fixture package for publish-to-runtime conformance:
- `fixtures/published_experience_contract/v1/fixture_manifest.json`
- `fixtures/published_experience_contract/v1/publish_payload_template.json`
- `fixtures/published_experience_contract/v1/client_authored_schema.json`

The fixture manifest defines:
- Contract version and schema version.
- Payload conformance inputs consumed by `publishing_tools` and `backend-experience-publishing`.
- Runtime dedicated-route smoke target consumed by `world_builder` CI gates.

When contract-affecting schema behavior changes, update this fixture package in the same change.

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
