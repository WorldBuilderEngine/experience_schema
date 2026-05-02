use crate::{
    assets::asset_ref::AssetRef,
    client_authored::client_authored_schema::ClientAuthoredSchema,
    client_authored::state_machines::state_machine_boot_handle_binding_schema::{
        StateMachineBootHandleBindingSchema, StateMachineBootHandleKindSchema,
    },
    client_authored::state_machines::state_machine_boot_named_handle_binding_schema::StateMachineBootNamedHandleKindSchema,
    client_authored::state_machines::state_machine_schema::StateMachineSchema,
    client_authored::worlds::world_object_view::AuthoredWorldObjectView,
    client_authored::worlds::world_schema::WorldSchema,
    service_authored::service_authored_schema::ServiceAuthoredSchema,
};
use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
#[repr(i32)]
pub enum ExperienceSchemaVersion {
    V1 = 1,
}

pub const CURRENT_EXPERIENCE_SCHEMA_VERSION: u32 = ExperienceSchemaVersion::V1 as i32 as u32;

fn default_schema_version() -> u32 {
    CURRENT_EXPERIENCE_SCHEMA_VERSION
}

/// Describes a fully serialized experience consumed by runtimes/clients.
///
/// This is the universal target format for all published experiences.
/// Experiences may have their own internal schemas and formats that transpile to this format.
#[derive(Deserialize, Serialize, Clone, PartialEq, Message)]
pub struct ExperienceSchema {
    /// The version of this schema. Older versions will need migration steps.
    #[serde(default = "default_schema_version")]
    #[prost(uint32, tag = "1")]
    pub schema_version: u32,

    /// Schema populated by backend trusted services.
    #[prost(message, required, tag = "2")]
    pub service_authored_schema: ServiceAuthoredSchema,

    /// Schema populated by clients. May still need verification on the backend side if submitted for publishing.
    #[prost(message, required, tag = "3")]
    pub client_authored_schema: ClientAuthoredSchema,
}

impl ExperienceSchema {
    pub fn encode_prost(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.encode_to_vec())
    }

    pub fn decode_prost(schema_bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::decode(schema_bytes)?)
    }

    pub fn compile_boot_handle_bindings(&mut self) -> anyhow::Result<()> {
        for world_schema in self.client_authored_schema.worlds.values_mut() {
            compile_world_boot_handle_bindings(world_schema)?;
        }
        Ok(())
    }
}

fn compile_world_boot_handle_bindings(world_schema: &mut WorldSchema) -> anyhow::Result<()> {
    let named_object_indices = world_schema
        .objects
        .iter()
        .enumerate()
        .filter_map(|(object_index, world_object)| {
            let named_handle = AuthoredWorldObjectView::new(world_object).named_handle()?;
            Some((named_handle, object_index as u32))
        })
        .collect::<std::collections::HashMap<_, _>>();

    for state_machine in &mut world_schema.state_machines {
        compile_state_machine_boot_handle_bindings(state_machine, &named_object_indices)?;
    }
    for world_object in &mut world_schema.objects {
        for state_machine in &mut world_object.state_machines {
            compile_state_machine_boot_handle_bindings(state_machine, &named_object_indices)?;
        }
    }

    Ok(())
}

fn compile_state_machine_boot_handle_bindings(
    state_machine: &mut StateMachineSchema,
    named_object_indices: &std::collections::HashMap<String, u32>,
) -> anyhow::Result<()> {
    let named_bindings = std::mem::take(&mut state_machine.boot_named_handle_bindings);
    for binding in named_bindings {
        let compiled_binding = match binding.resolved_handle_kind() {
            StateMachineBootNamedHandleKindSchema::Node => {
                let Some(target_object_index) = named_object_indices
                    .get(binding.named_handle.as_str())
                    .copied()
                else {
                    anyhow::bail!(
                        "boot node handle '{}' does not resolve to a compiled world object",
                        binding.named_handle
                    );
                };
                StateMachineBootHandleBindingSchema::new_object(
                    binding.local_id,
                    binding.property_id,
                    StateMachineBootHandleKindSchema::Node,
                    target_object_index,
                )
            }
            StateMachineBootNamedHandleKindSchema::Camera => {
                let Some(target_object_index) = named_object_indices
                    .get(binding.named_handle.as_str())
                    .copied()
                else {
                    anyhow::bail!(
                        "boot camera handle '{}' does not resolve to a compiled world object",
                        binding.named_handle
                    );
                };
                StateMachineBootHandleBindingSchema::new_object(
                    binding.local_id,
                    binding.property_id,
                    StateMachineBootHandleKindSchema::Camera,
                    target_object_index,
                )
            }
            StateMachineBootNamedHandleKindSchema::Asset => {
                StateMachineBootHandleBindingSchema::new_asset(
                    binding.local_id,
                    binding.property_id,
                    parse_boot_asset_ref(binding.named_handle.as_str())?,
                )
            }
        };
        state_machine.register_boot_handle_binding(compiled_binding);
    }

    Ok(())
}

fn parse_boot_asset_ref(named_handle: &str) -> anyhow::Result<AssetRef> {
    let normalized = named_handle.trim();
    if normalized.is_empty() {
        anyhow::bail!("boot asset handle cannot be empty");
    }
    if let Some((bundle_id, asset_path)) = normalized.split_once(':') {
        let bundle_id = bundle_id.trim();
        let asset_path = asset_path.trim();
        if bundle_id.is_empty() || asset_path.is_empty() {
            anyhow::bail!(
                "boot asset handle '{}' is not a valid bundle:path reference",
                normalized
            );
        }
        return Ok(AssetRef::new_with_bundle_id(
            bundle_id,
            std::path::PathBuf::from(asset_path),
        ));
    }
    Ok(AssetRef::new(std::path::PathBuf::from(normalized)))
}

