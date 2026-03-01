use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn normalize_bundle_identifier(bundle_identifier: impl Into<String>) -> String {
    bundle_identifier.into().trim().to_string()
}

/// The high-level source type for an asset bundle.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum AssetBundleKind {
    /// Authored content loaded from loose files.
    Unpacked,
    /// Runtime-ready content loaded from packed bytes.
    #[default]
    Packed,
    /// Runtime-generated content (for tests or procedural content).
    Generated,
}

/// A single serialized asset entry.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoredAssetSchema {
    /// Bundle-relative asset path.
    pub asset_path: PathBuf,
    /// Serialized bytes for this asset.
    pub asset_data: Vec<u8>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl StoredAssetSchema {
    pub fn new(asset_path: PathBuf, asset_data: Vec<u8>) -> Self {
        Self {
            asset_path,
            asset_data,
            _extensions: HashMap::new(),
        }
    }
}

/// A serializable asset bundle.
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct AssetBundleSchema {
    /// Unique identifier for this bundle within an experience.
    pub bundle_id: String,
    /// Bundle source classification.
    pub bundle_kind: AssetBundleKind,
    /// Serialized assets for this bundle.
    pub assets: Vec<StoredAssetSchema>,
    // Future-proof reserved extension space to allow inserting new members above.
    #[serde(default, flatten)]
    pub _extensions: HashMap<String, serde_json::Value>,
}

impl AssetBundleSchema {
    pub fn new(bundle_id: impl Into<String>, bundle_kind: AssetBundleKind) -> Self {
        Self {
            bundle_id: normalize_bundle_identifier(bundle_id),
            bundle_kind,
            assets: Vec::new(),
            _extensions: HashMap::new(),
        }
    }

    pub fn upsert_asset(&mut self, asset_path: PathBuf, asset_data: Vec<u8>) {
        if let Some(asset_schema_index) = self
            .assets
            .iter()
            .position(|asset_schema| asset_schema.asset_path == asset_path)
        {
            self.assets[asset_schema_index].asset_data = asset_data;
        } else {
            self.assets
                .push(StoredAssetSchema::new(asset_path, asset_data));
        }
    }

    pub fn find_asset(&self, asset_path: &Path) -> Option<&StoredAssetSchema> {
        self.assets
            .iter()
            .find(|asset_schema| asset_schema.asset_path == asset_path)
    }
}

pub type AssetStoreKind = AssetBundleKind;
pub type AssetStoreSchema = AssetBundleSchema;
