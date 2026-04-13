use crate::client_authored::worlds::hotspot_object_schemas::{
    HotspotMarkerSpriteObjectSchema, InteractableHotspotObjectSchema, TransitionHotspotObjectSchema,
};
use crate::client_authored::state_machines::state_machine_schema::StateMachineSchema;
use crate::client_authored::worlds::typed_object_schemas::{
    CameraObjectSchema, CameraProjectionSchema, StaticSpriteObjectSchema, StaticTextObjectSchema,
};
use crate::properties::property_map::PropertyMap;
use prost::Message;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct WorldObjectSchema {
    /// The custom properties (data) for this object.
    #[prost(message, required, tag = "1")]
    pub properties: PropertyMap,

    /// State machines (code) for this object.
    #[prost(message, repeated, tag = "2")]
    pub state_machines: Vec<StateMachineSchema>,
}

impl WorldObjectSchema {
    pub fn from_custom_properties(properties: PropertyMap) -> Self {
        Self {
            properties,
            state_machines: Vec::new(),
        }
    }

    pub fn with_state_machines(
        mut self,
        state_machines: Vec<StateMachineSchema>,
    ) -> Self {
        self.state_machines = state_machines;
        self
    }

    pub fn camera(camera: CameraObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "camera");
        match camera.projection {
            CameraProjectionSchema::Orthographic2d { pixels_per_unit } => {
                properties.insert_string("camera_projection", "orthographic_2d");
                properties.insert_float("pixels_per_unit", pixels_per_unit);
            }
            CameraProjectionSchema::Perspective3d {
                focal_length,
                camera_forward_xyz,
                camera_up_xyz,
            } => {
                properties.insert_string("camera_projection", "perspective_3d");
                properties.insert_float("focal_length", focal_length);
                properties.insert_float_array("camera_forward", camera_forward_xyz.to_vec());
                properties.insert_float_array("camera_up", camera_up_xyz.to_vec());
            }
        }
        properties.insert_float_array("position", camera.position_xyz.to_vec());
        properties.insert_bool("is_active_camera", camera.is_active_camera);
        if camera.debug_movement_units_per_second.is_finite() {
            properties.insert_float(
                "debug_movement_units_per_second",
                camera.debug_movement_units_per_second,
            );
        }
        if let Some(node_tag) = normalize_optional_string(camera.node_tag) {
            properties.insert_string("node_tag", node_tag);
        }
        if let Some(follow_target_node_tag) = normalize_optional_string(camera.follow_target_node_tag)
        {
            properties.insert_string("follow_target_node_tag", follow_target_node_tag);
        }
        if let Some(follow_target_distance_xyz) = camera.follow_target_distance_xyz {
            properties.insert_float_array(
                "follow_target_distance",
                follow_target_distance_xyz.to_vec(),
            );
        }
        if let Some(follow_units_per_second_xyz) = camera.follow_units_per_second_xyz {
            properties.insert_float_array(
                "follow_units_per_second",
                follow_units_per_second_xyz.to_vec(),
            );
        }
        if let Some(arm_distance) = camera.arm_distance {
            properties.insert_float("arm_distance", arm_distance);
        }
        if let Some(follow_scroll_type) = normalize_optional_string(camera.follow_scroll_type) {
            properties.insert_string("follow_scroll_type", follow_scroll_type);
        }

