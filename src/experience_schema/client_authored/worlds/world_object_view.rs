use super::kinded_world_object_schema::KindedWorldObjectSchema;
use super::world_object_schema::WorldObjectSchema;
use crate::client_authored::worlds::typed_object_schemas::{
    StaticSpritePresentationSizingModeSchema, StaticSpritePresentationSpaceSchema,
};
use crate::{
    assets::asset_ref::AssetRef, properties::authored_property_view::AuthoredPropertyView,
    properties::property_map::PropertyMap,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthoredWorldObjectKind {
    Camera,
    StaticSprite,
    StaticText,
    Physics2dPolygon,
    Custom,
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredWorldObjectView<'a> {
    world_object_schema: &'a WorldObjectSchema,
    property_view: Option<AuthoredPropertyView<'a>>,
}

impl<'a> AuthoredWorldObjectView<'a> {
    pub fn new(world_object_schema: &'a WorldObjectSchema) -> Self {
        Self {
            world_object_schema,
            property_view: (!world_object_schema.properties.is_empty())
                .then(|| AuthoredPropertyView::new(&world_object_schema.properties)),
        }
    }

    pub fn properties(&self) -> Option<&'a PropertyMap> {
        self.property_view
            .map(|property_view| property_view.properties())
    }

    pub fn property_view(&self) -> Option<AuthoredPropertyView<'a>> {
        self.property_view
    }

    pub fn kind_name(&self) -> Option<&'a str> {
        self.world_object_schema
            .kinded
            .as_ref()
            .map(KindedWorldObjectSchema::canonical_object_type_name)
            .or_else(|| self.string("object_type").map(|value| value.as_str()))
    }

    pub fn object_type(&self) -> Option<&'a str> {
        self.kind_name()
    }

    pub fn is_object_type(&self, expected_object_type: &str) -> bool {
        matches!(self.object_type(), Some(actual_object_type) if actual_object_type == expected_object_type)
    }

    pub fn kind(&self) -> Option<AuthoredWorldObjectKind> {
        match self.kind_name()? {
            "camera" => Some(AuthoredWorldObjectKind::Camera),
            "static_sprite" => Some(AuthoredWorldObjectKind::StaticSprite),
            "static_text" => Some(AuthoredWorldObjectKind::StaticText),
            "physics2d_polygon" => Some(AuthoredWorldObjectKind::Physics2dPolygon),
            _ => Some(AuthoredWorldObjectKind::Custom),
        }
    }

    pub fn as_camera(&self) -> Option<AuthoredCameraObjectView<'a>> {
        matches!(self.kind(), Some(AuthoredWorldObjectKind::Camera)).then_some(
            AuthoredCameraObjectView {
                world_object_view: *self,
            },
        )
    }

    pub fn as_static_sprite(&self) -> Option<AuthoredStaticSpriteObjectView<'a>> {
        matches!(self.kind(), Some(AuthoredWorldObjectKind::StaticSprite)).then_some(
            AuthoredStaticSpriteObjectView {
                world_object_view: *self,
            },
        )
    }

    pub fn as_static_text(&self) -> Option<AuthoredStaticTextObjectView<'a>> {
        matches!(self.kind(), Some(AuthoredWorldObjectKind::StaticText)).then_some(
            AuthoredStaticTextObjectView {
                world_object_view: *self,
            },
        )
    }

    pub fn as_physics2d_polygon(&self) -> Option<AuthoredPhysics2dPolygonObjectView<'a>> {
        matches!(self.kind(), Some(AuthoredWorldObjectKind::Physics2dPolygon)).then_some(
            AuthoredPhysics2dPolygonObjectView {
                world_object_view: *self,
            },
        )
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::Camera(camera)) => match property_name {
                "is_active_camera" => Some(camera.is_active_camera),
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => match property_name {
                "is_visible" => Some(static_sprite.is_visible),
                "visible_when_scene_active" => static_sprite.visible_when_scene_active,
                "repeat_x" => static_sprite.repeat_x,
                "repeat_y" => static_sprite.repeat_y,
                "interaction_enabled" => Some(static_sprite.interaction_enabled),
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticText(static_text)) => match property_name {
                "is_visible" => Some(static_text.is_visible),
                "interaction_enabled" => Some(static_text.interaction_enabled),
                _ => None,
            },
            Some(KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker)) => {
                match property_name {
                    "is_visible" => Some(hotspot_marker.is_visible),
                    "interaction_enabled" => Some(hotspot_marker.interaction_enabled),
                    _ => None,
                }
            }
            _ => self
                .property_view
                .and_then(|property_view| property_view.bool(property_name)),
        }
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::Camera(camera)) => match property_name {
                "pixels_per_unit" => match camera.projection {
                    crate::client_authored::worlds::typed_object_schemas::CameraProjectionSchema::Orthographic2d { pixels_per_unit } => Some(pixels_per_unit),
                    _ => None,
                },
                "focal_length" => match camera.projection {
                    crate::client_authored::worlds::typed_object_schemas::CameraProjectionSchema::Perspective3d { focal_length, .. } => Some(focal_length),
                    _ => None,
                },
                "debug_movement_units_per_second" => camera
                    .debug_movement_units_per_second
                    .is_finite()
                    .then_some(camera.debug_movement_units_per_second),
                "arm_distance" => camera.arm_distance,
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticText(static_text)) => match property_name {
                "outline_thickness_px" => static_text.outline_thickness_px,
                _ => None,
            },
            _ => self.property_view.and_then(|property_view| property_view.float(property_name)),
        }
    }

    pub fn int(&self, property_name: &str) -> Option<i64> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => match property_name {
                "tile_width_px" => static_sprite.tile_width_px.map(i64::from),
                "tile_height_px" => static_sprite.tile_height_px.map(i64::from),
                "intrinsic_width_px" => static_sprite.intrinsic_width_px.map(i64::from),
                "intrinsic_height_px" => static_sprite.intrinsic_height_px.map(i64::from),
                _ => None,
            },
            _ => self
                .property_view
                .and_then(|property_view| property_view.int(property_name)),
        }
    }

    pub fn float_array(&self, property_name: &str) -> Option<Vec<f64>> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::Camera(camera)) => match property_name {
                "position" => Some(camera.position_xyz.to_vec()),
                "camera_forward" => match &camera.projection {
                    crate::client_authored::worlds::typed_object_schemas::CameraProjectionSchema::Perspective3d {
                        camera_forward_xyz,
                        ..
                    } => Some(camera_forward_xyz.to_vec()),
                    _ => None,
                },
                "camera_up" => match &camera.projection {
                    crate::client_authored::worlds::typed_object_schemas::CameraProjectionSchema::Perspective3d {
                        camera_up_xyz,
                        ..
                    } => Some(camera_up_xyz.to_vec()),
                    _ => None,
                },
                "follow_target_distance" => camera
                    .follow_target_distance_xyz
                    .map(|value| value.to_vec()),
                "follow_units_per_second" => camera
                    .follow_units_per_second_xyz
                    .map(|value| value.to_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => match property_name {
                "position" => Some(static_sprite.position_xyz.to_vec()),
                "scale" => Some(static_sprite.scale_xy.to_vec()),
                "rotation_deg" => Some(static_sprite.rotation_deg_xyz.to_vec()),
                "anchor_normalized_xy" => static_sprite.anchor_normalized_xy.map(|value| value.to_vec()),
                "pivot_normalized_xy" => static_sprite.pivot_normalized_xy.map(|value| value.to_vec()),
                "margin_px" => static_sprite.margin_px.map(|value| value.to_vec()),
                "interaction_bounds_px" => static_sprite
                    .interaction_bounds_px
                    .map(|value| value.to_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::StaticText(static_text)) => match property_name {
                "position" => Some(static_text.position_xyz.to_vec()),
                "scale" => Some(static_text.scale_xy.to_vec()),
                "rotation_deg" => Some(static_text.rotation_deg_xyz.to_vec()),
                "anchor_normalized_xy" => static_text.anchor_normalized_xy.map(|value| value.to_vec()),
                "pivot_normalized_xy" => static_text.pivot_normalized_xy.map(|value| value.to_vec()),
                "margin_px" => static_text.margin_px.map(|value| value.to_vec()),
                "color_rgba" => static_text.color_rgba.map(|value| value.to_vec()),
                "outline_color_rgba" => static_text
                    .outline_color_rgba
                    .map(|value| value.to_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::TransitionHotspot(transition_hotspot)) => match property_name {
                "bounds_px" => Some(transition_hotspot.bounds_px.as_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::InteractableHotspot(interactable_hotspot)) => match property_name {
                "bounds_px" => Some(interactable_hotspot.bounds_px.as_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker)) => match property_name {
                "position" => Some(hotspot_marker.position_xyz.to_vec()),
                "scale" => Some(hotspot_marker.scale_xy.to_vec()),
                "rotation_deg" => Some(hotspot_marker.rotation_deg_xyz.to_vec()),
                _ => self.property_view.and_then(|property_view| property_view.float_array(property_name).cloned()),
            },
            Some(KindedWorldObjectSchema::UiRect(_) | KindedWorldObjectSchema::UiHitRegion(_)) => self
                .property_view
                .and_then(|property_view| property_view.float_array(property_name).cloned()),
            None => self
                .property_view
                .and_then(|property_view| property_view.float_array(property_name).cloned()),
        }
    }

    pub fn string(&self, property_name: &str) -> Option<&'a String> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::Camera(camera)) => match property_name {
                "named_handle" | "node_tag" => camera.named_handle.as_ref(),
                "follow_target_named_handle" | "follow_target_node_tag" => {
                    camera.follow_target_named_handle.as_ref()
                }
                "follow_scroll_type" => camera.follow_scroll_type.as_ref(),
                "camera_projection" => None,
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => match property_name {
                "named_handle" | "node_tag" => static_sprite.named_handle.as_ref(),
                "parent_named_handle" | "parent_node_tag" => {
                    static_sprite.parent_named_handle.as_ref()
                }
                "scene_id" => static_sprite.scene_id.as_ref(),
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticText(static_text)) => match property_name {
                "text" => Some(&static_text.text),
                "named_handle" | "node_tag" => static_text.named_handle.as_ref(),
                "parent_named_handle" | "parent_node_tag" => {
                    static_text.parent_named_handle.as_ref()
                }
                "scene_id" => static_text.scene_id.as_ref(),
                _ => None,
            },
            Some(KindedWorldObjectSchema::TransitionHotspot(transition_hotspot)) => {
                match property_name {
                    "object_type" => Some(&transition_hotspot.object_type),
                    "hotspot_id" => Some(&transition_hotspot.hotspot_id),
                    "from_scene_id" => Some(&transition_hotspot.from_scene_id),
                    "to_scene_id" => Some(&transition_hotspot.to_scene_id),
                    "activation_event" => transition_hotspot.activation_event.as_ref(),
                    "transition_started_event" => {
                        transition_hotspot.transition_started_event.as_ref()
                    }
                    "transition_completed_event" => {
                        transition_hotspot.transition_completed_event.as_ref()
                    }
                    _ => None,
                }
            }
            Some(KindedWorldObjectSchema::InteractableHotspot(interactable_hotspot)) => {
                match property_name {
                    "object_type" => Some(&interactable_hotspot.object_type),
                    "scene_id" => Some(&interactable_hotspot.scene_id),
                    "hotspot_id" => Some(&interactable_hotspot.hotspot_id),
                    "target_id" => Some(&interactable_hotspot.target_id),
                    "verb_id" => interactable_hotspot.verb_id.as_ref(),
                    "item_id" => interactable_hotspot.item_id.as_ref(),
                    "required_item_id" => interactable_hotspot.required_item_id.as_ref(),
                    "activation_event" => interactable_hotspot.activation_event.as_ref(),
                    "interaction_resolved_event" => {
                        interactable_hotspot.interaction_resolved_event.as_ref()
                    }
                    "inventory_collected_event" => {
                        interactable_hotspot.inventory_collected_event.as_ref()
                    }
                    "gate_blocked_event" => interactable_hotspot.gate_blocked_event.as_ref(),
                    "gate_unlocked_event" => interactable_hotspot.gate_unlocked_event.as_ref(),
                    "hover_entered_event" => interactable_hotspot.hover_entered_event.as_ref(),
                    "hover_exited_event" => interactable_hotspot.hover_exited_event.as_ref(),
                    "pressed_event" => interactable_hotspot.pressed_event.as_ref(),
                    _ => None,
                }
            }
            Some(KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker)) => {
                match property_name {
                    "scene_id" => Some(&hotspot_marker.scene_id),
                    "hotspot_id" => Some(&hotspot_marker.hotspot_id),
                    "marker_kind" => Some(&hotspot_marker.marker_kind),
                    "node_tag" => hotspot_marker.node_tag.as_ref(),
                    _ => None,
                }
            }
            _ => match property_name {
                "named_handle" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("named_handle")
                        .or_else(|| property_view.string("node_tag"))
                }),
                "parent_named_handle" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("parent_named_handle")
                        .or_else(|| property_view.string("parent_node_tag"))
                }),
                "follow_target_named_handle" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("follow_target_named_handle")
                        .or_else(|| property_view.string("follow_target_node_tag"))
                }),
                "node_tag" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("node_tag")
                        .or_else(|| property_view.string("named_handle"))
                }),
                "parent_node_tag" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("parent_node_tag")
                        .or_else(|| property_view.string("parent_named_handle"))
                }),
                "follow_target_node_tag" => self.property_view.and_then(|property_view| {
                    property_view
                        .string("follow_target_node_tag")
                        .or_else(|| property_view.string("follow_target_named_handle"))
                }),
                _ => self
                    .property_view
                    .and_then(|property_view| property_view.string(property_name)),
            },
        }
    }

    pub fn string_array(&self, property_name: &str) -> Option<&'a Vec<String>> {
        self.properties()
            .and_then(|properties| properties.get_string_array(property_name))
    }

    pub fn asset_ref(&self, property_name: &str) -> Option<&'a AssetRef> {
        match self.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => match property_name {
                "asset_ref" => Some(&static_sprite.asset_ref),
                _ => None,
            },
            Some(KindedWorldObjectSchema::StaticText(static_text)) => match property_name {
                "font_asset_ref" => Some(&static_text.font_asset_ref),
                _ => None,
            },
            Some(KindedWorldObjectSchema::InteractableHotspot(interactable_hotspot)) => {
                match property_name {
                    "default_asset_ref" => interactable_hotspot.default_asset_ref.as_ref(),
                    "hover_asset_ref" => interactable_hotspot.hover_asset_ref.as_ref(),
                    _ => None,
                }
            }
            Some(KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker)) => {
                match property_name {
                    "asset_ref" => Some(&hotspot_marker.asset_ref),
                    _ => None,
                }
            }
            _ => self
                .property_view
                .and_then(|property_view| property_view.asset_ref(property_name)),
        }
    }

    pub fn asset_refs(&self) -> std::vec::IntoIter<&'a AssetRef> {
        let asset_refs: Vec<&'a AssetRef> = match self.kind() {
            Some(AuthoredWorldObjectKind::StaticSprite) => self
                .as_static_sprite()
                .and_then(|sprite| sprite.asset_ref())
                .into_iter()
                .collect(),
            Some(AuthoredWorldObjectKind::StaticText) => self
                .as_static_text()
                .and_then(|text| text.font_asset_ref())
                .into_iter()
                .collect(),
            None => self
                .property_view
                .map(|property_view| property_view.asset_refs().collect())
                .unwrap_or_default(),
            Some(AuthoredWorldObjectKind::Custom) => self
                .property_view
                .map(|property_view| property_view.asset_refs().collect())
                .unwrap_or_default(),
            _ => self
                .world_object_schema
                .kinded
                .as_ref()
                .map(|kinded| match kinded {
                    KindedWorldObjectSchema::InteractableHotspot(interactable_hotspot) => {
                        let mut asset_refs = Vec::new();
                        if let Some(default_asset_ref) =
                            interactable_hotspot.default_asset_ref.as_ref()
                        {
                            asset_refs.push(default_asset_ref);
                        }
                        if let Some(hover_asset_ref) = interactable_hotspot.hover_asset_ref.as_ref()
                        {
                            asset_refs.push(hover_asset_ref);
                        }
                        asset_refs
                    }
                    KindedWorldObjectSchema::HotspotMarkerSprite(hotspot_marker) => {
                        vec![&hotspot_marker.asset_ref]
                    }
                    _ => Vec::new(),
                })
                .unwrap_or_default(),
        };
        asset_refs.into_iter()
    }

    pub fn sanitized_string(&self, property_name: &str) -> Option<String> {
        self.string(property_name)
            .map(|raw_value| raw_value.trim())
            .filter(|sanitized_property_value| !sanitized_property_value.is_empty())
            .map(|sanitized_property_value| sanitized_property_value.to_string())
    }

    pub fn named_handle(&self) -> Option<String> {
        self.sanitized_string("named_handle")
            .or_else(|| self.sanitized_string("node_tag"))
    }

    pub fn node_tag(&self) -> Option<String> {
        self.named_handle()
    }

    pub fn parent_named_handle(&self) -> Option<String> {
        self.sanitized_string("parent_named_handle")
            .or_else(|| self.sanitized_string("parent_node_tag"))
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.parent_named_handle()
    }

    pub fn follow_target_named_handle(&self) -> Option<String> {
        self.sanitized_string("follow_target_named_handle")
            .or_else(|| self.sanitized_string("follow_target_node_tag"))
    }

    pub fn positive_dimension(&self, property_name: &str) -> u32 {
        if let Some(raw_dimension) = self.int(property_name)
            && raw_dimension > 0
            && let Ok(dimension) = u32::try_from(raw_dimension)
        {
            return dimension;
        }

        0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredCameraObjectView<'a> {
    world_object_view: AuthoredWorldObjectView<'a>,
}

