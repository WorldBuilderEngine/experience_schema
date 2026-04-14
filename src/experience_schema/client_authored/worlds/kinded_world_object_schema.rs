use crate::client_authored::worlds::hotspot_object_schemas::{
    HotspotMarkerSpriteObjectSchema, InteractableHotspotObjectSchema, TransitionHotspotObjectSchema,
};
use crate::client_authored::worlds::typed_object_schemas::{
    CameraObjectSchema, StaticSpriteObjectSchema, StaticTextObjectSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum KindedWorldObjectSchema {
    Camera(CameraObjectSchema),
    StaticSprite(StaticSpriteObjectSchema),
    StaticText(StaticTextObjectSchema),
    TransitionHotspot(TransitionHotspotObjectSchema),
    InteractableHotspot(InteractableHotspotObjectSchema),
    HotspotMarkerSprite(HotspotMarkerSpriteObjectSchema),
}

impl KindedWorldObjectSchema {
    pub fn canonical_object_type_name(&self) -> &str {
        match self {
            Self::Camera(_) => "camera",
            Self::StaticSprite(_) => "static_sprite",
            Self::StaticText(_) => "static_text",
            Self::TransitionHotspot(transition_hotspot) => transition_hotspot.object_type.as_str(),
            Self::InteractableHotspot(interactable_hotspot) => {
                interactable_hotspot.object_type.as_str()
            }
            Self::HotspotMarkerSprite(_) => "static_sprite",
        }
    }
}
