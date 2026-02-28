<!--
source_wiki_id: 8f0e24b3-702d-42d7-bdee-c10c5b50fe17
source_system: governance-wiki-manager
domain_tag: experience
team_tag: schema
product_tag: experience_schema
source_status: published
source_created_at: 2026-02-27 05:08:55.269498700
source_updated_at: 2026-02-27 05:09:10.650860700
migrated_at_utc: 2026-02-28T18:44:04+00:00
-->
# Experience Schema Publish Contract

As of 2026-02-27, schema publish and browse metadata are explicitly decoupled.

## Contract
- `POST /v1/experiences` is for runtime schema only (`client_authored_schema` + runtime root fields).
- Browse metadata (tile image, tile color, screenshots, long description) is not part of schema publish payloads.
- Metadata must be managed through metadata-specific APIs/flows.

## Client Enforcement
- Client publish request builder strips `client_authored_schema.metadata` before submit.
- Client no longer injects a fake external CDN thumbnail URL into schema publish requests.
- Client ignores asset-ref style tile image values (`asset://...`, `asset:...`) and only treats direct image sources/thumbnail identifiers as tile image sources.
- Client no longer synthesizes a default tile color when service metadata omits or invalidates color.

## Why
- Experience schema should only encode runtime/simulation behavior and data.
- Browse/discovery presentation metadata is a separate system with independent lifecycle and validation.

## Regression Guidance
- Add/keep tests around publish request payload extraction so `metadata` is absent from schema submit payloads.
- Reject new code paths that couple schema serialization with tile/screenshot metadata defaults.