impl<'a> AuthoredCameraObjectView<'a> {
    pub fn property_view(&self) -> Option<AuthoredPropertyView<'a>> {
        self.world_object_view.property_view()
    }

    pub fn named_handle(&self) -> Option<String> {
        self.world_object_view.named_handle()
    }

    pub fn node_tag(&self) -> Option<String> {
        self.named_handle()
    }

    pub fn follow_target_named_handle(&self) -> Option<String> {
        self.world_object_view.follow_target_named_handle()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.world_object_view.float(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<Vec<f64>> {
        self.world_object_view.float_array(property_name)
    }

    pub fn string(&self, property_name: &str) -> Option<&'a String> {
        self.world_object_view.string(property_name)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredStaticSpriteObjectView<'a> {
    world_object_view: AuthoredWorldObjectView<'a>,
}

impl<'a> AuthoredStaticSpriteObjectView<'a> {
    pub fn asset_ref(&self) -> Option<&'a AssetRef> {
        self.world_object_view.asset_ref("asset_ref")
    }

    pub fn named_handle(&self) -> Option<String> {
        self.world_object_view.named_handle()
    }

    pub fn node_tag(&self) -> Option<String> {
        self.named_handle()
    }

    pub fn parent_named_handle(&self) -> Option<String> {
        self.world_object_view.parent_named_handle()
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.parent_named_handle()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<Vec<f64>> {
        self.world_object_view.float_array(property_name)
    }

    pub fn positive_dimension(&self, property_name: &str) -> u32 {
        self.world_object_view.positive_dimension(property_name)
    }

    pub fn presentation_space(&self) -> Option<StaticSpritePresentationSpaceSchema> {
        match self.world_object_view.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => {
                static_sprite.presentation_space
            }
            _ => self
                .world_object_view
                .property_view
                .and_then(|property_view| property_view.string("presentation_space"))
                .and_then(|raw_value| match raw_value.trim() {
                    "world" => Some(StaticSpritePresentationSpaceSchema::World),
                    "presented_viewport" => {
                        Some(StaticSpritePresentationSpaceSchema::PresentedViewport)
                    }
                    "gameplay_frame" => Some(StaticSpritePresentationSpaceSchema::GameplayFrame),
                    _ => None,
                }),
        }
    }

    pub fn presentation_sizing_mode(&self) -> Option<StaticSpritePresentationSizingModeSchema> {
        match self.world_object_view.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticSprite(static_sprite)) => {
                static_sprite.presentation_sizing_mode
            }
            _ => self
                .world_object_view
                .property_view
                .and_then(|property_view| property_view.string("presentation_sizing_mode"))
                .and_then(|raw_value| match raw_value.trim() {
                    "authored" => Some(StaticSpritePresentationSizingModeSchema::Authored),
                    "fit" => Some(StaticSpritePresentationSizingModeSchema::Fit),
                    "cover" => Some(StaticSpritePresentationSizingModeSchema::Cover),
                    "stretch" => Some(StaticSpritePresentationSizingModeSchema::Stretch),
                    "tile" => Some(StaticSpritePresentationSizingModeSchema::Tile),
                    _ => None,
                }),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredStaticTextObjectView<'a> {
    world_object_view: AuthoredWorldObjectView<'a>,
}

