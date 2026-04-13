use super::world_object_schema::WorldObjectSchema;
use crate::{assets::asset_ref::AssetRef, properties::property_map::PropertyMap};

#[derive(Clone, Copy, Debug)]
pub struct AuthoredWorldObjectView<'a> {
    world_object_schema: &'a WorldObjectSchema,
}

impl<'a> AuthoredWorldObjectView<'a> {
    pub fn new(world_object_schema: &'a WorldObjectSchema) -> Self {
        Self { world_object_schema }
    }

    pub fn properties(&self) -> &'a PropertyMap {
        &self.world_object_schema.properties
    }

    pub fn object_type(&self) -> Option<&'a str> {
        self.string("object_type").map(|value| value.as_str())
    }

    pub fn is_object_type(
        &self,
        expected_object_type: &str,
    ) -> bool {
        matches!(self.object_type(), Some(actual_object_type) if actual_object_type == expected_object_type)
    }

    pub fn bool(
        &self,
        property_name: &str,
    ) -> Option<bool> {
        self.properties().get_bool(property_name)
    }

    pub fn float(
        &self,
        property_name: &str,
    ) -> Option<f64> {
        self.properties().get_float(property_name)
    }

    pub fn int(
        &self,
        property_name: &str,
    ) -> Option<i64> {
        self.properties().get_int(property_name)
    }

    pub fn float_array(
        &self,
        property_name: &str,
    ) -> Option<&'a Vec<f64>> {
        self.properties().get_float_array(property_name)
    }

    pub fn string(
        &self,
        property_name: &str,
    ) -> Option<&'a String> {
        self.properties().get_string(property_name)
    }

    pub fn asset_ref(
        &self,
        property_name: &str,
    ) -> Option<&'a AssetRef> {
        self.properties().get_asset_ref(property_name)
    }

    pub fn sanitized_string(
        &self,
        property_name: &str,
    ) -> Option<String> {
        let raw_property_value = self.string(property_name)?;
        let sanitized_property_value = raw_property_value.trim();
        if sanitized_property_value.is_empty() {
            return None;
        }

        Some(sanitized_property_value.to_string())
    }

    pub fn node_tag(&self) -> Option<String> {
        self.sanitized_string("node_tag")
    }

    pub fn parent_node_tag(&self) -> Option<String> {
        self.sanitized_string("parent_node_tag")
    }

    pub fn positive_dimension(
        &self,
        property_name: &str,
    ) -> u32 {
        if let Some(raw_dimension) = self.int(property_name)
            && raw_dimension > 0
            && let Ok(dimension) = u32::try_from(raw_dimension)
        {
            return dimension;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::AuthoredWorldObjectView;
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
        properties.insert_asset_ref("asset_ref", AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")));

        let world_object_schema = WorldObjectSchema {
            properties,
            state_machines: Vec::new(),
        };
        let view = AuthoredWorldObjectView::new(&world_object_schema);

        assert!(view.is_object_type("static_sprite"));
        assert_eq!(view.node_tag().as_deref(), Some("sprite:tag"));
        assert_eq!(view.positive_dimension("tile_width_px"), 12);
        assert_eq!(
            view.asset_ref("asset_ref").and_then(|asset_ref| asset_ref.get_bundle_id()),
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
    }
}
