use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::hotspot_object_schemas::{
    HotspotMarkerSpriteObjectSchema, InteractableHotspotObjectSchema, TransitionHotspotObjectSchema,
};
use crate::client_authored::worlds::kinded_world_object_schema::KindedWorldObjectSchema;
use crate::client_authored::worlds::typed_object_schemas::{
    CameraObjectSchema, StaticSpriteObjectSchema, StaticTextObjectSchema,
    UiRectPrimitiveObjectSchema,
};
use crate::properties::property_map::PropertyMap;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType};
use prost::{DecodeError, Message};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct WorldObjectSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinded: Option<KindedWorldObjectSchema>,

    /// The custom properties (data) for this object.
    #[serde(default, skip_serializing_if = "PropertyMap::is_empty")]
    pub properties: PropertyMap,

    /// State machines (code) for this object.
    #[serde(default)]
    pub state_machines: Vec<StateMachineSchema>,
}

impl WorldObjectSchema {
    pub fn from_custom_properties(properties: PropertyMap) -> Self {
        Self {
            kinded: None,
            properties,
            state_machines: Vec::new(),
        }
    }

    pub fn with_state_machines(mut self, state_machines: Vec<StateMachineSchema>) -> Self {
        self.state_machines = state_machines;
        self
    }

    pub fn camera(camera: CameraObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::Camera(camera)),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn static_sprite(static_sprite: StaticSpriteObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::StaticSprite(static_sprite)),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn static_text(static_text: StaticTextObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::StaticText(static_text)),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn ui_rect(ui_rect: UiRectPrimitiveObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::UiRect(ui_rect)),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn transition_hotspot(transition_hotspot: TransitionHotspotObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::TransitionHotspot(
                transition_hotspot,
            )),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn interactable_hotspot(interactable_hotspot: InteractableHotspotObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::InteractableHotspot(
                interactable_hotspot,
            )),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }

    pub fn hotspot_marker_sprite(hotspot_marker: HotspotMarkerSpriteObjectSchema) -> Self {
        Self {
            kinded: Some(KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker)),
            properties: PropertyMap::new(),
            state_machines: Vec::new(),
        }
    }
}

