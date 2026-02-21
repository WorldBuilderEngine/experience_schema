use serde::{Deserialize, Serialize};

/// Describes client-authored presentation metadata for browse surfaces.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct ClientPublishedMetadataSchema {
    #[serde(default, alias = "title", alias = "name")]
    pub display_name: Option<String>,
    #[serde(default, alias = "summary")]
    pub short_description: Option<String>,
    #[serde(default)]
    pub tile_color_hex: Option<String>,
    #[serde(default, alias = "tile_icon_asset_ref", alias = "tile_image_asset_ref")]
    pub tile_icon_image: Option<String>,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default, alias = "screenshot_gallery", alias = "screenshot_image_urls")]
    pub screenshot_gallery_image_urls: Vec<String>,
    #[serde(default)]
    pub genre: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default, alias = "featured")]
    pub is_featured: bool,
}

impl ClientPublishedMetadataSchema {}