        Self::from_custom_properties(properties)
    }

    pub fn static_sprite(static_sprite: StaticSpriteObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "static_sprite");
        properties.insert_asset_ref("asset_ref", static_sprite.asset_ref);
        properties.insert_float_array("position", static_sprite.position_xyz.to_vec());
        properties.insert_float_array("scale", static_sprite.scale_xy.to_vec());
        properties.insert_float_array(
            "rotation_deg",
            static_sprite.rotation_deg_xyz.to_vec(),
        );
        properties.insert_bool("is_visible", static_sprite.is_visible);
        if let Some(node_tag) = normalize_optional_string(static_sprite.node_tag) {
            properties.insert_string("node_tag", node_tag);
        }
        if let Some(parent_node_tag) = normalize_optional_string(static_sprite.parent_node_tag) {
            properties.insert_string("parent_node_tag", parent_node_tag);
        }
        if let Some(scene_id) = normalize_optional_string(static_sprite.scene_id) {
            properties.insert_string("scene_id", scene_id);
        }
        if let Some(visible_when_scene_active) = static_sprite.visible_when_scene_active {
            properties.insert_bool(
                "visible_when_scene_active",
                visible_when_scene_active,
            );
        }
        if let Some(tile_width_px) = static_sprite.tile_width_px {
            properties.insert_int("tile_width_px", i64::from(tile_width_px));
        }
        if let Some(tile_height_px) = static_sprite.tile_height_px {
            properties.insert_int("tile_height_px", i64::from(tile_height_px));
        }
        if let Some(intrinsic_width_px) = static_sprite.intrinsic_width_px {
            properties.insert_int("intrinsic_width_px", i64::from(intrinsic_width_px));
        }
        if let Some(intrinsic_height_px) = static_sprite.intrinsic_height_px {
            properties.insert_int("intrinsic_height_px", i64::from(intrinsic_height_px));
        }
        if let Some(repeat_x) = static_sprite.repeat_x {
            properties.insert_bool("repeat_x", repeat_x);
        }
        if let Some(repeat_y) = static_sprite.repeat_y {
            properties.insert_bool("repeat_y", repeat_y);
        }
        if static_sprite.interaction_enabled {
            properties.insert_bool("interaction_enabled", true);
        }

        Self::from_custom_properties(properties)
    }

    pub fn static_text(static_text: StaticTextObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "static_text");
        properties.insert_asset_ref("font_asset_ref", static_text.font_asset_ref);
        properties.insert_string("text", static_text.text);
        properties.insert_float_array("position", static_text.position_xyz.to_vec());
        properties.insert_float_array("scale", static_text.scale_xy.to_vec());
        properties.insert_float_array(
            "rotation_deg",
            static_text.rotation_deg_xyz.to_vec(),
        );
        properties.insert_bool("is_visible", static_text.is_visible);
        if let Some(node_tag) = normalize_optional_string(static_text.node_tag) {
            properties.insert_string("node_tag", node_tag);
        }
        if let Some(parent_node_tag) = normalize_optional_string(static_text.parent_node_tag) {
            properties.insert_string("parent_node_tag", parent_node_tag);
        }
        if let Some(scene_id) = normalize_optional_string(static_text.scene_id) {
            properties.insert_string("scene_id", scene_id);
        }
        if static_text.interaction_enabled {
            properties.insert_bool("interaction_enabled", true);
        }
        if let Some(outline_color_rgba) = static_text.outline_color_rgba {
            properties.insert_float_array("outline_color_rgba", outline_color_rgba.to_vec());
        }
        if let Some(outline_thickness_px) = static_text.outline_thickness_px {
            properties.insert_float("outline_thickness_px", outline_thickness_px);
        }

        Self::from_custom_properties(properties)
    }

    pub fn transition_hotspot(transition_hotspot: TransitionHotspotObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", transition_hotspot.object_type);
        properties.insert_string("hotspot_id", transition_hotspot.hotspot_id);
        properties.insert_string("from_scene_id", transition_hotspot.from_scene_id);
        properties.insert_string("to_scene_id", transition_hotspot.to_scene_id);
        if let Some(activation_event) = transition_hotspot.activation_event {
            properties.insert_string("activation_event", activation_event);
        }
        if let Some(transition_started_event) = transition_hotspot.transition_started_event {
            properties.insert_string("transition_started_event", transition_started_event);
        }
        if let Some(transition_completed_event) = transition_hotspot.transition_completed_event {
            properties.insert_string("transition_completed_event", transition_completed_event);
        }
        properties.insert_float_array("bounds_px", transition_hotspot.bounds_px.as_vec());

        Self::from_custom_properties(properties)
    }

    pub fn interactable_hotspot(interactable_hotspot: InteractableHotspotObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", interactable_hotspot.object_type);
        properties.insert_string("scene_id", interactable_hotspot.scene_id);
        properties.insert_string("hotspot_id", interactable_hotspot.hotspot_id);
        properties.insert_string("target_id", interactable_hotspot.target_id);
        if let Some(verb_id) = interactable_hotspot.verb_id {
            properties.insert_string("verb_id", verb_id);
        }
        if let Some(item_id) = interactable_hotspot.item_id {
            properties.insert_string("item_id", item_id);
        }
        if let Some(required_item_id) = interactable_hotspot.required_item_id {
            properties.insert_string("required_item_id", required_item_id);
        }
        if let Some(consumes_required_item) = interactable_hotspot.consumes_required_item {
            properties.insert_bool("consumes_required_item", consumes_required_item);
        }
        if let Some(activation_event) = interactable_hotspot.activation_event {
            properties.insert_string("activation_event", activation_event);
        }
        if let Some(interaction_resolved_event) = interactable_hotspot.interaction_resolved_event {
            properties.insert_string("interaction_resolved_event", interaction_resolved_event);
        }
        if let Some(inventory_collected_event) = interactable_hotspot.inventory_collected_event {
            properties.insert_string("inventory_collected_event", inventory_collected_event);
        }
        if let Some(gate_blocked_event) = interactable_hotspot.gate_blocked_event {
            properties.insert_string("gate_blocked_event", gate_blocked_event);
        }
        if let Some(gate_unlocked_event) = interactable_hotspot.gate_unlocked_event {
            properties.insert_string("gate_unlocked_event", gate_unlocked_event);
        }
        properties.insert_float_array("bounds_px", interactable_hotspot.bounds_px.as_vec());
        if let Some(default_asset_ref) = interactable_hotspot.default_asset_ref {
            properties.insert_asset_ref("default_asset_ref", default_asset_ref);
        }
        if let Some(hover_asset_ref) = interactable_hotspot.hover_asset_ref {
            properties.insert_asset_ref("hover_asset_ref", hover_asset_ref);
        }
        if let Some(hover_entered_event) = interactable_hotspot.hover_entered_event {
            properties.insert_string("hover_entered_event", hover_entered_event);
        }
        if let Some(hover_exited_event) = interactable_hotspot.hover_exited_event {
            properties.insert_string("hover_exited_event", hover_exited_event);
        }
        if let Some(pressed_event) = interactable_hotspot.pressed_event {
            properties.insert_string("pressed_event", pressed_event);
        }

        Self::from_custom_properties(properties)
    }

    pub fn hotspot_marker_sprite(hotspot_marker: HotspotMarkerSpriteObjectSchema) -> Self {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "static_sprite");
        properties.insert_asset_ref("asset_ref", hotspot_marker.asset_ref);
        properties.insert_float_array("position", hotspot_marker.position_xyz.to_vec());
        properties.insert_float_array("scale", hotspot_marker.scale_xy.to_vec());
        properties.insert_float_array("rotation_deg", hotspot_marker.rotation_deg_xyz.to_vec());
        properties.insert_bool("is_visible", hotspot_marker.is_visible);
        properties.insert_string("scene_id", hotspot_marker.scene_id);
        if let Some(node_tag) = normalize_optional_string(hotspot_marker.node_tag) {
            properties.insert_string("node_tag", node_tag);
        }
        properties.insert_string("marker_kind", hotspot_marker.marker_kind);
        properties.insert_string("hotspot_id", hotspot_marker.hotspot_id);
        if hotspot_marker.interaction_enabled {
            properties.insert_bool("interaction_enabled", true);
        }

        Self::from_custom_properties(properties)
    }
}

