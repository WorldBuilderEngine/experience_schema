use crate::{
    assets::asset_ref::AssetRef,
    properties::{property::Property, property_map::PropertyMap},
};

#[derive(Clone, Copy, Debug)]
pub struct AuthoredPropertyView<'a> {
    properties: &'a PropertyMap,
}

impl<'a> AuthoredPropertyView<'a> {
    pub fn new(properties: &'a PropertyMap) -> Self {
        Self { properties }
    }

    pub fn properties(&self) -> &'a PropertyMap {
        self.properties
    }

    pub fn bool(&self, property_name: &str) -> Option<bool> {
        self.properties.get_bool(property_name)
    }

    pub fn float(&self, property_name: &str) -> Option<f64> {
        self.properties.get_float(property_name)
    }

    pub fn int(&self, property_name: &str) -> Option<i64> {
        self.properties.get_int(property_name)
    }

    pub fn float_array(&self, property_name: &str) -> Option<&'a Vec<f64>> {
        self.properties.get_float_array(property_name)
    }

    pub fn string(&self, property_name: &str) -> Option<&'a String> {
        self.properties.get_string(property_name)
    }

    pub fn asset_ref(&self, property_name: &str) -> Option<&'a AssetRef> {
        self.properties.get_asset_ref(property_name)
    }

    pub fn asset_refs(&self) -> impl Iterator<Item = &'a AssetRef> {
        self.properties
            .into_iter()
            .filter_map(|(_property_name, property)| match property {
                Property::AssetRef(asset_ref) => Some(asset_ref),
                _ => None,
            })
    }

    pub fn sanitized_string(&self, property_name: &str) -> Option<String> {
        let raw_property_value = self.string(property_name)?;
        let sanitized_property_value = raw_property_value.trim();
        if sanitized_property_value.is_empty() {
            return None;
        }

        Some(sanitized_property_value.to_string())
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

    pub fn positive_float_from(&self, property_names: &[&str]) -> Option<f64> {
        for property_name in property_names {
            let Some(raw_value) = self.float(property_name) else {
                continue;
            };
            if !raw_value.is_finite() || raw_value <= 0.0 {
                continue;
            }

            return Some(raw_value);
        }

        None
    }

    pub fn positive_size2_from(&self, property_names: &[&str]) -> Option<(f64, f64)> {
        self.read_size2_from(property_names, |component| component > 0.0)
    }

    pub fn non_negative_size2_from(&self, property_names: &[&str]) -> Option<(f64, f64)> {
        self.read_size2_from(property_names, |component| component >= 0.0)
    }

    pub fn normalized_string_from(&self, property_names: &[&str]) -> Option<String> {
        for property_name in property_names {
            let Some(raw_value) = self.string(property_name) else {
                continue;
            };

            let normalized_value = raw_value
                .trim()
                .to_ascii_lowercase()
                .replace('-', "_")
                .replace(' ', "_");
            if normalized_value.is_empty() {
                continue;
            }

            return Some(normalized_value);
        }

        None
    }

    fn read_size2_from(
        &self,
        property_names: &[&str],
        is_valid_component: impl Fn(f64) -> bool,
    ) -> Option<(f64, f64)> {
        for property_name in property_names {
            let Some(raw_components) = self.float_array(property_name) else {
                continue;
            };
            if raw_components.len() < 2 {
                continue;
            }

            let width = raw_components[0];
            let height = raw_components[1];
            if !width.is_finite()
                || !height.is_finite()
                || !is_valid_component(width)
                || !is_valid_component(height)
            {
                continue;
            }

            return Some((width, height));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::AuthoredPropertyView;
    use crate::{assets::asset_ref::AssetRef, properties::property_map::PropertyMap};
    use std::path::PathBuf;

    #[test]
    fn authored_property_view_reads_canonical_values() {
        let mut properties = PropertyMap::new();
        properties.insert_string("node_tag", " ui:start_button ");
        properties.insert_int("tile_width_px", 12);
        properties.insert_float("pixels_per_unit", 96.0);
        properties.insert_float_array("default_camera_viewport_size_px", vec![960.0, 540.0]);
        properties.insert_asset_ref(
            "asset_ref",
            AssetRef::new_with_bundle_id("embedded", PathBuf::from("sprite.png")),
        );

        let view = AuthoredPropertyView::new(&properties);

        assert_eq!(
            view.sanitized_string("node_tag").as_deref(),
            Some("ui:start_button")
        );
        assert_eq!(view.positive_dimension("tile_width_px"), 12);
        assert_eq!(view.positive_float_from(&["pixels_per_unit"]), Some(96.0));
        assert_eq!(
            view.positive_size2_from(&["default_camera_viewport_size_px"]),
            Some((960.0, 540.0))
        );
        assert_eq!(
            view.asset_ref("asset_ref")
                .and_then(|asset_ref| asset_ref.get_bundle_id()),
            Some("embedded")
        );
    }

    #[test]
    fn authored_property_view_normalizes_and_filters_invalid_values() {
        let mut properties = PropertyMap::new();
        properties.insert_string("presentation_policy_2d", " Fixed Height-Reveal ");
        properties.insert_float_array("default_camera_viewport_size_px", vec![0.0, -1.0]);
        properties.insert_float_array("ui_safe_frame_inset_px", vec![24.0, 12.0]);
        properties.insert_float("ultrawide_cap_aspect_ratio", 0.0);

        let view = AuthoredPropertyView::new(&properties);

        assert_eq!(
            view.normalized_string_from(&["presentation_policy_2d"])
                .as_deref(),
            Some("fixed_height_reveal")
        );
        assert_eq!(
            view.positive_size2_from(&["default_camera_viewport_size_px"]),
            None
        );
        assert_eq!(
            view.non_negative_size2_from(&["ui_safe_frame_inset_px"]),
            Some((24.0, 12.0))
        );
        assert_eq!(
            view.positive_float_from(&["ultrawide_cap_aspect_ratio"]),
            None
        );
    }
}