impl Message for WorldObjectSchema {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        WorldObjectSchemaBinaryWire::from(self.clone()).encode_raw(buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError> {
        let mut wire = WorldObjectSchemaBinaryWire::from(self.clone());
        wire.merge_field(tag, wire_type, buf, ctx)?;
        *self = wire.into_schema()?;
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        WorldObjectSchemaBinaryWire::from(self.clone()).encoded_len()
    }

    fn clear(&mut self) {
        *self = Self::default();
    }
}

#[allow(deprecated)]
fn world_object_schema_json_decode_error(error: serde_json::Error) -> DecodeError {
    DecodeError::new(format!("schema JSON message decode failed: {error}"))
}

#[derive(Clone, PartialEq, Message)]
struct WorldObjectSchemaBinaryWire {
    #[prost(bytes, optional, tag = "16")]
    kinded_json: Option<Vec<u8>>,
    #[prost(message, optional, tag = "17")]
    properties: Option<PropertyMap>,
    #[prost(message, repeated, tag = "18")]
    state_machines: Vec<StateMachineSchema>,
}

impl From<WorldObjectSchema> for WorldObjectSchemaBinaryWire {
    fn from(value: WorldObjectSchema) -> Self {
        Self {
            kinded_json: value.kinded.map(|kinded| {
                serde_json::to_vec(&kinded).expect("kinded world object should serialize")
            }),
            properties: Some(value.properties),
            state_machines: value.state_machines,
        }
    }
}

impl WorldObjectSchemaBinaryWire {
    fn into_schema(self) -> Result<WorldObjectSchema, DecodeError> {
        let kinded = match self.kinded_json {
            Some(bytes) => Some(
                serde_json::from_slice(&bytes).map_err(world_object_schema_json_decode_error)?,
            ),
            None => None,
        };
        Ok(WorldObjectSchema {
            kinded,
            properties: self.properties.unwrap_or_default(),
            state_machines: self.state_machines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::WorldObjectSchema;
    use crate::assets::asset_ref::AssetRef;
    use crate::client_authored::worlds::hotspot_object_schemas::{
        HotspotBoundsPx, InteractableHotspotObjectSchema,
    };
    use crate::client_authored::worlds::kinded_world_object_schema::KindedWorldObjectSchema;
    use crate::client_authored::worlds::typed_object_schemas::{
        CameraObjectSchema, CameraProjectionSchema, StaticSpriteObjectSchema,
        UiRectPrimitiveObjectSchema, UiRectSpecSchema, UiRectStyleSchema,
    };
    use crate::client_authored::worlds::world_object_view::AuthoredWorldObjectView;
    use prost::Message;
    use std::path::PathBuf;

    #[test]
    fn camera_constructor_emits_kinded_payload_without_mirrored_property_bag() {
        let world_object = WorldObjectSchema::camera(CameraObjectSchema {
            position_xyz: [1.0, 2.0, 3.0],
            projection: CameraProjectionSchema::Orthographic2d {
                pixels_per_unit: 96.0,
            },
            is_active_camera: true,
            debug_movement_units_per_second: 240.0,
            named_handle: Some(" camera:main ".to_string()),
            follow_target_named_handle: None,
            follow_target_distance_xyz: Some([4.0, 5.0, 6.0]),
            follow_units_per_second_xyz: Some([7.0, 8.0, 9.0]),
            arm_distance: Some(10.0),
            follow_scroll_type: None,
        });
        let world_object_view = AuthoredWorldObjectView::new(&world_object);

        assert!(world_object.properties.is_empty());
        assert_eq!(world_object_view.object_type(), Some("camera"));
        assert_eq!(
            world_object_view.named_handle().as_deref(),
            Some("camera:main")
        );
        assert_eq!(
            world_object_view.float_array("position"),
            Some(vec![1.0, 2.0, 3.0])
        );
        assert_eq!(world_object_view.float("pixels_per_unit"), Some(96.0));
        assert!(matches!(
            world_object.kinded,
            Some(KindedWorldObjectSchema::Camera(_))
        ));
    }

    #[test]
    fn static_sprite_constructor_emits_kinded_payload_without_mirrored_property_bag() {
        let mut static_sprite = StaticSpriteObjectSchema::new(
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")),
            [0.0, 0.0, 0.0],
            [2.0, 3.0],
        );
        static_sprite.intrinsic_width_px = Some(64);
        static_sprite.intrinsic_height_px = Some(32);
        static_sprite.interaction_enabled = true;

        let world_object = WorldObjectSchema::static_sprite(static_sprite);
        let world_object_view = AuthoredWorldObjectView::new(&world_object);

        assert!(world_object.properties.is_empty());
        assert_eq!(world_object_view.object_type(), Some("static_sprite"));
        assert_eq!(world_object_view.int("intrinsic_width_px"), Some(64));
        assert_eq!(world_object_view.int("intrinsic_height_px"), Some(32));
        assert_eq!(world_object_view.bool("interaction_enabled"), Some(true));
        assert!(matches!(
            world_object.kinded,
            Some(KindedWorldObjectSchema::StaticSprite(_))
        ));
    }

    #[test]
    fn ui_rect_constructor_emits_kinded_payload_without_mirrored_property_bag() {
        let world_object = WorldObjectSchema::ui_rect(UiRectPrimitiveObjectSchema::new(
            UiRectSpecSchema::default(),
            UiRectStyleSchema::filled([1.0, 0.0, 0.0, 1.0]),
        ));
        let world_object_view = AuthoredWorldObjectView::new(&world_object);

        assert!(world_object.properties.is_empty());
        assert_eq!(world_object_view.object_type(), Some("ui_rect"));
        assert!(matches!(
            world_object.kinded,
            Some(KindedWorldObjectSchema::UiRect(_))
        ));
    }

    #[test]
    fn interactable_hotspot_constructor_emits_kinded_payload_without_mirrored_property_bag() {
        let world_object =
            WorldObjectSchema::interactable_hotspot(InteractableHotspotObjectSchema {
                object_type: "sample_hotspot_interaction".to_string(),
                scene_id: "courtyard".to_string(),
                hotspot_id: "string_totem".to_string(),
                target_id: "memory_totem".to_string(),
                bounds_px: HotspotBoundsPx::new(640, 220, 280, 460),
                verb_id: Some("inspect".to_string()),
                item_id: None,
                required_item_id: None,
                consumes_required_item: None,
                activation_event: Some("activate".to_string()),
                interaction_resolved_event: Some("resolved".to_string()),
                inventory_collected_event: None,
                gate_blocked_event: None,
                gate_unlocked_event: None,
                default_asset_ref: Some(AssetRef::new_with_bundle_id(
                    "embedded",
                    PathBuf::from("default.png"),
                )),
                hover_asset_ref: Some(AssetRef::new_with_bundle_id(
                    "embedded",
                    PathBuf::from("hover.png"),
                )),
                hover_entered_event: Some("hovered".to_string()),
                hover_exited_event: Some("idle".to_string()),
                pressed_event: Some("pressed".to_string()),
            });
        let world_object_view = AuthoredWorldObjectView::new(&world_object);

        assert!(world_object.properties.is_empty());
        assert_eq!(
            world_object_view.object_type(),
            Some("sample_hotspot_interaction")
        );
        assert_eq!(
            world_object_view.string("target_id").map(String::as_str),
            Some("memory_totem")
        );
        assert_eq!(
            world_object_view.float_array("bounds_px"),
            Some(vec![640.0, 220.0, 280.0, 460.0])
        );
        assert_eq!(
            world_object_view
                .asset_ref("default_asset_ref")
                .and_then(|asset_ref| asset_ref.get_bundle_id()),
            Some("embedded")
        );
        assert!(matches!(
            world_object.kinded,
            Some(KindedWorldObjectSchema::InteractableHotspot(_))
        ));
    }

    #[test]
    fn prost_round_trips_world_object_schema_as_binary_message() {
        let world_object = WorldObjectSchema::camera(CameraObjectSchema {
            position_xyz: [1.0, 2.0, 3.0],
            projection: CameraProjectionSchema::Orthographic2d {
                pixels_per_unit: 96.0,
            },
            is_active_camera: false,
            debug_movement_units_per_second: 0.0,
            named_handle: Some("camera:main".to_string()),
            follow_target_named_handle: None,
            follow_target_distance_xyz: None,
            follow_units_per_second_xyz: None,
            arm_distance: None,
            follow_scroll_type: None,
        });

        let encoded = world_object.encode_to_vec();
        let decoded =
            WorldObjectSchema::decode(encoded.as_slice()).expect("world object should decode");

        assert_eq!(decoded, world_object);
    }
}
