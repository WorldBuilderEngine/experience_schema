# Anonymous Browse/Play Contract v1

Contract ID: `experience_schema.anonymous_browse_play.v1`

Purpose:
- Define one versioned response shape for anonymous 2D single-player browse-to-play.
- Keep data-center, gateway, and client field requirements aligned.

## Catalog Response

Type: `AnonymousCatalogResponseV1`

Required top-level fields:
- `contract_version: string`
- `experiences: AnonymousCatalogExperienceV1[]`

Optional top-level fields:
- `next_cursor: string | null`

## Detail Response

Type: `AnonymousExperienceDetailResponseV1`

Required top-level fields:
- `contract_version: string`
- `experience: AnonymousCatalogExperienceV1`

## Shared Experience Fields (Catalog + Detail)

Type: `AnonymousCatalogExperienceV1`

Required:
- `experience_id: string`
- `publish_id: string`
- `publish_version: number`
- `published_at_unix_seconds: number`
- `display_name: string`
- `short_description: string`

Optional:
- `long_description: string | null`
- `tile_color_hex: string | null`
- `tile_icon_image: string | null`
- `screenshot_gallery_image_urls: string[]`
- `genre: string | null`
- `tags: string[]`
- `is_featured: boolean`

## Schema Fetch Response

Type: `AnonymousSchemaFetchResponseV1`

Required:
- `contract_version: string`
- `publisher_info: PublisherInfoSchema`
- `experience_schema: ExperienceSchema`

## Play Bootstrap Response

Type: `AnonymousPlayBootstrapResponseV1`

Required:
- `contract_version: string`
- `experience_id: string`
- `publish_id: string`
- `publish_version: number`
- `entry_world_index: number`

## Canonical Payload Examples

See:
- `examples/anonymous_browse_play_v1/catalog_response.json`
- `examples/anonymous_browse_play_v1/detail_response.json`
- `examples/anonymous_browse_play_v1/schema_fetch_response.json`
- `examples/anonymous_browse_play_v1/play_bootstrap_response.json`
