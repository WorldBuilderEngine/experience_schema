use crate::assets::asset_ref::AssetRef;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CameraProjectionSchema {
    Orthographic2d {
        pixels_per_unit: f64,
    },
    Perspective3d {
        focal_length: f64,
        camera_forward_xyz: [f64; 3],
        camera_up_xyz: [f64; 3],
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraObjectSchema {
    pub position_xyz: [f64; 3],
    pub projection: CameraProjectionSchema,
    pub is_active_camera: bool,
    pub debug_movement_units_per_second: f64,
    #[serde(default, alias = "node_tag", skip_serializing_if = "Option::is_none")]
    pub named_handle: Option<String>,
    #[serde(
        default,
        alias = "follow_target_node_tag",
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_target_named_handle: Option<String>,
    pub follow_target_distance_xyz: Option<[f64; 3]>,
    pub follow_units_per_second_xyz: Option<[f64; 3]>,
    pub arm_distance: Option<f64>,
    pub follow_scroll_type: Option<String>,
}

impl CameraObjectSchema {
    pub fn orthographic_2d(position_xyz: [f64; 3], pixels_per_unit: f64) -> Self {
        Self {
            position_xyz,
            projection: CameraProjectionSchema::Orthographic2d { pixels_per_unit },
            is_active_camera: false,
            debug_movement_units_per_second: 0.0,
            named_handle: None,
            follow_target_named_handle: None,
            follow_target_distance_xyz: None,
            follow_units_per_second_xyz: None,
            arm_distance: None,
            follow_scroll_type: None,
        }
    }

    pub fn perspective_3d(position_xyz: [f64; 3], focal_length: f64) -> Self {
        Self {
            position_xyz,
            projection: CameraProjectionSchema::Perspective3d {
                focal_length,
                camera_forward_xyz: [0.0, 1.0, 0.0],
                camera_up_xyz: [0.0, 0.0, 1.0],
            },
            is_active_camera: false,
            debug_movement_units_per_second: 0.0,
            named_handle: None,
            follow_target_named_handle: None,
            follow_target_distance_xyz: None,
            follow_units_per_second_xyz: None,
            arm_distance: None,
            follow_scroll_type: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaticSpritePresentationSpaceSchema {
    World,
    PresentedViewport,
    GameplayFrame,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaticSpritePresentationSizingModeSchema {
    Authored,
    Fit,
    Cover,
    Stretch,
    Tile,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticSpriteObjectSchema {
    pub asset_ref: AssetRef,
    pub position_xyz: [f64; 3],
    pub scale_xy: [f64; 2],
    pub rotation_deg_xyz: [f64; 3],
    pub is_visible: bool,
    #[serde(default, alias = "node_tag", skip_serializing_if = "Option::is_none")]
    pub named_handle: Option<String>,
    #[serde(
        default,
        alias = "parent_node_tag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_named_handle: Option<String>,
    pub scene_id: Option<String>,
    pub visible_when_scene_active: Option<bool>,
    pub tile_width_px: Option<u32>,
    pub tile_height_px: Option<u32>,
    pub intrinsic_width_px: Option<u32>,
    pub intrinsic_height_px: Option<u32>,
    pub repeat_x: Option<bool>,
    pub repeat_y: Option<bool>,
    pub presentation_space: Option<StaticSpritePresentationSpaceSchema>,
    pub presentation_sizing_mode: Option<StaticSpritePresentationSizingModeSchema>,
    pub anchor_normalized_xy: Option<[f64; 2]>,
    pub pivot_normalized_xy: Option<[f64; 2]>,
    pub margin_px: Option<[f64; 2]>,
    pub interaction_enabled: bool,
}

impl StaticSpriteObjectSchema {
    pub fn new(asset_ref: AssetRef, position_xyz: [f64; 3], scale_xy: [f64; 2]) -> Self {
        Self {
            asset_ref,
            position_xyz,
            scale_xy,
            rotation_deg_xyz: [0.0, 0.0, 0.0],
            is_visible: true,
            named_handle: None,
            parent_named_handle: None,
            scene_id: None,
            visible_when_scene_active: None,
            tile_width_px: None,
            tile_height_px: None,
            intrinsic_width_px: None,
            intrinsic_height_px: None,
            repeat_x: None,
            repeat_y: None,
            presentation_space: None,
            presentation_sizing_mode: None,
            anchor_normalized_xy: None,
            pivot_normalized_xy: None,
            margin_px: None,
            interaction_enabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticTextObjectSchema {
    pub font_asset_ref: AssetRef,
    pub text: String,
    pub position_xyz: [f64; 3],
    pub scale_xy: [f64; 2],
    pub rotation_deg_xyz: [f64; 3],
    pub is_visible: bool,
    #[serde(default, alias = "node_tag", skip_serializing_if = "Option::is_none")]
    pub named_handle: Option<String>,
    #[serde(
        default,
        alias = "parent_node_tag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_named_handle: Option<String>,
    pub scene_id: Option<String>,
    pub presentation_space: Option<StaticSpritePresentationSpaceSchema>,
    pub presentation_sizing_mode: Option<StaticSpritePresentationSizingModeSchema>,
    pub anchor_normalized_xy: Option<[f64; 2]>,
    pub pivot_normalized_xy: Option<[f64; 2]>,
    pub margin_px: Option<[f64; 2]>,
    pub interaction_enabled: bool,
    pub color_rgba: Option<[f64; 4]>,
    pub outline_color_rgba: Option<[f64; 4]>,
    pub outline_thickness_px: Option<f64>,
}

impl StaticTextObjectSchema {
    pub fn new(
        font_asset_ref: AssetRef,
        text: impl Into<String>,
        position_xyz: [f64; 3],
        scale_xy: [f64; 2],
    ) -> Self {
        Self {
            font_asset_ref,
            text: text.into(),
            position_xyz,
            scale_xy,
            rotation_deg_xyz: [0.0, 0.0, 0.0],
            is_visible: true,
            named_handle: None,
            parent_named_handle: None,
            scene_id: None,
            presentation_space: None,
            presentation_sizing_mode: None,
            anchor_normalized_xy: None,
            pivot_normalized_xy: None,
            margin_px: None,
            interaction_enabled: false,
            color_rgba: None,
            outline_color_rgba: None,
            outline_thickness_px: None,
        }
    }
}