#[cfg(test)]
mod tests {
    use super::ExperienceSchema;
    use crate::experience_schema::client_authored::state_machines::state_machine_boot_handle_binding_schema::{
        StateMachineBootHandleBindingSchema, StateMachineBootHandleKindSchema,
    };
    use crate::experience_schema::client_authored::state_machines::state_machine_boot_named_handle_binding_schema::{
        StateMachineBootNamedHandleBindingSchema, StateMachineBootNamedHandleKindSchema,
    };
    use crate::experience_schema::client_authored::state_machines::api::StateMachineApiSchema;
    use crate::experience_schema::client_authored::state_machines::state_machine_node_schema::{
        StateMachineNodeSchema, StateMachineNodeTypeSchema,
    };
    use crate::experience_schema::client_authored::state_machines::state_machine_schema::StateMachineSchema;
    use crate::experience_schema::client_authored::worlds::world_object_schema::WorldObjectSchema;
    use crate::experience_schema::client_authored::worlds::world_schema::WorldSchema;
    use crate::properties::property_map::PropertyMap;

    #[test]
    fn prost_round_trip_preserves_custom_api_identifiers() {
        let mut schema = ExperienceSchema::default();
        schema
            .client_authored_schema
            .worlds
            .insert("".to_string(), Default::default());
        schema
            .client_authored_schema
            .worlds
            .get_mut("")
            .expect("world should exist")
            .state_machines
            .push({
                let mut state_machine = StateMachineSchema::new_with_seed("idle", 7);
                state_machine.nodes = vec![StateMachineNodeSchema::new(
                    "idle",
                    StateMachineNodeTypeSchema::ApiDispatch {
                        api: StateMachineApiSchema::from(
                            "custom:sample_story:dispatch_progression_complete",
                        ),
                        args_local_id: Some("args".to_string()),
                        emitted_local_event_names: Vec::new(),
                    },
                )];
                state_machine
            });

        let bytes = schema.encode_prost().expect("encode");
        let decoded = ExperienceSchema::decode_prost(&bytes).expect("decode");

        assert_eq!(
            decoded.client_authored_schema.worlds[""].state_machines[0].deterministic_seed,
            7
        );
        assert_eq!(
            decoded.client_authored_schema.worlds[""].state_machines[0].nodes[0].node_type,
            StateMachineNodeTypeSchema::ApiDispatch {
                api: StateMachineApiSchema::Custom(
                    "custom:sample_story:dispatch_progression_complete".to_string()
                ),
                args_local_id: Some("args".to_string()),
                emitted_local_event_names: Vec::new(),
            }
        );
    }

    #[test]
    fn compile_boot_handle_bindings_lowers_named_authoring_bindings_to_dense_references() {
        let mut schema = ExperienceSchema::default();
        let mut target_properties = PropertyMap::default();
        target_properties.insert_string("named_handle", "player");
        let mut camera_properties = PropertyMap::default();
        camera_properties.insert_string("named_handle", "camera:main");

        let mut state_machine = StateMachineSchema::new("idle");
        state_machine.register_boot_named_handle_binding(
            StateMachineBootNamedHandleBindingSchema::new(
                "node_args",
                "target_node",
                StateMachineBootNamedHandleKindSchema::Node,
                "player",
            ),
        );
        state_machine.register_boot_named_handle_binding(
            StateMachineBootNamedHandleBindingSchema::new(
                "camera_args",
                "target_camera",
                StateMachineBootNamedHandleKindSchema::Camera,
                "camera:main",
            ),
        );
        state_machine.register_boot_named_handle_binding(
            StateMachineBootNamedHandleBindingSchema::new(
                "asset_args",
                "target_asset",
                StateMachineBootNamedHandleKindSchema::Asset,
                "ui:icons/start.png",
            ),
        );

        schema.client_authored_schema.worlds.insert(
            "default".to_string(),
            WorldSchema {
                objects: vec![
                    WorldObjectSchema {
                        properties: target_properties,
                        ..Default::default()
                    },
                    WorldObjectSchema::default(),
                    WorldObjectSchema {
                        properties: camera_properties,
                        ..Default::default()
                    },
                ],
                state_machines: vec![state_machine],
                ..Default::default()
            },
        );

        schema
            .compile_boot_handle_bindings()
            .expect("named boot bindings should compile");

        let compiled_state_machine =
            &schema.client_authored_schema.worlds["default"].state_machines[0];
        assert!(compiled_state_machine.boot_named_handle_bindings.is_empty());
        assert_eq!(
            compiled_state_machine.boot_handle_bindings,
            vec![
                StateMachineBootHandleBindingSchema::new_object(
                    "node_args",
                    "target_node",
                    StateMachineBootHandleKindSchema::Node,
                    0,
                ),
                StateMachineBootHandleBindingSchema::new_object(
                    "camera_args",
                    "target_camera",
                    StateMachineBootHandleKindSchema::Camera,
                    2,
                ),
                StateMachineBootHandleBindingSchema::new_asset(
                    "asset_args",
                    "target_asset",
                    crate::assets::asset_ref::AssetRef::new_with_bundle_id(
                        "ui",
                        std::path::PathBuf::from("icons/start.png")
                    ),
                ),
            ]
        );
    }
}
