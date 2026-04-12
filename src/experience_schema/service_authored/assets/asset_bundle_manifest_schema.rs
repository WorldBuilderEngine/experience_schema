use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trusted runtime-facing asset facts derived by backend publish/build flows.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestSchema {
    /// Bundle identifiers keyed to trusted runtime metadata entries.
    #[serde(default)]
    #[prost(map = "string, message", tag = "1")]
    pub bundles: HashMap<String, AssetBundleManifestEntrySchema>,
}

/// Trusted runtime metadata for one asset bundle identifier.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestEntrySchema {
    /// Optional runtime-visible bundle content hash derived during publish/build flows.
    #[serde(default)]
    #[prost(string, tag = "1")]
    pub content_hash: String,

    /// Trusted per-asset runtime facts derived from bundle bytes.
    ///
    /// Keep this limited to facts stripped runtimes actually consume after payload stripping.
    #[serde(default)]
    #[prost(map = "string, message", tag = "2")]
    pub assets: HashMap<String, AssetBundleManifestAssetMetadataSchema>,
}

/// Trusted intrinsic asset metadata for a single runtime-visible asset.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestAssetMetadataSchema {
    /// Trusted intrinsic asset width in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "1")]
    pub width_px: u32,

    /// Trusted intrinsic asset height in pixels for runtime sizing.
    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub height_px: u32,

    /// Trusted static-text font metrics derived during publish/build flows so runtimes can
    /// measure authored strings without loading raw font bytes.
    #[serde(default)]
    #[prost(message, optional, tag = "3")]
    pub static_text_font_metrics: Option<AssetBundleManifestStaticTextFontMetricsSchema>,
}

impl AssetBundleManifestAssetMetadataSchema {
    pub fn has_intrinsic_dimensions(&self) -> bool {
        self.width_px > 0 && self.height_px > 0
    }

    pub fn has_static_text_font_metrics(&self) -> bool {
        self.static_text_font_metrics
            .as_ref()
            .is_some_and(AssetBundleManifestStaticTextFontMetricsSchema::is_bound)
    }
}

/// Trusted static-text font metrics derived from a font asset for the glyphs an experience uses.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestStaticTextFontMetricsSchema {
    /// Canonical glyph scale used when deriving these metrics.
    #[serde(default)]
    #[prost(float, tag = "1")]
    pub glyph_scale_px: f32,

    /// Padding applied around measured text bounds.
    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub padding_px: u32,

    /// Trusted glyph metrics keyed by Unicode scalar value.
    #[serde(default)]
    #[prost(map = "uint32, message", tag = "3")]
    pub glyphs: HashMap<u32, AssetBundleManifestStaticTextGlyphMetricsSchema>,

    /// Exact authored-string layout bounds derived at publish/build time for strings that this
    /// experience uses with the font.
    #[serde(default)]
    #[prost(map = "string, message", tag = "4")]
    pub authored_text_layouts: HashMap<String, AssetBundleManifestStaticTextLayoutSchema>,
}

impl AssetBundleManifestStaticTextFontMetricsSchema {
    pub fn is_bound(&self) -> bool {
        self.glyph_scale_px > 0.0 && (!self.glyphs.is_empty() || !self.authored_text_layouts.is_empty())
    }
}

/// Trusted per-glyph metrics sufficient to reconstruct current stripped-runtime text bounds.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct AssetBundleManifestStaticTextGlyphMetricsSchema {
    /// Horizontal advance in pixels at the canonical glyph scale.
    #[serde(default)]
    #[prost(float, tag = "1")]
    pub advance_px: f32,

    /// Outline minimum x bound in pixels at pen_x = 0.
    #[serde(default)]
    #[prost(float, tag = "2")]
    pub min_x_px: f32,

    /// Outline minimum y bound in pixels at the canonical baseline.
    #[serde(default)]
    #[prost(float, tag = "3")]
    pub min_y_px: f32,

    /// Outline maximum x bound in pixels at pen_x = 0.
    #[serde(default)]
    #[prost(float, tag = "4")]
    pub max_x_px: f32,

    /// Outline maximum y bound in pixels at the canonical baseline.
    #[serde(default)]
    #[prost(float, tag = "5")]
    pub max_y_px: f32,

    /// Whether the glyph produced an outline during measurement.
    #[serde(default)]
    #[prost(bool, tag = "6")]
    pub has_outline: bool,
}

/// Trusted exact text layout bounds for an authored string measured from the source font asset.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Message)]
pub struct AssetBundleManifestStaticTextLayoutSchema {
    #[serde(default)]
    #[prost(uint32, tag = "1")]
    pub width_px: u32,

    #[serde(default)]
    #[prost(uint32, tag = "2")]
    pub height_px: u32,
}
