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
}
