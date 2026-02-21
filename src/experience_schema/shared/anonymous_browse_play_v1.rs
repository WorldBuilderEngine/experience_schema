use crate::{
    experience_schema::ExperienceSchema,
    service_authored::meta_data::publisher_info_schema::PublisherInfoSchema,
};
use serde::{Deserialize, Serialize};

pub const ANONYMOUS_BROWSE_PLAY_V1: &str = "experience_schema.anonymous_browse_play.v1";

/// Canonical anonymous catalog/listing response contract.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AnonymousCatalogResponseV1 {
    pub contract_version: String,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub experiences: Vec<AnonymousCatalogExperienceV1>,
}

/// Canonical anonymous detail response contract.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AnonymousExperienceDetailResponseV1 {
    pub contract_version: String,
    pub experience: AnonymousCatalogExperienceV1,
}

/// Canonical anonymous schema fetch response contract.
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct AnonymousSchemaFetchResponseV1 {
    pub contract_version: String,
    pub publisher_info: PublisherInfoSchema,
    pub experience_schema: ExperienceSchema,
}

/// Canonical anonymous play bootstrap response contract.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AnonymousPlayBootstrapResponseV1 {
    pub contract_version: String,
    pub experience_id: String,
    pub publish_id: String,
    pub publish_version: usize,
    #[serde(default)]
    pub entry_world_index: usize,
}

/// Canonical experience shape shared by catalog/detail responses.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AnonymousCatalogExperienceV1 {
    pub experience_id: String,
    pub publish_id: String,
    pub publish_version: usize,
    pub published_at_unix_seconds: u64,
    pub display_name: String,
    pub short_description: String,
    #[serde(default)]
    pub long_description: Option<String>,
    #[serde(default)]
    pub tile_color_hex: Option<String>,
    #[serde(default)]
    pub tile_icon_image: Option<String>,
    #[serde(default)]
    pub screenshot_gallery_image_urls: Vec<String>,
    #[serde(default)]
    pub genre: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub is_featured: bool,
}

#[cfg(test)]
mod tests {
    use super::{
        ANONYMOUS_BROWSE_PLAY_V1, AnonymousCatalogExperienceV1, AnonymousCatalogResponseV1,
    };

    #[test]
    fn catalog_response_defaults_to_empty_experience_list() {
        let response = AnonymousCatalogResponseV1 {
            contract_version: ANONYMOUS_BROWSE_PLAY_V1.to_string(),
            next_cursor: None,
            experiences: Vec::new(),
        };

        assert_eq!(response.contract_version, ANONYMOUS_BROWSE_PLAY_V1);
        assert!(response.experiences.is_empty());
    }

    #[test]
    fn catalog_experience_keeps_required_browse_fields() {
        let experience = AnonymousCatalogExperienceV1 {
            experience_id: "exp_demo".to_string(),
            publish_id: "pub_demo".to_string(),
            publish_version: 1,
            published_at_unix_seconds: 1_770_000_000,
            display_name: "Dungeon Crawler".to_string(),
            short_description: "Fight through a compact dungeon.".to_string(),
            long_description: None,
            tile_color_hex: Some("#3a5f8f".to_string()),
            tile_icon_image: Some("assets://tiles/dungeon.png".to_string()),
            screenshot_gallery_image_urls: vec!["https://cdn.example/dungeon-1.png".to_string()],
            genre: Some("adventure".to_string()),
            tags: vec!["single-player".to_string(), "2d".to_string()],
            is_featured: true,
        };

        assert_eq!(experience.display_name, "Dungeon Crawler");
        assert_eq!(experience.short_description, "Fight through a compact dungeon.");
        assert_eq!(experience.publish_version, 1);
    }
}