impl<'a> AuthoredStaticTextObjectView<'a> {
    pub fn font_asset_ref(&self) -> Option<&'a AssetRef> {
        self.world_object_view.asset_ref("font_asset_ref")
    }

    pub fn text(&self) -> Option<String> {
        self.world_object_view.sanitized_string("text")
    }

    pub fn named_handle(&self) -> Option<String> {
        self.world_object_view.named_handle()
    }

    pub fn node_tag(&self) -> Option<String> {
        self.named_handle()
    }

    pub fn parent_named_handle(&self) -> Option<String> {
        self.world_object_view.parent_named_handle()
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.parent_named_handle()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.world_object_view.float(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<Vec<f64>> {
        self.world_object_view.float_array(property_name)
    }

    pub fn presentation_space(&self) -> Option<StaticSpritePresentationSpaceSchema> {
        match self.world_object_view.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticText(static_text)) => {
                static_text.presentation_space
            }
            _ => self
                .world_object_view
                .property_view
                .and_then(|property_view| property_view.string("presentation_space"))
                .and_then(|raw_value| match raw_value.trim() {
                    "world" => Some(StaticSpritePresentationSpaceSchema::World),
                    "presented_viewport" => {
                        Some(StaticSpritePresentationSpaceSchema::PresentedViewport)
                    }
                    "gameplay_frame" => Some(StaticSpritePresentationSpaceSchema::GameplayFrame),
                    _ => None,
                }),
        }
    }

    pub fn presentation_sizing_mode(&self) -> Option<StaticSpritePresentationSizingModeSchema> {
        match self.world_object_view.world_object_schema.kinded.as_ref() {
            Some(KindedWorldObjectSchema::StaticText(static_text)) => {
                static_text.presentation_sizing_mode
            }
            _ => self
                .world_object_view
                .property_view
                .and_then(|property_view| property_view.string("presentation_sizing_mode"))
                .and_then(|raw_value| match raw_value.trim() {
                    "authored" => Some(StaticSpritePresentationSizingModeSchema::Authored),
                    "fit" => Some(StaticSpritePresentationSizingModeSchema::Fit),
                    "cover" => Some(StaticSpritePresentationSizingModeSchema::Cover),
                    "stretch" => Some(StaticSpritePresentationSizingModeSchema::Stretch),
                    "tile" => Some(StaticSpritePresentationSizingModeSchema::Tile),
                    _ => None,
                }),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredPhysics2dPolygonObjectView<'a> {
    world_object_view: AuthoredWorldObjectView<'a>,
}

impl<'a> AuthoredPhysics2dPolygonObjectView<'a> {
    pub fn int(&self, property_name: &str) -> Option<i64> {
        self.world_object_view.int(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<Vec<f64>> {
        self.world_object_view.float_array(property_name)
    }

    pub fn string_array(&self, property_name: &str) -> Option<&'a Vec<String>> {
        self.world_object_view.string_array(property_name)
    }
}

#[cfg(test)]
mod tests {
    use super::{AuthoredWorldObjectKind, AuthoredWorldObjectView};
    use crate::{
        assets::asset_ref::AssetRef,
        client_authored::worlds::world_object_schema::WorldObjectSchema,
        properties::property_map::PropertyMap,
    };
    use std::path::PathBuf;

    #[test]
    fn authored_world_object_view_reads_canonical_fields() {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "static_sprite");
        properties.insert_string("named_handle", " sprite:tag ");
        properties.insert_int("tile_width_px", 12);
        properties.insert_asset_ref(
            "asset_ref",
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")),
        );

        let world_object_schema = WorldObjectSchema {
            kinded: None,
            properties,
            state_machines: Vec::new(),
        };
        let view = AuthoredWorldObjectView::new(&world_object_schema);

        assert_eq!(view.kind(), Some(AuthoredWorldObjectKind::StaticSprite));
        assert_eq!(view.named_handle().as_deref(), Some("sprite:tag"));
        assert_eq!(view.positive_dimension("tile_width_px"), 12);
        assert_eq!(
            view.asset_ref("asset_ref")
                .and_then(|asset_ref| asset_ref.get_bundle_id()),
            Some("embedded")
        );
        assert!(view.as_static_sprite().is_some());
    }

    #[test]
    fn authored_world_object_view_exposes_typed_static_text_fields() {
        let mut properties = PropertyMap::new();
        properties.insert_string("object_type", "static_text");
        properties.insert_string("text", " hello ");
        properties.insert_asset_ref(
            "font_asset_ref",
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("font.ttf")),
        );

        let world_object_schema = WorldObjectSchema {
            kinded: None,
            properties,
            state_machines: Vec::new(),
        };
        let text_view = AuthoredWorldObjectView::new(&world_object_schema)
            .as_static_text()
            .expect("static text should resolve");

        assert_eq!(text_view.text().as_deref(), Some("hello"));
        assert_eq!(
            text_view
                .font_asset_ref()
                .and_then(|asset_ref| asset_ref.get_bundle_id()),
            Some("embedded")
        );
    }

    #[test]
    fn authored_world_object_view_ignores_blank_tag_and_non_positive_dimensions() {
        let mut properties = PropertyMap::new();
        properties.insert_string("named_handle", "   ");
        properties.insert_int("tile_width_px", 0);

        let world_object_schema = WorldObjectSchema {
            kinded: None,
            properties,
            state_machines: Vec::new(),
        };
        let view = AuthoredWorldObjectView::new(&world_object_schema);

        assert_eq!(view.named_handle(), None);
        assert_eq!(view.positive_dimension("tile_width_px"), 0);
        assert_eq!(view.kind(), None);
    }
}
