use prost::{Enumeration, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Enumeration)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum StateMachineBootNamedHandleKindSchema {
    Node = 0,
    Camera = 1,
    Asset = 2,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct StateMachineBootNamedHandleBindingSchema {
    #[prost(string, tag = "1")]
    pub local_id: String,
    #[prost(string, tag = "2")]
    pub property_id: String,
    #[prost(enumeration = "StateMachineBootNamedHandleKindSchema", tag = "3")]
    pub handle_kind: i32,
    #[prost(string, tag = "4")]
    pub named_handle: String,
}

impl StateMachineBootNamedHandleBindingSchema {
    pub fn new(
        local_id: impl Into<String>,
        property_id: impl Into<String>,
        handle_kind: StateMachineBootNamedHandleKindSchema,
        named_handle: impl Into<String>,
    ) -> Self {
        Self {
            local_id: local_id.into().trim().to_string(),
            property_id: property_id.into().trim().to_string(),
            handle_kind: handle_kind as i32,
            named_handle: named_handle.into().trim().to_string(),
        }
    }

    pub fn resolved_handle_kind(&self) -> StateMachineBootNamedHandleKindSchema {
        StateMachineBootNamedHandleKindSchema::try_from(self.handle_kind)
            .unwrap_or(StateMachineBootNamedHandleKindSchema::Node)
    }
}

#[cfg(test)]
mod tests {
    use super::{StateMachineBootNamedHandleBindingSchema, StateMachineBootNamedHandleKindSchema};
    use prost::Message;

    #[test]
    fn constructor_trims_fields_and_preserves_kind() {
        let binding = StateMachineBootNamedHandleBindingSchema::new(
            " runtime_values ",
            " target_handle_bytes ",
            StateMachineBootNamedHandleKindSchema::Camera,
            " camera:primary ",
        );

        assert_eq!(binding.local_id, "runtime_values");
        assert_eq!(binding.property_id, "target_handle_bytes");
        assert_eq!(
            binding.resolved_handle_kind(),
            StateMachineBootNamedHandleKindSchema::Camera
        );
        assert_eq!(binding.named_handle, "camera:primary");
    }

    #[test]
    fn prost_round_trip_preserves_binding() {
        let binding = StateMachineBootNamedHandleBindingSchema::new(
            "runtime_values",
            "target_handle_bytes",
            StateMachineBootNamedHandleKindSchema::Node,
            "ui:panel",
        );

        let encoded = binding.encode_to_vec();
        let decoded = StateMachineBootNamedHandleBindingSchema::decode(encoded.as_slice())
            .expect("binding should decode");

        assert_eq!(decoded, binding);
    }
}
