use super::world_object_schema::WorldObjectSchema;
use crate::{
    assets::asset_ref::AssetRef,
    properties::authored_property_view::AuthoredPropertyView,
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
    property_view: AuthoredPropertyView<'a>,
}

impl<'a> AuthoredWorldObjectView<'a> {
    pub fn new(world_object_schema: &'a WorldObjectSchema) -> Self {
        Self {
            world_object_schema,
            property_view: AuthoredPropertyView::new(&world_object_schema.properties),
        }
    }

    pub fn properties(&self) -> &'a PropertyMap {
        &self.world_object_schema.properties
    }

    pub fn property_view(&self) -> AuthoredPropertyView<'a> {
        self.property_view
    }

    pub fn kind_name(&self) -> Option<&'a str> {
        self.string("object_type").map(|value| value.as_str())
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
        self.property_view.bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.property_view.float(property_name)
    }

    pub fn int(&self, property_name: &str) -> Option<i64> {
        self.property_view.int(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
        self.property_view.float_array(property_name)
    }

    pub fn string(&self, property_name: &str) -> Option<&'a String> {
        self.property_view.string(property_name)
    }

    pub fn string_array(&self, property_name: &str) -> Option<&'a Vec<String>> {
        self.properties().get_string_array(property_name)
    }

    pub fn asset_ref(&self, property_name: &str) -> Option<&'a AssetRef> {
        self.property_view.asset_ref(property_name)
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
            _ => self.property_view.asset_refs().collect(),
        };
        asset_refs.into_iter()
    }

    pub fn sanitized_string(&self, property_name: &str) -> Option<String> {
        self.property_view.sanitized_string(property_name)
    }

    pub fn node_tag(&self) -> Option<String> {
        self.sanitized_string("node_tag")
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.sanitized_string("parent_node_tag")
    }

    pub fn positive_dimension(&self, property_name: &str) -> u32 {
        self.property_view.positive_dimension(property_name)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AuthoredCameraObjectView<'a> {
    world_object_view: AuthoredWorldObjectView<'a>,
}

impl<'a> AuthoredCameraObjectView<'a> {
    pub fn property_view(&self) -> AuthoredPropertyView<'a> {
        self.world_object_view.property_view()
    }

    pub fn node_tag(&self) -> Option<String> {
        self.world_object_view.node_tag()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.world_object_view.float(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
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

    pub fn node_tag(&self) -> Option<String> {
        self.world_object_view.node_tag()
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.world_object_view.parent_node_tag()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
        self.world_object_view.float_array(property_name)
    }

    pub fn positive_dimension(&self, property_name: &str) -> u32 {
        self.world_object_view.positive_dimension(property_name)
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

    pub fn node_tag(&self) -> Option<String> {
        self.world_object_view.node_tag()
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.world_object_view.parent_node_tag()
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.world_object_view.bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.world_object_view.float(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
        self.world_object_view.float_array(property_name)
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

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
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
        properties.insert_string("node_tag", " sprite:tag ");
        properties.insert_int("tile_width_px", 12);
        properties.insert_asset_ref(
            "asset_ref",
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")),
        );

        let world_object_schema = WorldObjectSchema {
            properties,
            state_machines: Vec::new(),
        };
        let view = AuthoredWorldObjectView::new(&world_object_schema);

        assert_eq!(view.kind(), Some(AuthoredWorldObjectKind::StaticSprite));
        assert_eq!(view.node_tag().as_deref(), Some("sprite:tag"));
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
        properties.insert_string("node_tag", "   ");
        properties.insert_int("tile_width_px", 0);

        let world_object_schema = WorldObjectSchema {
            properties,
            state_machines: Vec::new(),
        };
        let view = AuthoredWorldObjectView::new(&world_object_schema);

        assert_eq!(view.node_tag(), None);
        assert_eq!(view.positive_dimension("tile_width_px"), 0);
        assert_eq!(view.kind(), None);
    }
}