fn normalize_optional_string(raw_value: Option<String>) -> Option<String> {
    let normalized_value = raw_value?.trim().to_string();
    if normalized_value.is_empty() {
        return None;
    }

    Some(normalized_value)
}

#[cfg(test)]
mod tests {
    use super::WorldObjectSchema;
    use crate::assets::asset_ref::AssetRef;
    use crate::client_authored::worlds::hotspot_object_schemas::{
        HotspotBoundsPx, InteractableHotspotObjectSchema,
    };
    use crate::client_authored::worlds::typed_object_schemas::{
        CameraObjectSchema, CameraProjectionSchema, StaticSpriteObjectSchema,
    };
    use std::path::PathBuf;

    #[test]
    fn camera_constructor_emits_canonical_property_bag() {
        let world_object = WorldObjectSchema::camera(CameraObjectSchema {
            position_xyz: [1.0, 2.0, 3.0],
            projection: CameraProjectionSchema::Orthographic2d {
                pixels_per_unit: 96.0,
            },
            is_active_camera: true,
            debug_movement_units_per_second: 240.0,
            node_tag: Some(" camera:main ".to_string()),
            follow_target_node_tag: None,
            follow_target_distance_xyz: Some([4.0, 5.0, 6.0]),
            follow_units_per_second_xyz: Some([7.0, 8.0, 9.0]),
            arm_distance: Some(10.0),
            follow_scroll_type: None,
        });

        assert_eq!(
            world_object.properties.get_string("object_type").map(String::as_str),
            Some("camera")
        );
        assert_eq!(
            world_object.properties.get_string("node_tag").map(String::as_str),
            Some("camera:main")
        );
        assert_eq!(
            world_object.properties.get_float_array("position"),
            Some(&vec![1.0, 2.0, 3.0])
        );
        assert_eq!(
            world_object.properties.get_float("pixels_per_unit"),
            Some(96.0)
        );
    }

    #[test]
    fn static_sprite_constructor_emits_intrinsic_dimensions_when_present() {
        let mut static_sprite = StaticSpriteObjectSchema::new(
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")),
            [0.0, 0.0, 0.0],
            [2.0, 3.0],
        );
        static_sprite.intrinsic_width_px = Some(64);
        static_sprite.intrinsic_height_px = Some(32);
        static_sprite.interaction_enabled = true;

        let world_object = WorldObjectSchema::static_sprite(static_sprite);

        assert_eq!(
            world_object.properties.get_string("object_type").map(String::as_str),
            Some("static_sprite")
        );
        assert_eq!(world_object.properties.get_int("intrinsic_width_px"), Some(64));
        assert_eq!(world_object.properties.get_int("intrinsic_height_px"), Some(32));
        assert_eq!(
            world_object.properties.get_bool("interaction_enabled"),
            Some(true)
        );
    }

    #[test]
    fn interactable_hotspot_constructor_emits_canonical_property_bag() {
        let world_object = WorldObjectSchema::interactable_hotspot(InteractableHotspotObjectSchema {
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

        assert_eq!(
            world_object.properties.get_string("object_type").map(String::as_str),
            Some("sample_hotspot_interaction")
        );
        assert_eq!(
            world_object.properties.get_string("target_id").map(String::as_str),
            Some("memory_totem")
        );
        assert_eq!(
            world_object.properties.get_float_array("bounds_px"),
            Some(&vec![640.0, 220.0, 280.0, 460.0])
        );
        assert_eq!(
            world_object
                .properties
                .get_asset_ref("default_asset_ref")
                .and_then(|asset_ref| asset_ref.get_bundle_id()),
            Some("embedded")
        );
    }
}
