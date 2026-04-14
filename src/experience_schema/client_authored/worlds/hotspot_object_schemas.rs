use crate::assets::asset_ref::AssetRef;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotspotBoundsPx {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl HotspotBoundsPx {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn as_vec(self) -> Vec<f64> {
        vec![
            f64::from(self.x),
            f64::from(self.y),
            f64::from(self.width),
            f64::from(self.height),
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransitionHotspotObjectSchema {
    pub object_type: String,
    pub hotspot_id: String,
    pub from_scene_id: String,
    pub to_scene_id: String,
    pub bounds_px: HotspotBoundsPx,
    pub activation_event: Option<String>,
    pub transition_started_event: Option<String>,
    pub transition_completed_event: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InteractableHotspotObjectSchema {
    pub object_type: String,
    pub scene_id: String,
    pub hotspot_id: String,
    pub target_id: String,
    pub bounds_px: HotspotBoundsPx,
    pub verb_id: Option<String>,
    pub item_id: Option<String>,
    pub required_item_id: Option<String>,
    pub consumes_required_item: Option<bool>,
    pub activation_event: Option<String>,
    pub interaction_resolved_event: Option<String>,
    pub inventory_collected_event: Option<String>,
    pub gate_blocked_event: Option<String>,
    pub gate_unlocked_event: Option<String>,
    pub default_asset_ref: Option<AssetRef>,
    pub hover_asset_ref: Option<AssetRef>,
    pub hover_entered_event: Option<String>,
    pub hover_exited_event: Option<String>,
    pub pressed_event: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HotspotMarkerSpriteObjectSchema {
    pub scene_id: String,
    pub hotspot_id: String,
    pub marker_kind: String,
    pub asset_ref: AssetRef,
    pub position_xyz: [f64; 3],
    pub scale_xy: [f64; 2],
    pub rotation_deg_xyz: [f64; 3],
    pub is_visible: bool,
    pub node_tag: Option<String>,
    pub interaction_enabled: bool,
}
